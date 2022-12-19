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

pub struct CetkaikCore;

pub struct CetkaikCompact;

pub trait CetkaikRepresentation {
    type AbsoluteCoord;
    type RelativeCoord: Copy;
    type Perspective;
    type RelativeBoard: Copy;
    type RelativePiece: Eq;
    type RelativeSide: Copy;
    fn to_absolute_coord(coord: Self::RelativeCoord, p: Self::Perspective) -> Self::AbsoluteCoord;
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord>;
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece>;
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool;
    fn tam2() -> Self::RelativePiece;
    fn is_upward(s: Self::RelativeSide) -> bool;
}

/// `cetkaik_core` クレートに基づいており、視点に依らない絶対座標での表現と、視点に依る相対座標への表現を正しく相互変換できる。
impl CetkaikRepresentation for CetkaikCore {
    type AbsoluteCoord = cetkaik_core::absolute::Coord;
    type RelativeCoord = cetkaik_core::relative::Coord;
    type Perspective = crate::Perspective;
    type RelativeBoard = cetkaik_core::relative::Board;
    type RelativePiece = cetkaik_core::relative::Piece;
    type RelativeSide = cetkaik_core::relative::Side;
    fn to_absolute_coord(coord: Self::RelativeCoord, p: Self::Perspective) -> Self::AbsoluteCoord {
        cetkaik_core::perspective::to_absolute_coord(coord, p)
    }
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord> {
        let [i, j] = coord;
        match (
            i.checked_add_signed(row_delta),
            j.checked_add_signed(col_delta),
        ) {
            (Some(l @ 0..=8), Some(m @ 0..=8)) => Some([l, m]),
            _ => None,
        }
    }
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece> {
        let [i, j] = coord;
        board[i][j]
    }
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool {
        coord == [2, 2]
            || coord == [2, 6]
            || coord == [3, 3]
            || coord == [3, 5]
            || coord == [4, 4]
            || coord == [5, 3]
            || coord == [5, 5]
            || coord == [6, 2]
            || coord == [6, 6]
    }
    fn tam2() -> Self::RelativePiece {
        cetkaik_core::relative::Piece::Tam2
    }
    fn is_upward(s: Self::RelativeSide) -> bool {
        s == cetkaik_core::relative::Side::Upward
    }
}

/// `cetkaik_compact_representation` クレートに基づいており、視点を決め打ちして絶対座標=相対座標として表現する。
/// この impl においては、IAは常に一番下の行であり、初期状態でIA行を占有していたプレイヤーは駒が上向き（=あなた）である。
/// つまり、`Upward` は常に `IASide` へと読み替えられる。 
impl CetkaikRepresentation for CetkaikCompact {
    type AbsoluteCoord = cetkaik_compact_representation::Coord;
    type RelativeCoord = cetkaik_compact_representation::Coord;
    type Perspective = cetkaik_compact_representation::Perspective;
    type RelativeBoard = cetkaik_compact_representation::Board;
    type RelativePiece = cetkaik_compact_representation::PieceWithSide;
    type RelativeSide = cetkaik_core::absolute::Side; // ここも absolute
    fn to_absolute_coord(coord: Self::RelativeCoord, _p: Self::Perspective) -> Self::AbsoluteCoord {
        coord
    }
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord> {
        cetkaik_compact_representation::Coord::add_delta(coord, row_delta, col_delta)
    }
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece> {
        board.peek(coord)
    }
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool {
        Self::RelativeCoord::is_tam_hue_by_default(coord)
    }
    fn tam2() -> Self::RelativePiece {
        unsafe { cetkaik_compact_representation::PieceWithSide::new_unchecked(0o300) }
    }
    fn is_upward(s: Self::RelativeSide) -> bool {
        s == cetkaik_core::absolute::Side::IASide
    }
}

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
pub struct MovablePositions<T> {
    pub finite: Vec<T>,
    pub infinite: Vec<T>,
}

