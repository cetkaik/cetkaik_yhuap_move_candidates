fn from_hand_candidates(game_state: &PureGameState) -> Vec<PureOpponentMove> {
    let mut ans = vec![];
    for piece in &game_state.f.hop1zuo1of_downward {
        for empty_square in empty_squares(&game_state) {
            ans.push(PureOpponentMove::NonTamMoveFromHand {
                color: piece.color,
                prof: piece.prof,
                dest: to_absolute_coord(empty_square, game_state.ia_is_down),
            })
        }
    }
    ans
}

mod calculate_movable;

fn is_water([row, col]: Coord) -> bool {
     (row == 4 && col == 2)
        || (row == 4 && col == 3)
        || (row == 4 && col == 4)
        || (row == 4 && col == 5)
        || (row == 4 && col == 6)
        || (row == 2 && col == 4)
        || (row == 3 && col == 4)
        || (row == 5 && col == 4)
        || (row == 6 && col == 4)
}

fn to_absolute_coord(coord: Coord, ia_is_down: bool) -> AbsoluteCoord {
    let [row, col] = coord;

    let columns = vec![
        AbsoluteColumn::K,
        AbsoluteColumn::L,
        AbsoluteColumn::N,
        AbsoluteColumn::T,
        AbsoluteColumn::Z,
        AbsoluteColumn::X,
        AbsoluteColumn::C,
        AbsoluteColumn::M,
        AbsoluteColumn::P,
    ];

    let rows = vec![
        AbsoluteRow::A,
        AbsoluteRow::E,
        AbsoluteRow::I,
        AbsoluteRow::U,
        AbsoluteRow::O,
        AbsoluteRow::Y,
        AbsoluteRow::AI,
        AbsoluteRow::AU,
        AbsoluteRow::IA,
    ];

    (
        rows[if ia_is_down { row } else { 8 - row }],
        columns[if ia_is_down { col } else { 8 - col }],
    )
}

