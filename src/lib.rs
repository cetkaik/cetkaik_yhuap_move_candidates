#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::too_many_lines,
    clippy::non_ascii_literal,
    clippy::module_name_repetitions,
    clippy::use_self
)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

/// Spits out all the possible opponent (downward)'s move that is played from the hop1zuo1 onto the board.
#[must_use]
pub fn from_hop1zuo1_candidates(game_state: &PureGameState) -> Vec<PureMove> {
    let mut ans = vec![];
    for piece in &game_state.f.hop1zuo1of_downward {
        for empty_square in empty_squares(game_state) {
            ans.push(PureMove::NonTamMoveFromHopZuo {
                color: piece.color,
                prof: piece.prof,
                dest: to_absolute_coord(empty_square, game_state.perspective),
            });
        }
    }
    ans
}

pub use pure_move::PureMove;

mod calculate_movable;
pub use calculate_movable::calculate_movable_positions_for_either_side;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct MovablePositions {
    pub finite: Vec<Coord>,
    pub infinite: Vec<Coord>,
}

fn can_get_occupied_by(
    side: Side,
    dest: Coord,
    piece_to_move: Piece,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> bool {
    if piece_to_move == Piece::Tam2 {
        let [i, j] = dest;
        let dest_piece = board[i][j];
        /* It is allowed to enter an empty square */
        dest_piece == None
    } else {
        can_get_occupied_by_non_tam(side, dest, board, tam_itself_is_tam_hue)
    }
}

fn empty_neighbors_of(board: Board, c: Coord) -> Vec<Coord> {
    calculate_movable::eight_neighborhood(c)
        .iter()
        .filter(|[i, j]| board[*i][*j].is_none())
        .copied()
        .collect()
}

fn can_get_occupied_by_non_tam(
    side: Side,
    dest: Coord,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> bool {
    /* Intentionally does not verify whether the piece itself is of opponent */
    let is_protected_by_opponent_tam_hue_auai = |side: Side, coord: Coord| {
        calculate_movable::eight_neighborhood(coord)
            .into_iter()
            .filter(|[a, b]| {
                let piece = board[*a][*b];
                match piece {
                    None | Some(Piece::Tam2) => false,
                    Some(Piece::NonTam2Piece {
                        side: piece_side,
                        prof: piece_prof,
                        color: _,
                    }) => {
                        piece_prof == Profession::Uai1
                            && piece_side != side
                            && calculate_movable::is_tam_hue([*a, *b], board, tam_itself_is_tam_hue)
                    }
                }
            })
            .count()
            > 0
    };

    let [i, j] = dest;
    let dest_piece = board[i][j];

    match dest_piece {
        Some(Piece::Tam2) => false, /* Tam2 can never be taken */

        None => true, /* It is always allowed to enter an empty square */
        Some(Piece::NonTam2Piece {
            side: piece_side,
            prof: _,
            color: _,
        }) => {
            piece_side != side /* cannot take your own piece */ &&
            !is_protected_by_opponent_tam_hue_auai(
                side,
                dest
            )
        } /* must not be protected by tam2 hue a uai1 */
    }
}

const fn is_ciurl_required(dest: Coord, moving_piece_prof: Profession, src: Coord) -> bool {
    is_water(dest) && !is_water(src) && !matches!(moving_piece_prof, Profession::Nuak1)
}

fn generate_candidates_when_stepping(
    game_state: &PureGameState,
    src: Coord,
    step: Coord,
    rotated_coord: Coord,
    rotated_piece: NonTam2PieceUpward,
) -> Vec<PureMove> {
    let perspective = game_state.perspective;
    let tam_itself_is_tam_hue: bool = game_state.tam_itself_is_tam_hue;
    /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
    let mut subtracted_rotated_board = rotate_board(game_state.f.current_board);
    subtracted_rotated_board[rotated_coord[0]][rotated_coord[1]] = None; /* must remove the piece to prevent self-occlusion */

    let MovablePositions { finite, infinite } = calculate_movable::calculate_movable_positions(
        rotate_coord(step),
        rotated_piece.into(),
        subtracted_rotated_board,
        tam_itself_is_tam_hue,
    );

    let candidates: Vec<Coord> = finite.iter().map(|c| rotate_coord(*c)).collect::<Vec<_>>();
    let candidates_inf: Vec<Coord> = infinite.iter().map(|c| rotate_coord(*c)).collect();
    [
        &candidates
            .iter()
            .flat_map(|final_dest| {
                let NonTam2PieceUpward {
                    color: rotated_piece_color,
                    prof: rotated_piece_prof,
                } = rotated_piece;
                if can_get_occupied_by(
                    Side::Downward,
                    *final_dest,
                    Piece::NonTam2Piece {
                        color: rotated_piece_color,
                        prof: rotated_piece_prof,
                        side: Side::Downward,
                    },
                    rotate_board(subtracted_rotated_board),
                    tam_itself_is_tam_hue,
                ) {
                    vec![PureMove::NonTamMoveSrcStepDstFinite {
                        src: to_absolute_coord(src, perspective),
                        step: to_absolute_coord(step, perspective),
                        dest: to_absolute_coord(*final_dest, perspective),
                        is_water_entry_ciurl: is_ciurl_required(
                            *final_dest,
                            rotated_piece_prof,
                            src,
                        ),
                    }]
                    .into_iter()
                } else {
                    vec![].into_iter()
                }
            })
            .collect::<Vec<PureMove>>()[..],
        &candidates_inf
            .iter()
            .flat_map(|planned_dest| {
                let NonTam2PieceUpward {
                    color: rotated_piece_color,
                    prof: rotated_piece_prof,
                } = rotated_piece;
                if !can_get_occupied_by(
                    Side::Downward,
                    *planned_dest,
                    Piece::NonTam2Piece {
                        color: rotated_piece_color,
                        prof: rotated_piece_prof,
                        side: Side::Downward,
                    },
                    rotate_board(subtracted_rotated_board),
                    tam_itself_is_tam_hue,
                ) {
                    return vec![].into_iter();
                    // retry
                }
                let obj: PureMove = PureMove::InfAfterStep {
                    src: to_absolute_coord(src, perspective),
                    step: to_absolute_coord(step, perspective),
                    planned_direction: to_absolute_coord(*planned_dest, perspective),
                };
                vec![obj].into_iter()
            })
            .collect::<Vec<PureMove>>()[..],
    ]
    .concat()
}

/// Spits out all the possible opponent (downward)'s move that is played by moving a piece on the board, not from the hop1zuo1.
#[must_use]
pub fn not_from_hop1zuo1_candidates_(config: &Config, game_state: &PureGameState) -> Vec<PureMove> {
    let mut ans = vec![];
    for Rotated {
        rotated_piece,
        rotated_coord,
    } in get_opponent_pieces_and_tam_rotated(game_state)
    {
        let MovablePositions { finite, infinite } = calculate_movable::calculate_movable_positions(
            rotated_coord,
            rotated_piece,
            rotate_board(game_state.f.current_board),
            game_state.tam_itself_is_tam_hue,
        );

        let candidates: Vec<Coord> = [
            &finite.into_iter().map(rotate_coord).collect::<Vec<_>>()[..],
            &infinite.into_iter().map(rotate_coord).collect::<Vec<_>>()[..],
        ]
        .concat();

        let src: Coord = rotate_coord(rotated_coord);

        for tentative_dest in candidates {
            let dest_piece = game_state.f.current_board[tentative_dest[0]][tentative_dest[1]];

            match rotated_piece {
                TamOrUpwardPiece::Tam2 => {
                    /* avoid self-occlusion */
                    let mut subtracted_rotated_board = rotate_board(game_state.f.current_board);
                    subtracted_rotated_board[rotated_coord[0]][rotated_coord[1]] = None;
                    // FIXME: tam2 ty sak2 not handled
                    if dest_piece == None {
                        /* empty square; first move is completed without stepping */
                        let fst_dst: Coord = tentative_dest;
                        ans.append(&mut calculate_movable::eight_neighborhood(fst_dst).iter().flat_map(|neighbor| {
                            /* if the neighbor is empty, that is the second destination */
                            let snd_dst: Coord = *neighbor;
                            if game_state.f.current_board[neighbor[0]][neighbor[1]] ==
                                None /* the neighbor is utterly occupied */ ||
                                *neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                vec![PureMove::TamMoveNoStep {
                                    second_dest: to_absolute_coord(snd_dst, game_state.perspective),
                                    first_dest: to_absolute_coord(fst_dst, game_state.perspective),
                                    src: to_absolute_coord(src, game_state.perspective),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: Coord = *neighbor;
                                empty_neighbors_of(rotate_board(subtracted_rotated_board), step)
                                    .iter().flat_map(|snd_dst| {
                                    vec![PureMove::TamMoveStepsDuringLatter {
                                        first_dest: to_absolute_coord(fst_dst, game_state.perspective),
                                        second_dest: to_absolute_coord(*snd_dst, game_state.perspective),
                                        src: to_absolute_coord(src, game_state.perspective),
                                        step: to_absolute_coord(step, game_state.perspective),
                                    }].into_iter()
                                }).collect::<Vec<PureMove>>().into_iter()
                            }
                        }).collect::<Vec<PureMove>>());
                    } else {
                        /* not an empty square: must complete the first move */
                        let step = tentative_dest;
                        ans.append(
                            &mut empty_neighbors_of(rotate_board(subtracted_rotated_board), step)
                                .iter()
                                .flat_map(|fst_dst| {
                                    let v = empty_neighbors_of(
                                        rotate_board(subtracted_rotated_board),
                                        *fst_dst,
                                    );
                                    v.iter()
                                        .flat_map(move |snd_dst| {
                                            vec![PureMove::TamMoveStepsDuringFormer {
                                                first_dest: to_absolute_coord(
                                                    *fst_dst,
                                                    game_state.perspective,
                                                ),
                                                second_dest: to_absolute_coord(
                                                    *snd_dst,
                                                    game_state.perspective,
                                                ),
                                                src: to_absolute_coord(src, game_state.perspective),
                                                step: to_absolute_coord(
                                                    step,
                                                    game_state.perspective,
                                                ),
                                            }]
                                            .into_iter()
                                        })
                                        .collect::<Vec<PureMove>>()
                                        .into_iter()
                                })
                                .collect::<Vec<PureMove>>(),
                        );
                    }
                }
                TamOrUpwardPiece::NonTam2Piece {
                    color: rotated_piece_color,
                    prof: rotated_piece_prof,
                } => {
                    let candidates_when_stepping = || {
                        generate_candidates_when_stepping(
                            game_state,
                            src,
                            tentative_dest, // tentative_dest becomes the position on which the stepping occurs
                            rotated_coord,
                            NonTam2PieceUpward {
                                color: rotated_piece_color,
                                prof: rotated_piece_prof,
                            },
                        )
                    };
                    match dest_piece {
                        None => {
                            // cannot step
                            ans.append(&mut vec![PureMove::NonTamMoveSrcDst {
                                src: to_absolute_coord(src, game_state.perspective),
                                dest: to_absolute_coord(tentative_dest, game_state.perspective),
                                is_water_entry_ciurl: is_ciurl_required(
                                    tentative_dest,
                                    rotated_piece_prof,
                                    src,
                                ),
                            }]);
                        }
                        Some(Piece::Tam2) => {
                            // if allowed by config, allow stepping on Tam2;
                            if config.allow_kut2tam2 {
                                ans.append(&mut candidates_when_stepping());
                            } else {
                                ans.append(&mut vec![]);
                            }
                        }
                        Some(Piece::NonTam2Piece {
                            side: Side::Upward,
                            color: _,
                            prof: _,
                        }) => {
                            // opponent's piece; stepping and taking both attainable

                            // except when protected by tam2 hue a uai1
                            if can_get_occupied_by(
                                Side::Downward,
                                tentative_dest,
                                Piece::NonTam2Piece {
                                    color: rotated_piece_color,
                                    prof: rotated_piece_prof,
                                    side: Side::Downward,
                                },
                                game_state.f.current_board,
                                game_state.tam_itself_is_tam_hue,
                            ) {
                                ans.append(
                                    &mut [
                                        &[PureMove::NonTamMoveSrcDst {
                                            src: to_absolute_coord(src, game_state.perspective),
                                            dest: to_absolute_coord(
                                                tentative_dest,
                                                game_state.perspective,
                                            ),
                                            is_water_entry_ciurl: is_ciurl_required(
                                                tentative_dest,
                                                rotated_piece_prof,
                                                src,
                                            ),
                                        }][..],
                                        &candidates_when_stepping()[..],
                                    ]
                                    .concat(),
                                );
                            } else {
                                ans.append(&mut candidates_when_stepping());
                            }
                        }
                        Some(_) => {
                            ans.append(&mut candidates_when_stepping());
                        }
                    }
                }
            }
        }
    }

    ans
}

fn get_opponent_pieces_and_tam_rotated(game_state: &PureGameState) -> Vec<Rotated> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let coord = [rand_i, rand_j];
            let piece = game_state.f.current_board[rand_i][rand_j];
            if let Some(p) = piece {
                match p {
                    Piece::Tam2 => ans.push(Rotated {
                        rotated_piece: TamOrUpwardPiece::Tam2,
                        rotated_coord: rotate_coord(coord),
                    }),
                    Piece::NonTam2Piece {
                        side: Side::Downward,
                        prof,
                        color,
                    } => {
                        let rot_piece = NonTam2PieceUpward { color, prof };
                        ans.push(Rotated {
                            rotated_piece: rot_piece.into(),
                            rotated_coord: rotate_coord(coord),
                        });
                    }
                    Piece::NonTam2Piece {
                        side: Side::Upward, ..
                    } => {}
                }
            }
        }
    }
    ans
}

