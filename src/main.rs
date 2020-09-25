fn from_hand_candidates(game_state: &PureGameState) -> Vec<PureOpponentMove> {
    let mut ans = vec![];
    for piece in &game_state.f.hop1zuo1of_downward {
        for empty_square in empty_squares(&game_state) {
            ans.push(PureOpponentMove::NonTamMoveFromHand {
                color: piece.color,
                prof: piece.prof,
                dest: toAbsoluteCoord_(empty_square, game_state.ia_is_down),
            })
        }
    }
    ans
}

mod calculate_movable;

fn is_water([row, col]: Coord) -> bool {
    return (row == 4 && col == 2)
        || (row == 4 && col == 3)
        || (row == 4 && col == 4)
        || (row == 4 && col == 5)
        || (row == 4 && col == 6)
        || (row == 2 && col == 4)
        || (row == 3 && col == 4)
        || (row == 5 && col == 4)
        || (row == 6 && col == 4);
}

fn toAbsoluteCoord_(coord: Coord, ia_is_down: bool) -> AbsoluteCoord {
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

    return (
        rows[if ia_is_down { row } else { 8 - row }],
        columns[if ia_is_down { col } else { 8 - col }],
    );
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
    return if piece_to_move == Piece::Tam2 {
        let [i, j] = dest;
        let dest_piece = board[i][j];
        /* It is allowed to enter an empty square */
        dest_piece == None
    } else {
        can_get_occupied_by_non_tam(side, dest, board, tam_itself_is_tam_hue)
    };
}

fn empty_neighbors_of(board: Board, c: Coord) -> Vec<Coord> {
    calculate_movable::eight_neighborhood(c)
        .iter()
        .filter(|a| {
            let [i, j] = a;
            board[*i][*j] == None
        })
        .map(|a| *a)
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
                return match piece {
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
                };
            })
            .collect::<Vec<_>>()
            .len()
            > 0
    };

    let [i, j] = dest;
    let dest_piece = board[i][j];

    return match dest_piece {
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
    };
}