pub struct MovablePositions {
    finite: Vec<Coord>,
    infinite: Vec<Coord>,
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
        .filter(|a| {
            let [i, j] = a;
            board[*i][*j] == None
        })
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
                    None => false,
                    Some(Piece::Tam2) => false,
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

fn not_from_hand_candidates_(config: Config, game_state: &PureGameState) -> Vec<PureOpponentMove> {
    let mut ans = vec![];
    for Rotated {
        rotated_piece,
        rotated_coord,
    } in get_opponent_pieces_rotated(&game_state)
    {
        let MovablePositions {
            finite: guide_list_yellow,
            infinite: guide_list_green,
        } = calculate_movable::calculate_movable_positions(
            rotated_coord,
            rotated_piece,
            rotate_board(game_state.f.current_board),
            game_state.tam_itself_is_tam_hue,
        );

        let candidates: Vec<Coord> = [
            &guide_list_yellow
                .into_iter()
                .map(rotate_coord)
                .collect::<Vec<_>>()[..],
            &guide_list_green
                .into_iter()
                .map(rotate_coord)
                .collect::<Vec<_>>()[..],
        ]
        .concat();

        let src: Coord = rotate_coord(rotated_coord);

        for dest in candidates {
            fn is_ciurl_required(dest: Coord, moving_piece_prof: Profession, src: Coord) -> bool {
                is_water(dest) && !is_water(src) && moving_piece_prof != Profession::Nuak1
            }
            let dest_piece = game_state.f.current_board[dest[0]][dest[1]];

            let candidates_when_stepping = |rotated_piece| -> Vec<PureOpponentMove> {
                let step = dest; // less confusing

                /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
                let mut subtracted_rotated_board = rotate_board(game_state.f.current_board);
                subtracted_rotated_board[rotated_coord[0]][rotated_coord[1]] = None; /* must remove the piece to prevent self-occlusion */

                let MovablePositions {
                    finite: guide_list_yellow,
                    infinite: guide_list_green,
                } = calculate_movable::calculate_movable_positions(
                    rotate_coord(step),
                    rotated_piece,
                    subtracted_rotated_board,
                    game_state.tam_itself_is_tam_hue,
                );

                let candidates: Vec<Coord> = guide_list_yellow
                    .iter()
                    .map(|c| rotate_coord(*c))
                    .collect::<Vec<_>>();
                let candidates_inf: Vec<Coord> =
                    guide_list_green.iter().map(|c| rotate_coord(*c)).collect();
                 [
                  &candidates.iter().flat_map(|final_dest| {
                      let (rotated_piece_color , rotated_piece_prof) = match rotated_piece {
                          Piece::Tam2 => panic!(),
                          Piece::NonTam2Piece{color, prof, side: _} => (color, prof)
                      };
                     if can_get_occupied_by(
                        Side::Downward,
                        *final_dest,
                        Piece::NonTam2Piece {
                            color: rotated_piece_color,
                            prof: rotated_piece_prof,
                            side: Side::Downward
                        },
                        rotate_board(subtracted_rotated_board),
                        game_state.tam_itself_is_tam_hue
                    )
                    {
                        vec![PureOpponentMove::PotentialWaterEntry(PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcStepDstFinite {
                            src: to_absolute_coord(src, game_state.ia_is_down),
                            step: to_absolute_coord(step, game_state.ia_is_down),
                            dest: to_absolute_coord(*final_dest, game_state.ia_is_down),
                            is_water_entry_ciurl: is_ciurl_required(
                                *final_dest,
                                rotated_piece_prof,
                                src
                            )
                        })].into_iter()
                    } else { vec![].into_iter() }
                  }).collect::<Vec<PureOpponentMove>>()[..],
                  &candidates_inf.iter().flat_map(|planned_dest| {
                    let (rotated_piece_color , rotated_piece_prof) = match rotated_piece {
                        Piece::Tam2 => panic!(),
                        Piece::NonTam2Piece{color, prof, side: _} => (color, prof)
                    };
                    if
                      !can_get_occupied_by(
                          Side::Downward,
                          *planned_dest,
                          Piece::NonTam2Piece{
                          color: rotated_piece_color,
                          prof: rotated_piece_prof,
                          side: Side::Downward
                        },
                          rotate_board(subtracted_rotated_board),
                          game_state.tam_itself_is_tam_hue
                      )
                     {
                      return vec![].into_iter();
                      // retry
                    }
                    let obj: PureOpponentMove = PureOpponentMove::InfAfterStep{
                      src: to_absolute_coord(src, game_state.ia_is_down),
                      step: to_absolute_coord(step, game_state.ia_is_down),
                      planned_direction: to_absolute_coord(
                          *planned_dest,
                          game_state.ia_is_down
                      ),
                    };
                    vec![obj].into_iter()
                  }).collect::<Vec<PureOpponentMove>>()[..]
                ].concat()
            };

            match rotated_piece {
                Piece::Tam2 => {
                    /* avoid self-occlusion */
                    let mut subtracted_rotated_board = rotate_board(game_state.f.current_board);
                    subtracted_rotated_board[rotated_coord[0]][rotated_coord[1]] = None;
                    // FIXME: tam2 ty sak2 not handled
                    if dest_piece == None {
                        /* empty square; first move is completed without stepping */
                        let fst_dst: Coord = dest;
                        ans.append(&mut calculate_movable::eight_neighborhood(fst_dst).iter().flat_map(|neighbor| {
                            /* if the neighbor is empty, that is the second destination */
                            if game_state.f.current_board[neighbor[0]][neighbor[1]] ==
                                None /* the neighbor is utterly occupied */ ||
                                *neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                let snd_dst: Coord = *neighbor;
                                vec![PureOpponentMove::TamMoveNoStep {
                                    second_dest: to_absolute_coord(snd_dst, game_state.ia_is_down),
                                    first_dest: to_absolute_coord(fst_dst, game_state.ia_is_down),
                                    src: to_absolute_coord(src, game_state.ia_is_down),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: Coord = *neighbor;
                                empty_neighbors_of(rotate_board(subtracted_rotated_board), step)
                                    .iter().flat_map(|snd_dst| {
                                    vec![PureOpponentMove::TamMoveStepsDuringLatter {
                                        first_dest: to_absolute_coord(fst_dst, game_state.ia_is_down),
                                        second_dest: to_absolute_coord(*snd_dst, game_state.ia_is_down),
                                        src: to_absolute_coord(src, game_state.ia_is_down),
                                        step: to_absolute_coord(step, game_state.ia_is_down),
                                    }].into_iter()
                                }).collect::<Vec<PureOpponentMove>>().into_iter()
                            }
                        }).collect::<Vec<PureOpponentMove>>());
                    } else {
                        /* not an empty square: must complete the first move */
                        let step = dest;
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
                                            vec![PureOpponentMove::TamMoveStepsDuringFormer {
                                                first_dest: to_absolute_coord(
                                                    *fst_dst,
                                                    game_state.ia_is_down,
                                                ),
                                                second_dest: to_absolute_coord(
                                                    *snd_dst,
                                                    game_state.ia_is_down,
                                                ),
                                                src: to_absolute_coord(src, game_state.ia_is_down),
                                                step: to_absolute_coord(step, game_state.ia_is_down),
                                            }]
                                            .into_iter()
                                        })
                                        .collect::<Vec<PureOpponentMove>>()
                                        .into_iter()
                                })
                                .collect::<Vec<PureOpponentMove>>(),
                        );
                    }
                }
                Piece::NonTam2Piece {
                    color: rotated_piece_color,
                    prof: rotated_piece_prof,
                    side: _,
                } => {
                    if dest_piece == None {
                        // cannot step
                        let obj: PureOpponentMoveWithPotentialWaterEntry =
                            PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcDst {
                                src: to_absolute_coord(src, game_state.ia_is_down),
                                dest: to_absolute_coord(dest, game_state.ia_is_down),
                                is_water_entry_ciurl: is_ciurl_required(
                                    dest,
                                    rotated_piece_prof,
                                    src,
                                ),
                            };
                        ans.append(&mut vec![PureOpponentMove::PotentialWaterEntry(obj)]);
                    } else if dest_piece == Some(Piece::Tam2) {
                        // if allowed by config, allow stepping on Tam2;
                        if config.allow_kut2tam2 {
                            ans.append(&mut candidates_when_stepping(rotated_piece));
                        } else {
                            ans.append(&mut vec![]);
                        }
                    } else if let Some(Piece::NonTam2Piece {
                        side: Side::Upward,
                        color: _,
                        prof: _,
                    }) = dest_piece
                    {
                        // opponent's piece; stepping and taking both attainable

                        // except when protected by tam2 hue a uai1
                        if !can_get_occupied_by(
                            Side::Downward,
                            dest,
                            Piece::NonTam2Piece {
                                color: rotated_piece_color,
                                prof: rotated_piece_prof,
                                side: Side::Downward,
                            },
                            game_state.f.current_board,
                            game_state.tam_itself_is_tam_hue,
                        ) {
                            ans.append(&mut candidates_when_stepping(rotated_piece));
                        } else {
                            ans.append(   
                                &mut [
                                    &[PureOpponentMove::PotentialWaterEntry(
                                        PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcDst {
                                            src: to_absolute_coord(src, game_state.ia_is_down),
                                            dest: to_absolute_coord(dest, game_state.ia_is_down),
                                            is_water_entry_ciurl: is_ciurl_required(
                                                dest,
                                                rotated_piece_prof,
                                                src,
                                            ),
                                        },
                                    )][..],
                                    &candidates_when_stepping(rotated_piece)[..],
                                ]
                                .concat(),
                            );
                        }
                    } else {
                        ans.append(&mut candidates_when_stepping(rotated_piece));
                    }
                }
            }
        }
    }

    ans
}