fn can_get_occupied_by(
    side: Side,
    dest: Coord,
    piece_to_move: Piece,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> bool {
    if piece_to_move == Piece::Tam2 {
        /* It is allowed to enter an empty square */
        CetkaikCore::relative_get(board, dest).is_none()
    } else {
        can_get_occupied_by_non_tam(side, dest, board, tam_itself_is_tam_hue)
    }
}

fn empty_neighbors_of<T: CetkaikRepresentation>(
    board: T::RelativeBoard,
    c: T::RelativeCoord,
) -> impl Iterator<Item = T::RelativeCoord> {
    calculate_movable::iter::eight_neighborhood::<T>(c)
        .filter(move |coord| T::relative_get(board, *coord).is_none())
}

fn can_get_occupied_by_non_tam(
    side: Side,
    dest: Coord,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> bool {
    /* Intentionally does not verify whether the piece itself is of opponent */
    let is_protected_by_opponent_tam_hue_auai = |side: Side, coord: Coord| {
        calculate_movable::vec::eight_neighborhood::<CetkaikCore>(coord)
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
                            && calculate_movable::is_tam_hue::<CetkaikCore>(
                                [*a, *b],
                                board,
                                tam_itself_is_tam_hue,
                            )
                    }
                }
            })
            .count()
            > 0
    };

    let dest_piece = CetkaikCore::relative_get(board, dest);

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