fn empty_squares(game_state: &PureGameState) -> Vec<Coord> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let coord: Coord = [rand_i, rand_j];
            if game_state.f.current_board[rand_i][rand_j] == None {
                ans.push(coord);
            }
        }
    }
    ans
}

use cetkaik_core::relative::{
    is_water, rotate_board, rotate_coord, Board, Coord, Field, NonTam2PieceUpward, Piece, Side,
};

pub use cetkaik_core::absolute;

struct Rotated {
    rotated_piece: TamOrUpwardPiece,
    rotated_coord: Coord,
}

pub mod pure_move;

pub struct Config {
    pub allow_kut2tam2: bool,
}

#[cfg(test)]
mod tests;

use calculate_movable::TamOrUpwardPiece;

pub use cetkaik_core::perspective::*;
pub use cetkaik_core::{Color, Profession};

#[derive(Debug)]
pub struct PureGameState {
    pub f: Field,
    pub perspective: Perspective,
    pub tam_itself_is_tam_hue: bool,
    pub opponent_has_just_moved_tam: bool,
}

/// According to <https://github.com/cetkaik/cetkaik_yhuap_move_candidates/pull/7>,
/// this function was a bottleneck that accounted for roughly fifty percent of all the runtime
/// in an attempt to implement an AI for this game.
/// Hence, this module contains multiple implementations of this functions and test that they are equal.
#[allow(clippy::similar_names)]
pub mod get_blocker_deltas;