fn get_opponent_pieces_rotated(game_state: &PureGameState) -> Vec<Rotated> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let coord = [rand_i, rand_j];
            let piece = game_state.f.current_board[rand_i][rand_j];
            if let Some(p) = piece {
                match p {
                    Piece::Tam2 => ans.push(Rotated {
                        rotated_piece: p,
                        rotated_coord: rotate_coord(coord),
                    }),
                    Piece::NonTam2Piece {
                        side: Side::Downward,
                        prof,
                        color,
                    } => {
                        let rot_piece = NonTam2PieceUpward { prof, color };
                        ans.push(Rotated {
                            rotated_piece: rot_piece.into(),
                            rotated_coord: rotate_coord(coord),
                        });
                    }
                    _ => {}
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

impl From<NonTam2PieceUpward> for Piece {
    fn from(from: NonTam2PieceUpward) -> Piece {
        Piece::NonTam2Piece {
            color: from.color,
            prof: from.prof,
            side: Side::Upward,
        }
    }
}

impl From<NonTam2PieceDownward> for Piece {
    fn from(from: NonTam2PieceDownward) -> Piece {
        Piece::NonTam2Piece {
            color: from.color,
            prof: from.prof,
            side: Side::Downward,
        }
    }
}

fn rotate_coord(c: Coord) -> Coord {
    [(8 - c[0]), (8 - c[1])]
}

pub type Coord = [usize; 2];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AbsoluteRow {
    A,
    E,
    I,
    U,
    O,
    Y,
    AI,
    AU,
    IA,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AbsoluteColumn {
    K,
    L,
    N,
    T,
    Z,
    X,
    C,
    M,
    P,
}

pub type AbsoluteCoord = (AbsoluteRow, AbsoluteColumn);

pub struct Rotated {
    rotated_piece: Piece,
    rotated_coord: Coord,
}

mod serialize;

#[derive(Clone, Copy)]
pub enum PureOpponentMoveWithPotentialWaterEntry {
    NonTamMoveSrcDst {
        src: AbsoluteCoord,
        dest: AbsoluteCoord,
        is_water_entry_ciurl: bool,
    },

    NonTamMoveSrcStepDstFinite {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        dest: AbsoluteCoord,
        is_water_entry_ciurl: bool,
    },
}

#[derive(Clone, Copy)]
pub enum PureOpponentMove {
    PotentialWaterEntry(PureOpponentMoveWithPotentialWaterEntry),
    InfAfterStep {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        planned_direction: AbsoluteCoord,
    },
    NonTamMoveFromHand {
        color: Color,
        prof: Profession,
        dest: AbsoluteCoord,
    },
    TamMoveNoStep {
        src: AbsoluteCoord,
        first_dest: AbsoluteCoord,
        second_dest: AbsoluteCoord,
    },
    TamMoveStepsDuringFormer {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        first_dest: AbsoluteCoord,
        second_dest: AbsoluteCoord,
    },
    TamMoveStepsDuringLatter {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        first_dest: AbsoluteCoord,
        second_dest: AbsoluteCoord,
    },
}

struct Config {
    allow_kut2tam2: bool,
}

#[cfg(test)]
mod tests;

fn rotate_piece_or_null(p: Option<Piece>) -> Option<Piece> {
    let p = p?;
    match p {
        Piece::Tam2 => Some(p),
        Piece::NonTam2Piece { prof, color, side } => Some(Piece::NonTam2Piece {
            prof,
            color,
            side: !side,
        }),
    }
}

impl std::ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Upward => Side::Downward,
            Side::Downward => Side::Upward,
        }
    }
}