/// Spits out all the possible opponent (downward)'s move that is played by moving a piece on the board, not from the hop1zuo1.
#[must_use]
pub fn not_from_hop1zuo1_candidates_(config: &Config, game_state: &PureGameState) -> Vec<PureMove> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let src = [rand_i, rand_j];
            let piece = game_state.f.current_board[rand_i][rand_j];
            if let Some(p) = piece {
                match p {
                    Piece::Tam2 => {
                        let candidates: Vec<Coord> =
                            calculate_movable::vec::eight_neighborhood::<CetkaikCore>(src);
                        for tentative_dest in candidates {
                            let dest_piece =
                                game_state.f.current_board[tentative_dest[0]][tentative_dest[1]];

                            /* avoid self-occlusion */
                            let mut subtracted_board = game_state.f.current_board;
                            subtracted_board[src[0]][src[1]] = None;
                            // FIXME: tam2 ty sak2 not handled
                            if dest_piece.is_none() {
                                /* empty square; first move is completed without stepping */
                                let fst_dst: Coord = tentative_dest;
                                ans.append(&mut calculate_movable::iter::eight_neighborhood::<CetkaikCore>(fst_dst).flat_map(|neighbor| {
                            /* if the neighbor is empty, that is the second destination */
                            let snd_dst: Coord = neighbor;
                            if game_state.f.current_board[neighbor[0]][neighbor[1]].is_none() /* the neighbor is utterly occupied */ ||
                                neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                vec![PureMove::TamMoveNoStep {
                                    second_dest: to_absolute_coord(snd_dst, game_state.perspective),
                                    first_dest: to_absolute_coord(fst_dst, game_state.perspective),
                                    src: to_absolute_coord(src, game_state.perspective),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: Coord = neighbor;
                                empty_neighbors_of::<CetkaikCore>(subtracted_board, step)
                                    .flat_map(|snd_dst| {
                                    vec![PureMove::TamMoveStepsDuringLatter {
                                        first_dest: to_absolute_coord(fst_dst, game_state.perspective),
                                        second_dest: to_absolute_coord(snd_dst, game_state.perspective),
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
                                    &mut empty_neighbors_of::<CetkaikCore>(subtracted_board, step)
                                        .flat_map(|fst_dst| {
                                            let v = empty_neighbors_of::<CetkaikCore>(
                                                subtracted_board,
                                                fst_dst,
                                            );
                                            v.flat_map(move |snd_dst| {
                                                vec![PureMove::TamMoveStepsDuringFormer {
                                                    first_dest: to_absolute_coord(
                                                        fst_dst,
                                                        game_state.perspective,
                                                    ),
                                                    second_dest: to_absolute_coord(
                                                        snd_dst,
                                                        game_state.perspective,
                                                    ),
                                                    src: to_absolute_coord(
                                                        src,
                                                        game_state.perspective,
                                                    ),
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
                    }
                    Piece::NonTam2Piece {
                        side: Side::Downward,
                        prof,
                        color,
                    } => {
                        let MovablePositions { finite, infinite } =
                            calculate_movable::calculate_movable_positions_for_nontam::<CetkaikCore>(
                                src,
                                prof,
                                game_state.f.current_board,
                                game_state.tam_itself_is_tam_hue,
                                Side::Downward,
                            );

                        let candidates: Vec<Coord> = [&finite[..], &infinite[..]].concat();
                        for tentative_dest in candidates {
                            let dest_piece =
                                game_state.f.current_board[tentative_dest[0]][tentative_dest[1]];

                            let candidates_when_stepping = || {
                                let step = tentative_dest; // tentative_dest becomes the position on which the stepping occurs

                                let perspective = game_state.perspective;
                                let tam_itself_is_tam_hue: bool = game_state.tam_itself_is_tam_hue;
                                /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
                                let mut subtracted_board = game_state.f.current_board;
                                subtracted_board[src[0]][src[1]] = None; /* must remove the piece to prevent self-occlusion */

                                let MovablePositions { finite, infinite } =
                                    calculate_movable::calculate_movable_positions_for_nontam::<CetkaikCore>(
                                        step,
                                        prof,
                                        subtracted_board,
                                        tam_itself_is_tam_hue,
                                        Side::Downward,
                                    );

                                let candidates = finite.into_iter();
                                let candidates_inf = infinite.into_iter();
                                [
                                    &candidates
                                        .flat_map(|final_dest| {
                                            if can_get_occupied_by(
                                                Side::Downward,
                                                final_dest,
                                                Piece::NonTam2Piece {
                                                    color,
                                                    prof,
                                                    side: Side::Downward,
                                                },
                                                subtracted_board,
                                                tam_itself_is_tam_hue,
                                            ) {
                                                vec![PureMove::NonTamMoveSrcStepDstFinite {
                                                    src: to_absolute_coord(src, perspective),
                                                    step: to_absolute_coord(step, perspective),
                                                    dest: to_absolute_coord(
                                                        final_dest,
                                                        perspective,
                                                    ),
                                                    is_water_entry_ciurl: is_ciurl_required(
                                                        final_dest, prof, src,
                                                    ),
                                                }]
                                                .into_iter()
                                            } else {
                                                vec![].into_iter()
                                            }
                                        })
                                        .collect::<Vec<PureMove>>()[..],
                                    &candidates_inf
                                        .flat_map(|planned_dest| {
                                            if !can_get_occupied_by(
                                                Side::Downward,
                                                planned_dest,
                                                Piece::NonTam2Piece {
                                                    color,
                                                    prof,
                                                    side: Side::Downward,
                                                },
                                                subtracted_board,
                                                tam_itself_is_tam_hue,
                                            ) {
                                                return vec![].into_iter();
                                                // retry
                                            }
                                            let obj: PureMove = PureMove::InfAfterStep {
                                                src: to_absolute_coord(src, perspective),
                                                step: to_absolute_coord(step, perspective),
                                                planned_direction: to_absolute_coord(
                                                    planned_dest,
                                                    perspective,
                                                ),
                                            };
                                            vec![obj].into_iter()
                                        })
                                        .collect::<Vec<PureMove>>()[..],
                                ]
                                .concat()
                            };
                            match dest_piece {
                                None => {
                                    // cannot step
                                    ans.append(&mut vec![PureMove::NonTamMoveSrcDst {
                                        src: to_absolute_coord(src, game_state.perspective),
                                        dest: to_absolute_coord(
                                            tentative_dest,
                                            game_state.perspective,
                                        ),
                                        is_water_entry_ciurl: is_ciurl_required(
                                            tentative_dest,
                                            prof,
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
                                            color,
                                            prof,
                                            side: Side::Downward,
                                        },
                                        game_state.f.current_board,
                                        game_state.tam_itself_is_tam_hue,
                                    ) {
                                        ans.append(
                                            &mut [
                                                &[PureMove::NonTamMoveSrcDst {
                                                    src: to_absolute_coord(
                                                        src,
                                                        game_state.perspective,
                                                    ),
                                                    dest: to_absolute_coord(
                                                        tentative_dest,
                                                        game_state.perspective,
                                                    ),
                                                    is_water_entry_ciurl: is_ciurl_required(
                                                        tentative_dest,
                                                        prof,
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
            if game_state.f.current_board[rand_i][rand_j].is_none() {
                ans.push(coord);
            }
        }
    }
    ans
}

use cetkaik_core::relative::{is_water, Board, Coord, Field, Piece, Side};

pub use cetkaik_core::absolute;

pub mod pure_move;

pub struct Config {
    pub allow_kut2tam2: bool,
}

#[cfg(test)]
mod tests;

pub use cetkaik_core::perspective::{to_absolute_coord, Perspective};
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