fn not_from_hand_candidates_(config: Config, game_state: &PureGameState) -> Vec<PureOpponentMove> {
    let mut ans = vec![];
    for Rotated {
        rotated_piece,
        rotated_coord,
    } in get_opponent_pieces_rotated(&game_state)
    {
        let MovablePositions {
            finite: guideListYellow,
            infinite: guideListGreen,
        } = calculate_movable::calculate_movable_positions(
            rotated_coord,
            rotated_piece,
            rotate_board(game_state.f.current_board),
            game_state.tam_itself_is_tam_hue,
        );

        let candidates: Vec<Coord> = [
            &guideListYellow
                .into_iter()
                .map(rotate_coord)
                .collect::<Vec<_>>()[..],
            &guideListGreen
                .into_iter()
                .map(rotate_coord)
                .collect::<Vec<_>>()[..],
        ]
        .concat();

        let src: Coord = rotate_coord(rotated_coord);

        for dest in candidates {
            fn is_ciurl_required(dest: Coord, moving_piece_prof: Profession, src: Coord) -> bool {
                return is_water(dest) && !is_water(src) && moving_piece_prof != Profession::Nuak1;
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
                return [
                  &candidates.iter().flat_map(|final_dest| {
                      let (rotated_piece_color , rotated_piece_prof) = match rotated_piece {
                          Piece::Tam2 => panic!(),
                          Piece::NonTam2Piece{color, prof, side} => (color, prof)
                      };
                    if
                      can_get_occupied_by(
                          Side::Downward,
                          *final_dest,
                          Piece::NonTam2Piece{
                          color: rotated_piece_color,
                          prof: rotated_piece_prof,
                          side: Side::Downward
                        },
                          rotate_board(subtracted_rotated_board),
                          game_state.tam_itself_is_tam_hue
                      )
                     {
                      let obj: PureOpponentMoveWithPotentialWaterEntry = PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcStepDstFinite {
                          src: toAbsoluteCoord_(src, game_state.ia_is_down),
                          step: toAbsoluteCoord_(step, game_state.ia_is_down),
                          dest: toAbsoluteCoord_(*final_dest, game_state.ia_is_down),
                          is_water_entry_ciurl: is_ciurl_required(
                              *final_dest,
                              rotated_piece_prof,
                              src
                          )
                      };
                      return vec![PureOpponentMove::PotentialWaterEntry(obj)].into_iter();
                    } else {return vec![].into_iter()};
                  }).collect::<Vec<PureOpponentMove>>()[..],
                  &candidates_inf.iter().flat_map(|planned_dest| {
                    let (rotated_piece_color , rotated_piece_prof) = match rotated_piece {
                        Piece::Tam2 => panic!(),
                        Piece::NonTam2Piece{color, prof, side} => (color, prof)
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
                      src: toAbsoluteCoord_(src, game_state.ia_is_down),
                      step: toAbsoluteCoord_(step, game_state.ia_is_down),
                      planned_direction: toAbsoluteCoord_(
                          *planned_dest,
                          game_state.ia_is_down
                      ),
                    };
                    return vec![obj].into_iter();
                  }).collect::<Vec<PureOpponentMove>>()[..]
                ].concat();
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
                            return if game_state.f.current_board[neighbor[0]][neighbor[1]] ==
                                None /* the neighbor is utterly occupied */ ||
                                *neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                let snd_dst: Coord = *neighbor;
                                vec![PureOpponentMove::TamMoveNoStep {
                                    second_dest: toAbsoluteCoord_(snd_dst, game_state.ia_is_down),
                                    first_dest: toAbsoluteCoord_(fst_dst, game_state.ia_is_down),
                                    src: toAbsoluteCoord_(src, game_state.ia_is_down),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: Coord = *neighbor;
                                empty_neighbors_of(rotate_board(subtracted_rotated_board), step)
                                    .iter().flat_map(|snd_dst| {
                                    return vec![PureOpponentMove::TamMoveStepsDuringLatter {
                                        first_dest: toAbsoluteCoord_(fst_dst, game_state.ia_is_down),
                                        second_dest: toAbsoluteCoord_(*snd_dst, game_state.ia_is_down),
                                        src: toAbsoluteCoord_(src, game_state.ia_is_down),
                                        step: toAbsoluteCoord_(step, game_state.ia_is_down),
                                    }].into_iter();
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
                                                first_dest: toAbsoluteCoord_(
                                                    *fst_dst,
                                                    game_state.ia_is_down,
                                                ),
                                                second_dest: toAbsoluteCoord_(
                                                    *snd_dst,
                                                    game_state.ia_is_down,
                                                ),
                                                src: toAbsoluteCoord_(src, game_state.ia_is_down),
                                                step: toAbsoluteCoord_(step, game_state.ia_is_down),
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
                    side: rotated_piece_side,
                } => {
                    if dest_piece == None {
                        // cannot step
                        let obj: PureOpponentMoveWithPotentialWaterEntry =
                            PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcDst {
                                src: toAbsoluteCoord_(src, game_state.ia_is_down),
                                dest: toAbsoluteCoord_(dest, game_state.ia_is_down),
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
                        color,
                        prof,
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
                                            src: toAbsoluteCoord_(src, game_state.ia_is_down),
                                            dest: toAbsoluteCoord_(dest, game_state.ia_is_down),
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
    return [(8 - c[0]), (8 - c[1])];
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

fn main() {
    println!("Hello, world!");
}

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
    return ans;
}

const INITIAL_MOVES_NO_KUT_TAM: [&'static str; 240] = [
    "KA片LAKA",
    "KA片KE心KA", /* 黒筆 */
    "LA片TIXO水",
    "LA片TILO",
    "LA片TILA", /* 黒馬 */
    "NA片NIKO",
    "NA片ZANA",
    "NA片KANA", /* 黒車 */
    "TA片ZE",
    "TA片NE", /* 黒将不踏 */
    "TA片NANE",
    "TA片TENE",
    "TA片TEZE",
    "TA片ZAZE",
    "TA片ZATA",
    "TA片NATA", /* 黒将踏 */
    "ZA片ZE",   /* 赤王不踏 */
    "ZA片XECE",
    "ZA片XACE", /* 赤王踏而CE */
    "ZA片TEZA",
    "ZA片TAZA",
    "ZA片XAZA",
    "ZA片XEZA", /* 赤王踏而ZA */
    "ZA片TEZE",
    "ZA片TAZE",
    "ZA片XAZE",
    "ZA片XEZE", /* 赤王踏而ZE */
    "ZA片TENE",
    "ZA片TANE", /* 赤王踏而NE */
    "XA片CE",
    "XA片ZE", /* 赤将不踏 */
    "XA片XECE",
    "XA片XEZE",
    "XA片CACE",
    "XA片CAXA",
    "XA片ZAZE",
    "XA片ZAXA", /* 赤将踏 */
    "CA片CIPO",
    "CA片PACA",
    "CA片ZACA", /* 赤車 */
    "MA片XIMO",
    "MA片XITO水",
    "MA片XIMA", /* 赤馬 */
    "PA片MAPA",
    "PA片PE心PA", /* 赤筆 */
    "KE片KIKU",
    "KE片KIKE",
    "KE片KAKE",
    "KE片LE心NE",
    "KE片LE心KE", /* 赤巫 */
    "LE片NE",      /* 赤弓不踏 */
    "LE片LI心LU",
    "LE片LI心LO",
    "LE片LI心LY",
    "LE片LI心LAI",
    "LE片LI心LE", /* 赤弓踏前 */
    "LE片LA心LE", /* 赤弓踏後 */
    "LE片TE心ZE",
    "LE片TE心NE",
    "LE片TE心LE", /* 赤弓踏左 */
    "LE片KE心LE",
    "LE片KE心NE", /* 赤弓踏右 */
    "TE片ZIXU",
    "TE片ZITU",
    "TE片ZITE", /* 赤虎踏船 */
    "TE片NI心TU",
    "TE片NI心KO",
    "TE片NI心LU",
    "TE片NI心TE", /* 赤虎踏兵 */
    "TE片ZATE",    /* 赤虎踏王 */
    "TE片NATE",    /* 赤虎踏車 */
    "XE片ZIXU",
    "XE片ZITU",
    "XE片ZIXE", /* 黒虎踏船 */
    "XE片CI心PO",
    "XE片CI心MU",
    "XE片CI心XU",
    "XE片CI心XE", /* 黒虎踏水 */
    "XE片ZAXE",    /* 黒虎踏王 */
    "XE片CAXE",    /* 黒虎踏車 */
    "ME片CE",      /* 黒弓不踏 */
    "ME片MI心MU",
    "ME片MI心MO",
    "ME片MI心MY",
    "ME片MI心MAI",
    "ME片MI心ME", /* 黒弓踏前 */
    "ME片MA心ME", /* 黒弓踏後 */
    "ME片XE心CE",
    "ME片XE心ME",
    "ME片XE心ZE", /* 黒弓踏右 */
    "ME片PE心ME",
    "ME片PE心CE", /* 黒弓踏左 */
    "PE片PIPU",
    "PE片PIPE",
    "PE片PAPE",
    "PE片ME心PE",
    "PE片ME心CE", /* 黒巫 */
    "KI片KU",
    "LI片LU",
    "TI片TU",
    "ZI片ZU",
    "XI片XU",
    "MI片MU",
    "PI片PU", /* 兵 */
    "NI片NU",
    "NI片TITU",
    "NI片LILU",
    "NI片NE",
    "NI片NO水", /* 皇処之兵 */
    "CI片CU",
    "CI片MIMU",
    "CI片XIXU",
    "CI片CE",
    "CI片CO水", /* 皇処之兵 */
    /* 皇 */
    "ZO皇[XY]ZO",
    "ZO皇[XY]XO",
    "ZO皇[XY]CO",
    "ZO皇[XY]ZY",
    "ZO皇[XY]CY",
    "ZO皇[XY]ZAITY",
    "ZO皇[XY]ZAIZY",
    "ZO皇[XY]ZAIXY",
    "ZO皇[XY]ZAIZAU",
    "ZO皇[XY]XAIZY",
    "ZO皇[XY]XAIXY",
    "ZO皇[XY]XAICY",
    "ZO皇[XY]XAIZAU",
    "ZO皇[XY]XAICAU",
    "ZO皇[XY]CAIXY",
    "ZO皇[XY]CAICY",
    "ZO皇[XY]CAIMY",
    "ZO皇[XY]CAICAU",
    "ZO皇[ZY]TO",
    "ZO皇[ZY]ZO",
    "ZO皇[ZY]XO",
    "ZO皇[ZY]TY",
    "ZO皇[ZY]XY",
    "ZO皇[ZY]TAINY",
    "ZO皇[ZY]TAITY",
    "ZO皇[ZY]TAIZY",
    "ZO皇[ZY]TAINAU",
    "ZO皇[ZY]TAIZAU",
    "ZO皇[ZY]ZAITY",
    "ZO皇[ZY]ZAIZY",
    "ZO皇[ZY]ZAIXY",
    "ZO皇[ZY]ZAIZAU",
    "ZO皇[ZY]XAIZY",
    "ZO皇[ZY]XAIXY",
    "ZO皇[ZY]XAICY",
    "ZO皇[ZY]XAIZAU",
    "ZO皇[ZY]XAICAU",
    "ZO皇[TY]NO",
    "ZO皇[TY]TO",
    "ZO皇[TY]ZO",
    "ZO皇[TY]NY",
    "ZO皇[TY]ZY",
    "ZO皇[TY]NAILY",
    "ZO皇[TY]NAINY",
    "ZO皇[TY]NAITY",
    "ZO皇[TY]NAINAU",
    "ZO皇[TY]TAINY",
    "ZO皇[TY]TAITY",
    "ZO皇[TY]TAIZY",
    "ZO皇[TY]TAINAU",
    "ZO皇[TY]TAIZAU",
    "ZO皇[TY]ZAITY",
    "ZO皇[TY]ZAIZY",
    "ZO皇[TY]ZAIXY",
    "ZO皇[TY]ZAIZAU",
    "ZO皇[XO]ZU",
    "ZO皇[XO]XU",
    "ZO皇[XO]CU",
    "ZO皇[XO]ZO",
    "ZO皇[XO]CO",
    "ZO皇[XO]ZY",
    "ZO皇[XO]XY",
    "ZO皇[XO]CY",
    "ZO皇[TO]NU",
    "ZO皇[TO]TU",
    "ZO皇[TO]ZU",
    "ZO皇[TO]NO",
    "ZO皇[TO]ZO",
    "ZO皇[TO]NY",
    "ZO皇[TO]TY",
    "ZO皇[TO]ZY",
    "ZO皇[XU]ZU",
    "ZO皇[XU]CU",
    "ZO皇[XU]ZO",
    "ZO皇[XU]XO",
    "ZO皇[XU]CO",
    "ZO皇[XU]ZIZE",
    "ZO皇[XU]ZITU",
    "ZO皇[XU]ZIZU",
    "ZO皇[XU]ZIXU",
    "ZO皇[XU]XIZE",
    "ZO皇[XU]XICE",
    "ZO皇[XU]XIZU",
    "ZO皇[XU]XIXU",
    "ZO皇[XU]XICU",
    "ZO皇[XU]CICE",
    "ZO皇[XU]CIXU",
    "ZO皇[XU]CICU",
    "ZO皇[XU]CIMU",
    "ZO皇[ZU]TU",
    "ZO皇[ZU]XU",
    "ZO皇[ZU]TO",
    "ZO皇[ZU]ZO",
    "ZO皇[ZU]XO",
    "ZO皇[ZU]TINE",
    "ZO皇[ZU]TIZE",
    "ZO皇[ZU]TINU",
    "ZO皇[ZU]TITU",
    "ZO皇[ZU]TIZU",
    "ZO皇[ZU]ZIZE",
    "ZO皇[ZU]ZITU",
    "ZO皇[ZU]ZIZU",
    "ZO皇[ZU]ZIXU",
    "ZO皇[ZU]XIZE",
    "ZO皇[ZU]XICE",
    "ZO皇[ZU]XIZU",
    "ZO皇[ZU]XIXU",
    "ZO皇[ZU]XICU",
    "ZO皇[TU]NU",
    "ZO皇[TU]ZU",
    "ZO皇[TU]NO",
    "ZO皇[TU]TO",
    "ZO皇[TU]ZO",
    "ZO皇[TU]NINE",
    "ZO皇[TU]NILU",
    "ZO皇[TU]NINU",
    "ZO皇[TU]NITU",
    "ZO皇[TU]TINE",
    "ZO皇[TU]TIZE",
    "ZO皇[TU]TINU",
    "ZO皇[TU]TITU",
    "ZO皇[TU]TIZU",
    "ZO皇[TU]ZIZE",
    "ZO皇[TU]ZITU",
    "ZO皇[TU]ZIZU",
    "ZO皇[TU]ZIXU",
];

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

const INITIAL_BOARD: Board = [
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kua2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Maun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kaun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Uai1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Io,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Uai1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kaun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Maun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kua2,
            side: Side::Downward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Tuk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Gua2,
            side: Side::Downward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Dau2,
            side: Side::Downward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Dau2,
            side: Side::Downward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Gua2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Tuk2,
            side: Side::Downward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Nuak1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
    ],
    [None, None, None, None, None, None, None, None, None],
    [
        None,
        None,
        None,
        None,
        Some(Piece::Tam2),
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Nuak1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Tuk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Gua2,
            side: Side::Upward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Dau2,
            side: Side::Upward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Dau2,
            side: Side::Upward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Gua2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Tuk2,
            side: Side::Upward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kua2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Maun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kaun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Uai1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Io,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Uai1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kaun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Maun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kua2,
            side: Side::Upward,
        }),
    ],
];

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

const INITIAL_BOARD_SAMPLE: PureGameState = PureGameState {
    ia_is_down: true,
    tam_itself_is_tam_hue: true,
    opponent_has_just_moved_tam: false,
    f: Field {
        hop1zuo1of_downward: vec![],
        hop1zuo1of_upward: vec![],
        current_board: INITIAL_BOARD,
    },
};

const TAM_ITSELF_IS_NOT_TAM_HUE_SAMPLE: PureGameState = PureGameState {
    ia_is_down: true,
    tam_itself_is_tam_hue: false,
    opponent_has_just_moved_tam: false,
    f: Field {
        hop1zuo1of_downward: vec![],
        hop1zuo1of_upward: vec![],
        current_board: INITIAL_BOARD,
    },
};

const SIMPLE_BOARD: Board = [
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kua2,
            side: Side::Downward,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
];

mod test_cases;