fn rotate_board(b: Board) -> Board {
    let mut ans: Board = [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ];
    for i in 0..9 {
        for j in 0..9 {
            ans[i][j] = rotate_piece_or_null(b[8 - i][8 - j]);
        }
    }
    ans
}

type Board = [Row; 9];
type Row = [Option<Piece>; 9];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Piece {
    Tam2,
    NonTam2Piece {
        color: Color,
        prof: Profession,
        side: Side,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Kok1,  // Red, 赤
    Huok2, // Black, 黒
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Profession {
    Nuak1, // Vessel, 船, felkana
    Kauk2, // Pawn, 兵, elmer
    Gua2,  // Rook, 弓, gustuer
    Kaun1, // Bishop, 車, vadyrd
    Dau2,  // Tiger, 虎, stistyst
    Maun1, // Horse, 馬, dodor
    Kua2,  // Clerk, 筆, kua
    Tuk2,  // Shaman, 巫, terlsk
    Uai1,  // General, 将, varxle
    Io,    // King, 王, ales
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Side {
    Upward,
    Downward,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonTam2PieceDownward {
    color: Color,
    prof: Profession,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonTam2PieceUpward {
    color: Color,
    prof: Profession,
}

#[derive(Debug)]
pub struct PureGameState {
    f: Field,
    ia_is_down: bool,
    tam_itself_is_tam_hue: bool,
    opponent_has_just_moved_tam: bool,
}

#[derive(Debug)]
pub struct Field {
    current_board: Board,
    hop1zuo1of_upward: Vec<NonTam2PieceUpward>,
    hop1zuo1of_downward: Vec<NonTam2PieceDownward>,
}
