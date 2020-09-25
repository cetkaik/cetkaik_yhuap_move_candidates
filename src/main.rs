fn from_hand_candidates(gameState: &PureGameState) -> Vec<PureOpponentMove> {
    let mut ans = vec![];
    for piece in &gameState.f.hop1zuo1OfDownward {
        for empty_square in empty_squares(&gameState) {ans.push(
            PureOpponentMove::NonTamMoveFromHand {
              color: piece.color,
              prof: piece.prof,
              dest: toAbsoluteCoord_(empty_square, gameState.IA_is_down)
            }
        )}
    }
    ans
}

fn toAbsoluteCoord_(
    coord: Coord,
    IA_is_down: bool
  ) -> AbsoluteCoord {
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
      AbsoluteColumn::P
    ];
  
    let rows = vec![AbsoluteRow::A, AbsoluteRow::E, AbsoluteRow::I, AbsoluteRow::U, AbsoluteRow::O, AbsoluteRow::Y, AbsoluteRow::AI, AbsoluteRow::AU, AbsoluteRow::IA];
  
    return (
      rows[if IA_is_down {row } else  {8 - row}],
      columns[if IA_is_down {col} else {8 - col}]
    );
  }
 
/*  
pub struct MovablePositions {
    finite: Vec<Coord>,
    infinite: Vec<Coord>
}

fn calculateMovablePositions(
    coord: Coord,
    piece: Piece,
    board: Board,
    tam_itself_is_tam_hue: bool
) -> MovablePositions {
    unimplemented!()
}  
*/

fn not_from_hand_candidates_(config: Config, gameState: &PureGameState) -> Vec<PureOpponentMove> {
    /*let mut ans = vec![];
    for Rotated { rotated_piece, rotated_coord} in get_opponent_pieces_rotated(&gameState)  {
            let MovablePositions {
              finite: guideListYellow,
              infinite: guideListGreen
            } = calculateMovablePositions(
              rotated_coord,
              rotated_piece,
              rotateBoard(gameState.f.currentBoard),
              gameState.tam_itself_is_tam_hue
            );
      
           let candidates: Vec<Coord> = [
              ...guideListYellow.map(rotateCoord),
              ...guideListGreen.map(rotateCoord)
            ];
      
            const src: Coord = rotateCoord(rotated_coord);
      
            return candidates.flatMap((dest: Coord): PureOpponentMove[] => {
              function is_ciurl_required(
                dest: Coord,
                moving_piece_prof: Profession,
                src: Coord
              ) {
                return (
                  isWater(dest) &&
                  !isWater(src) &&
                  moving_piece_prof !== Profession.Nuak1
                );
              }
              const destPiece = gameState.f.currentBoard[dest[0]][dest[1]];
      
              const candidates_when_stepping = (
                rotated_piece: NonTam2PieceUpward
              ) => {
                const step = dest; // less confusing
      
                /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
                const subtracted_rotated_board = rotateBoard(
                  gameState.f.currentBoard
                );
                subtracted_rotated_board[rotated_coord[0]][
                  rotated_coord[1]
                ] = null; /* must remove the piece to prevent self-occlusion */
      
                const {
                  finite: guideListYellow,
                  infinite: guideListGreen
                } = calculateMovablePositions(
                  rotateCoord(step),
                  rotated_piece,
                  subtracted_rotated_board,
                  gameState.tam_itself_is_tam_hue
                );
      
                const candidates: Coord[] = guideListYellow.map(rotateCoord);
                const candidates_inf: Coord[] = guideListGreen.map(rotateCoord);
                return [
                  ...candidates.flatMap(finalDest => {
                    if (
                      canGetOccupiedBy(
                        Side.Downward,
                        finalDest,
                        {
                          color: rotated_piece.color,
                          prof: rotated_piece.prof,
                          side: Side.Downward
                        },
                        rotateBoard(subtracted_rotated_board),
                        gameState.tam_itself_is_tam_hue
                      )
                    ) {
                      const obj: PureOpponentMoveWithPotentialWaterEntry = {
                        type: "NonTamMove",
                        data: {
                          type: "SrcStepDstFinite",
                          src: toAbsoluteCoord_(src, gameState.IA_is_down),
                          step: toAbsoluteCoord_(step, gameState.IA_is_down),
                          dest: toAbsoluteCoord_(finalDest, gameState.IA_is_down),
                          is_water_entry_ciurl: is_ciurl_required(
                            finalDest,
                            rotated_piece.prof,
                            src
                          )
                        }
                      };
                      return [obj];
                    } else return [];
                  }),
                  ...candidates_inf.flatMap(planned_dest => {
                    if (
                      !canGetOccupiedBy(
                        Side.Downward,
                        planned_dest,
                        {
                          color: rotated_piece.color,
                          prof: rotated_piece.prof,
                          side: Side.Downward
                        },
                        rotateBoard(subtracted_rotated_board),
                        gameState.tam_itself_is_tam_hue
                      )
                    ) {
                      return [];
                      // retry
                    }
                    const obj: PureOpponentMove = {
                      type: "InfAfterStep",
                      src: toAbsoluteCoord_(src, gameState.IA_is_down),
                      step: toAbsoluteCoord_(step, gameState.IA_is_down),
                      plannedDirection: toAbsoluteCoord_(
                        planned_dest,
                        gameState.IA_is_down
                      ),
                      stepping_ciurl: null,
                      finalResult: null
                    };
                    return [obj];
                  })
                ];
              };
      
              if (rotated_piece === "Tam2") {
                /* avoid self-occlusion */
                const subtracted_rotated_board = rotateBoard(
                  gameState.f.currentBoard
                );
                subtracted_rotated_board[rotated_coord[0]][rotated_coord[1]] = null;
                // FIXME: tam2 ty sak2 not handled
                if (destPiece === null) {
                  /* empty square; first move is completed without stepping */
                  const fstdst: Coord = dest;
                  return eightNeighborhood(fstdst).flatMap(
                    (neighbor): PureOpponentMove[] => {
                      /* if the neighbor is empty, that is the second destination */
                      if (
                        gameState.f.currentBoard[neighbor[0]][neighbor[1]] ==
                          null /* the neighbor is utterly occupied */ ||
                        coordEq(
                          neighbor,
                          src
                        ) /* the neighbor is occupied by yourself, which means it is actually empty */
                      ) {
                        const snddst: Coord = neighbor;
                        return [
                          {
                            type: "TamMove",
                            stepStyle: "NoStep",
                            secondDest: toAbsoluteCoord_(
                              snddst,
                              gameState.IA_is_down
                            ),
                            firstDest: toAbsoluteCoord_(fstdst, gameState.IA_is_down),
                            src: toAbsoluteCoord_(src, gameState.IA_is_down)
                          }
                        ];
                      } else {
                        /* if not, step from there */
                        const step: Coord = neighbor;
                        return empty_neighbors_of(
                          rotateBoard(subtracted_rotated_board),
                          step
                        ).flatMap(snddst => {
                          return [
                            {
                              type: "TamMove",
                              stepStyle: "StepsDuringLatter",
                              firstDest: toAbsoluteCoord_(
                                fstdst,
                                gameState.IA_is_down
                              ),
                              secondDest: toAbsoluteCoord_(
                                snddst,
                                gameState.IA_is_down
                              ),
                              src: toAbsoluteCoord_(src, gameState.IA_is_down),
                              step: toAbsoluteCoord_(step, gameState.IA_is_down)
                            }
                          ];
                        });
                      }
                    }
                  );
                } else {
                  /* not an empty square: must complete the first move */
                  const step = dest;
                  return empty_neighbors_of(
                    rotateBoard(subtracted_rotated_board),
                    step
                  ).flatMap(fstdst =>
                    empty_neighbors_of(
                      rotateBoard(subtracted_rotated_board),
                      fstdst
                    ).flatMap(snddst => [
                      {
                        type: "TamMove",
                        stepStyle: "StepsDuringFormer",
                        firstDest: toAbsoluteCoord_(fstdst, gameState.IA_is_down),
                        secondDest: toAbsoluteCoord_(snddst, gameState.IA_is_down),
                        src: toAbsoluteCoord_(src, gameState.IA_is_down),
                        step: toAbsoluteCoord_(step, gameState.IA_is_down)
                      }
                    ])
                  );
                }
              } else if (destPiece === null) {
                // cannot step
                const obj: PureOpponentMoveWithPotentialWaterEntry = {
                  type: "NonTamMove",
                  data: {
                    type: "SrcDst",
                    src: toAbsoluteCoord_(src, gameState.IA_is_down),
                    dest: toAbsoluteCoord_(dest, gameState.IA_is_down),
                    is_water_entry_ciurl: is_ciurl_required(
                      dest,
                      rotated_piece.prof,
                      src
                    )
                  }
                };
                return [obj];
              } else if (destPiece === "Tam2") {
                // if allowed by config, allow stepping on Tam2;
                if (config.allow_kut2tam2) {
                  return candidates_when_stepping(rotated_piece);
                } else {
                  return [];
                }
              } else if (destPiece.side === Side.Upward) {
                // opponent's piece; stepping and taking both attainable
      
                // except when protected by tam2 hue a uai1
                if (
                  !canGetOccupiedBy(
                    Side.Downward,
                    dest,
                    {
                      color: rotated_piece.color,
                      prof: rotated_piece.prof,
                      side: Side.Downward
                    },
                    gameState.f.currentBoard,
                    gameState.tam_itself_is_tam_hue
                  )
                ) {
                  return candidates_when_stepping(rotated_piece);
                }
      
                return [
                  {
                    type: "NonTamMove",
                    data: {
                      type: "SrcDst",
                      src: toAbsoluteCoord_(src, gameState.IA_is_down),
                      dest: toAbsoluteCoord_(dest, gameState.IA_is_down),
                      is_water_entry_ciurl: is_ciurl_required(
                        dest,
                        rotated_piece.prof,
                        src
                      )
                    }
                  },
                  ...candidates_when_stepping(rotated_piece)
                ];
              } else {
                return candidates_when_stepping(rotated_piece);
              }
            });
          }
      
          */

    unimplemented!()
}

fn get_opponent_pieces_rotated (
    gameState: &PureGameState
)-> Vec<Rotated> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let coord = [rand_i, rand_j];
            let piece = gameState.f.currentBoard[rand_i][rand_j];
            if let Some(p) = piece {
                match p {
                    Piece::Tam2 => ans.push(Rotated{ rotated_piece: p, rotated_coord: rotateCoord(coord) }),
                    Piece::NonTam2Piece {side: Side::Downward, prof, color} => {
                        let rot_piece = NonTam2PieceUpward {
                            prof,
                            color,
                        };
                        ans.push(Rotated { rotated_piece: rot_piece.into(), rotated_coord: rotateCoord(coord) })
                        ;
                    }
                    _ => {}
                }
            }
        }
    }
    ans
}
    

fn empty_squares(gameState: &PureGameState) -> Vec<Coord> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let coord: Coord = [rand_i, rand_j];
            if gameState.f.currentBoard[rand_i][rand_j] == None {
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

fn rotateCoord(c: Coord) -> Coord {
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

mod serialize {
    use super::*;

    pub fn serializeCoord(coord: Coord) -> String {
        format!("[{},{}]", coord[0], coord[1])
    }

    pub fn serializeAbsoluteCoord(coord: AbsoluteCoord) -> String {
        let (row, column) = coord;
        format!(
            "{}{}",
            match column {
                AbsoluteColumn::K => "K",
                AbsoluteColumn::L => "L",
                AbsoluteColumn::M => "M",
                AbsoluteColumn::N => "N",
                AbsoluteColumn::P => "P",
                AbsoluteColumn::Z => "Z",
                AbsoluteColumn::X => "X",
                AbsoluteColumn::C => "C",
                AbsoluteColumn::T => "T",
            },
            match row {
                AbsoluteRow::A => "A",
                AbsoluteRow::E => "E",
                AbsoluteRow::I => "I",
                AbsoluteRow::O => "O",
                AbsoluteRow::U => "U",
                AbsoluteRow::Y => "Y",
                AbsoluteRow::IA => "IA",
                AbsoluteRow::AI => "AI",
                AbsoluteRow::AU => "AU",
            }
        )
    }

    pub fn serializeProf(prof: Profession) -> &'static str {
        match prof {
            Profession::Nuak1 => "船",
            Profession::Kauk2 => "兵",
            Profession::Gua2 => "弓",
            Profession::Kaun1 => "車",
            Profession::Dau2 => "虎",
            Profession::Maun1 => "馬",
            Profession::Kua2 => "筆",
            Profession::Tuk2 => "巫",
            Profession::Uai1 => "将",
            Profession::Io => "王",
        }
    }

    pub fn serializeColor(color: Color) -> &'static str {
        match color {
            Color::Huok2 => "黒",
            Color::Kok1 => "赤",
        }
    }

    pub fn serializeSide(side: Side) -> &'static str {
        match side {
            Side::Upward => "↑",
            Side::Downward => "↓",
        }
    }

    pub fn serializePiece(p: Piece) -> String {
        match p {
            Piece::Tam2 => "皇".to_string(),
            Piece::NonTam2Piece { prof, color, side } => format!(
                "{}{}{}",
                serializeColor(color),
                serializeProf(prof),
                serializeSide(side)
            ),
        }
    }

    pub fn serializeRotated(r: Rotated) -> String {
        format!(
            "{}{}",
            serializeCoord(r.rotated_coord),
            serializePiece(r.rotated_piece)
        )
    }

    pub fn serializePureOpponentMove(mov: PureOpponentMove) -> String {
        match mov {
            PureOpponentMove::InfAfterStep {
                src,
                step,
                plannedDirection,
            } => format!(
                "{}片{}心{}",
                serializeAbsoluteCoord(src),
                serializeAbsoluteCoord(step),
                serializeAbsoluteCoord(plannedDirection)
            ),
            PureOpponentMove::NonTamMoveFromHand { color, prof, dest } => format!(
                "{}{}{}",
                serializeColor(color),
                serializeProf(prof),
                serializeAbsoluteCoord(dest)
            ),
            PureOpponentMove::PotentialWaterEntry(
                PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcDst {
                    src,
                    dest,
                    is_water_entry_ciurl,
                },
            ) => format!(
                "{}片{}{}",
                serializeAbsoluteCoord(src),
                serializeAbsoluteCoord(dest),
                if is_water_entry_ciurl { "水" } else { "" }
            ),
            PureOpponentMove::PotentialWaterEntry(
                PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcStepDstFinite {
                    src,
                    dest,
                    is_water_entry_ciurl,
                    step,
                },
            ) => format!(
                "{}片{}{}{}",
                serializeAbsoluteCoord(src),
                serializeAbsoluteCoord(step),
                serializeAbsoluteCoord(dest),
                if is_water_entry_ciurl { "水" } else { "" }
            ),
            PureOpponentMove::TamMoveNoStep {
                src,
                firstDest,
                secondDest,
            } => format!(
                "{}皇{}{}",
                serializeAbsoluteCoord(src),
                serializeAbsoluteCoord(firstDest),
                serializeAbsoluteCoord(secondDest)
            ),
            PureOpponentMove::TamMoveStepsDuringFormer {
                src,
                firstDest,
                secondDest,
                step,
            } => format!(
                "{}皇{}[{}]{}",
                serializeAbsoluteCoord(src),
                serializeAbsoluteCoord(step),
                serializeAbsoluteCoord(firstDest),
                serializeAbsoluteCoord(secondDest)
            ),
            PureOpponentMove::TamMoveStepsDuringLatter {
                src,
                firstDest,
                secondDest,
                step,
            } => format!(
                "{}皇[{}]{}{}",
                serializeAbsoluteCoord(src),
                serializeAbsoluteCoord(firstDest),
                serializeAbsoluteCoord(step),
                serializeAbsoluteCoord(secondDest)
            ),
        }
    }
}

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

pub enum PureOpponentMove {
    PotentialWaterEntry(PureOpponentMoveWithPotentialWaterEntry),
    InfAfterStep {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        plannedDirection: AbsoluteCoord,
    },
    NonTamMoveFromHand {
        color: Color,
        prof: Profession,
        dest: AbsoluteCoord,
    },
    TamMoveNoStep {
        src: AbsoluteCoord,
        firstDest: AbsoluteCoord,
        secondDest: AbsoluteCoord,
    },
    TamMoveStepsDuringFormer {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        firstDest: AbsoluteCoord,
        secondDest: AbsoluteCoord,
    },
    TamMoveStepsDuringLatter {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        firstDest: AbsoluteCoord,
        secondDest: AbsoluteCoord,
    },
}

struct Config {
    allow_kut2tam2: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn not_from_hand_candidates(gameState: &PureGameState) -> Vec<PureOpponentMove> {
        not_from_hand_candidates_(
            Config {
                allow_kut2tam2: false,
            },
            gameState,
        )
    }

    fn not_from_hand_candidates_with_kut2tam2(gameState: &PureGameState) -> Vec<PureOpponentMove> {
        not_from_hand_candidates_(
            Config {
                allow_kut2tam2: true,
            },
            gameState,
        )
    }

    fn runTest<T, F, F2>(
        fun: F,
        sample: &PureGameState,
        serializer: F2,
        tested_against: &[&'static str],
    ) -> bool
    where
        F: FnOnce(&PureGameState) -> Vec<T>,
        F2: Fn(T) -> String,
    {
        use std::collections::HashSet;
        let set1 = fun(sample)
            .into_iter()
            .map(serializer)
            .collect::<HashSet<String>>();
        let set2 = tested_against
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();

        set1 == set2
    }

    
    
    mod test_not_from_hand_candidates_with_kut2tam2 {
        use crate::serialize::*;
        
        #[test]
fn test_tamCornerSample() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates_with_kut2tam2,
                    &crate::test_cases::tamCornerSample(),
                    serializePureOpponentMove,
                    &[
                        "CAI片XAI",
                        "CAI片CY",
                        "MAI片MAUMAI",
                        "PAI片PAUPAI",
                        "MAU片MIAMAU",
                        "MAU片PAUMAU",
                        "MAU片MAIMAU",
                        "PAU片MAUPAU",
                        "PAU片PAIPAU",
                        "MIA片MAUMIA",
                        /* 撃皇 */
                        "PAU片PIAPAU",
                        "MIA片PIAMIA",
                    ],
                ),
                true,
            );
        }

        #[test]
fn test_tamItselfIsNotTamHueSample() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates_with_kut2tam2,
                    &crate::tamItselfIsNotTamHueSample,
                    serializePureOpponentMove,
                    &[
                        &crate::INITIAL_MOVES_NO_KUT_TAM[..],
                        &vec![
                            "ZI片ZOXO",
                            "ZI片ZOTO",
                            "ZI片ZOCO",
                            "ZI片ZONO",
                            "ZI片ZO心ZY",
                            "ZI片ZO心ZAI",
                            "ZI片ZO心ZU",
                            "ZI片ZO心ZI",
                            "ZI片ZO心ZE",
                        ][..],
                    ]
                    .concat(),
                ),
                true,
            );
        }

        #[test]
fn test_initialBoardSample() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates_with_kut2tam2,
                    &crate::initialBoardSample,
                    serializePureOpponentMove,
                    &[
                        &crate::INITIAL_MOVES_NO_KUT_TAM[..],
                        &vec![
                            "ZI片ZOXO",
                            "ZI片ZOTO",
                            "ZI片ZOCO",
                            "ZI片ZONO",
                            "ZI片ZO心ZY",
                            "ZI片ZO心ZAI",
                            "ZI片ZO心ZU",
                            "ZI片ZO心ZI",
                            "ZI片ZO心ZE",
                        ],
                    ]
                    .concat(),
                ),
                true,
            );
        }
    }

    mod not_from_hand_candidates {
        use crate::serialize::*;
        #[test]
fn test_simpleBoardSample_3() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::test_cases::simpleBoardSample_3(),
                    serializePureOpponentMove,
                    &["PI片PU"],
                ),
                true,
            );
        }
        #[test]
fn test_simpleBoardSample_1_IA_is_down_true() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::test_cases::simpleBoardSample_1(),
                    serializePureOpponentMove,
                    &[
                        "KA片LA", /* horizontal */
                        "KA片KE",
                        "KA片KI",
                        "KA片KU",
                        "KA片KO",
                        "KA片KY",     /* vertical */
                        "KA片KAI",    /* take */
                        "KA片KAILAI", /* step and then horizontal */
                        "KA片KAI心KAU",
                        "KA片KAI心KIA", /* step and then plan to proceed */
                        "KA片KAI心KY",
                        "KA片KAI心KO",
                        "KA片KAI心KU",
                        "KA片KAI心KI",
                        "KA片KAI心KE",
                        "KA片KAI心KA", /* step and then plan u-turn */
                    ],
                ),
                true,
            );
        }
        #[test]
fn test_simpleBoardSample_2_IA_is_down_false() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::test_cases::simpleBoardSample_2(),
                    serializePureOpponentMove,
                    &[
                        "PIA片MIA", /* horizontal */
                        "PIA片PAU",
                        "PIA片PAI",
                        "PIA片PY",
                        "PIA片PO",
                        "PIA片PU",   /* vertical */
                        "PIA片PI",   /* take */
                        "PIA片PIMI", /* step and then horizontal */
                        "PIA片PI心PE",
                        "PIA片PI心PA", /* step and then plan to proceed */
                        "PIA片PI心PU",
                        "PIA片PI心PO",
                        "PIA片PI心PY",
                        "PIA片PI心PAI",
                        "PIA片PI心PAU",
                        "PIA片PI心PIA", /* step and then plan u-turn */
                    ],
                ),
                true,
            );
        }
        #[test]
fn test_initialBoardSample() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::initialBoardSample,
                    serializePureOpponentMove,
                    &crate::INITIAL_MOVES_NO_KUT_TAM,
                ),
                true,
            );
        }
        #[test]
fn test_complicatedBoardSample_1() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::test_cases::complicatedBoardSample_1(),
                    serializePureOpponentMove,
                    &[
                        "XA片XE",
                        "TE片ZI水",
                        "TE片ZITU",
                        "TE片ZIXE",
                        "TE片ZITE",
                        "TE片NI心TU",
                        "TE片NI心LU",
                        "TE片NI心ZA",
                        "TE片NI心TE",
                        "TE片NI心LE",
                        "TE片ZA",
                        "TE片NA",
                        "TE片NATE",
                        "TE片NALE",
                        "ZE片XICU",
                        "ZE片XIZI水",
                        "ZE片XICE",
                        "ZE片XIZE",
                        "ZE片ZI水",
                        "ZE片ZITU",
                        "ZE片ZIXE",
                        "ZE片TITU",
                        "ZE片TINU",
                        "ZE片TIZI水",
                        "ZE片TIZE",
                        "ZE片TINE",
                        "ZE片XE",
                        "ZE片XEZI水",
                        "ZE片XECE",
                        "ZE片XEZE",
                        "ZE片XECA",
                        "ZE片XEZA",
                        "ZE片TEZI水",
                        "ZE片TEZE",
                        "ZE片TENE",
                        "ZE片TEZA",
                        "ZE片TENA",
                        "ZE片XACE",
                        "ZE片XAXE",
                        "ZE片XAZE",
                        "ZE片XACA",
                        "ZE片XAZA",
                        "ZE片TA",
                        "ZE片TAZE",
                        "ZE片TANE",
                        "ZE片TAZA",
                        "ZE片TANA",
                        "ME片MIMU",
                        "KI片KO",
                        "KI片KA",
                        "KI片NIKO",
                        "KI片NIKA",
                        "KI片NIZA",
                        "NI片ZO心LAU",
                        "NI片ZO心NAI",
                        "NI片ZO心LE",
                        "NI片ZO心NI",
                        "NI片KO",
                        "NI片ZA",
                        "NI片KA",
                        "TI片XO水",
                        "TI片XOTAI",
                        "TI片XOTI",
                        "TI片LO",
                        "TI片LA",
                        "TI片XATI",
                        "XI片XU心CO",
                        "XI片XU心ZI",
                        "XI片XE",
                        "XI片XE心XI",
                        "XI片XE心CE",
                        "XI片CI心PO",
                        "XI片CI心MU",
                        "XI片CI心XE",
                        "XI片ZI水",
                        "XI片ZI心XI",
                        "CI片MU",
                        "CI片CU",
                        "CI片XUCO水",
                        "CI片XUXO水",
                        "CI片XUCU",
                        "CI片XUCI",
                        "CI片XUZI水",
                        "CI片MIMU",
                        "CI片MICU",
                        "CI片MIPI",
                        "CI片MICI",
                        "CI片MIPE",
                        "CI片MICE",
                        "CI片XICU",
                        "CI片XICI",
                        "CI片XIZI水",
                        "CI片XICE",
                        "CI片MEPI",
                        "CI片MECI",
                        "CI片MEPE",
                        "CI片MECE",
                        "CI片MEPA",
                        "CI片MECA",
                        "CI片CE",
                        "CI片CECI",
                        "CI片CEXE",
                        "CI片CEMA",
                        "CI片XE",
                        "CI片XECI",
                        "CI片XEZI水",
                        "CI片XECE",
                        "CI片XECA",
                        "CI片XEZA",
                        "MI片MU",
                        "MI片MEMI",
                        "MI片MEMA",
                        "MI片ME心PE",
                        "MI片ME心CE",
                        "MI片PI",
                        "MI片PIPE",
                        "MI片PI心MI",
                        "MI片CI心CU",
                        "MI片CI心CO",
                        "MI片CI心CE",
                        "MI片CI心CA",
                        "MI片CI心MI",
                        "MI片CI心PI",
                        "MI片CI心ZI",
                        "MI片CI心PO",
                        "MI片CI心MU",
                        "MI片CI心PA",
                        "MI片CI心ZA",
                        "MI片CI心XE",
                        "ZU片ZOZY",
                        "ZU片ZOXO",
                        "ZU片ZOZU",
                        "ZU片ZOZAI",
                        "XU片XO水",
                        "XU片XOXY",
                        "XU片CU",
                        "XU片XIXU",
                        "PU片PO",
                        "PU片PY",
                        "PU片PY心PAI",
                        "TO片NO",
                        "TO片TY",
                        "TO片TAI",
                        "TO片TAU",
                        "TO片TAUZAU",
                        "TO片TAUNAU",
                        "TO片TAU心TIA",
                        "TO片TAU心TAI",
                        "TO片TAU心TY",
                        "TO片TAU心TO",
                        "TO片TAU心TU",
                        "TO片ZO心ZY",
                        "TO片ZO心ZAI",
                        "TO片ZO心XO",
                        "TO片ZO心TO",
                        "TO片ZO心NO",
                        "TO片ZO心LO",
                        "TO片ZO心KO",
                        "TO片TU",
                        "TO片TU心TO",
                        "TO片TU心TY",
                        "TO片TU心TAI",
                        "TO片TU心TAU",
                        "TO片TU心NU",
                        "ZO片XY",
                        "ZO片XYZAI",
                        "ZO片XYZY",
                        "ZO片XYCO",
                        "ZO片XYXO",
                        "ZO片XYZO",
                        "ZO片ZY",
                        "ZO片TY",
                        "ZO片XO",
                        "ZO片XOXY",
                        "ZO片XOZY",
                        "ZO片XOCO",
                        "ZO片XOZO",
                        "ZO片XOCU",
                        "ZO片TOZY",
                        "ZO片TOTY",
                        "ZO片TONY",
                        "ZO片TOZO",
                        "ZO片TONO",
                        "ZO片TONU",
                        "ZO片XUCO",
                        "ZO片XUXO",
                        "ZO片XUZO",
                        "ZO片XUCU",
                        "ZO片XUZI",
                        "ZO片ZUXO",
                        "ZO片ZUZO",
                        "ZO片ZUTU",
                        "ZO片TU",
                        "ZO片TUZO",
                        "ZO片TUNO",
                        "ZO片TUNU",
                        "ZO片TUZI",
                        "LY片LAI",
                        "LY片LAI心LAU",
                        "LY片LAI心LIA",
                        "CY片CAI心PIA",
                        "CY片CAI心MAU",
                        "CY片CAI心XAU",
                        "CY片CAI心XY",
                        "CY片CO水",
                        "CY片CO心CY",
                        "CY片CO心CU",
                        "CY片CO心XO",
                        "CY片MY心PY",
                        "CY片MY心CY",
                        "CY片MY心XY",
                        "CY片XY",
                        "CY片XY心ZAI",
                        "CY片XY心CO",
                        "MY片MAIMAU",
                        "MY片MAIPAI",
                        "MY片MAIMY",
                        "MY片MAIMIA",
                        "XAI片XAU",
                        "CAI片CAU",
                        "CAI片CAUMAU",
                        "CAI片CAUXAU",
                        "CAI片CAU心CAI",
                        "CAI片CYXY",
                        "CAI片CY心CAI",
                        "CAI片CY心CAU",
                        "CAI片CY心CO",
                        "CAI片MAI心MAU",
                        "CAI片MAI心MIA",
                        "CAI片MAI心PAI",
                        "CAI片MAI心CAI",
                        "CAI片XAICAI",
                        "CAI片XAIZAI水",
                        "CAI片XAI心XAU",
                        "CAI片XAI心XY",
                        "MAI片MAU",
                        "MAI片CAU",
                        "MAI片CAUMIA",
                        "MAI片CAUMAU",
                        "MAI片CAUXAU",
                        "MAI片CAUMAI",
                        "MAI片PAI",
                        "MAI片CAIMAU",
                        "MAI片CAICAU",
                        "MAI片CAIXAU",
                        "MAI片CAIMAI",
                        "MAI片CAIXY",
                        "MAI片PY",
                        "MAI片PYPAI",
                        "MAI片PYMAI",
                        "MAI片PYPO",
                        "MAI片MYPAI",
                        "MAI片MYMAI",
                        "MAI片MYPY",
                        "MAI片MYPO",
                        "MAI片MYCO水",
                        "MAI片CYMAI",
                        "MAI片CYXY",
                        "MAI片CYCO水",
                        "MAI片CYXO水",
                        "KIA片KAI",
                        "KIA片NIA",
                        "XIA片ZAU",
                        "XIA片CAU",
                        "XIA片CAUMIA",
                        "XIA片CAUXIA",
                        /* 皇 */
                        "PAU皇[PIA]MAU",
                        "PAU皇[PIA]PAU",
                        "PAU皇[PIA]MIA",
                        "PAU皇[MIA]CAUMAU",
                        "PAU皇[MIA]CAUMIA",
                        "PAU皇[MIA]CIAMAU",
                        "PAU皇[MIA]CIAMIA",
                        "PAU皇[MIA]MAU",
                        "PAU皇[MIA]PAU",
                        "PAU皇[MIA]PIA",
                        "PAU皇[MAU]CAUMAU",
                        "PAU皇[MAU]CAUMIA",
                        "PAU皇[MAU]CAIMAU",
                        "PAU皇[MAU]CIAMAU",
                        "PAU皇[MAU]CIAMIA",
                        "PAU皇[MAU]PAI",
                        "PAU皇[MAU]PAU",
                        "PAU皇[MAU]MIA",
                        "PAU皇[MAU]PIA",
                        "PAU皇[MAU]MAIPAI",
                        "PAU皇[MAU]MAIMAU",
                        "PAU皇[MAU]MAIPAU",
                        "PAU皇[PAI]MAU",
                        "PAU皇[PAI]PAU",
                        "PAU皇[PAI]MYPO",
                        "PAU皇[PAI]MYPAI",
                        "PAU皇[PAI]PYPO",
                        "PAU皇[PAI]PYPAI",
                        "PAU皇[PAI]MAIPAI",
                        "PAU皇[PAI]MAIMAU",
                        "PAU皇[PAI]MAIPAU",
                        "PAU皇MAI[PAI]MAU",
                        "PAU皇MAI[PAI]PAU",
                        "PAU皇MAI[MAU]PAI",
                        "PAU皇MAI[MAU]MIA",
                        "PAU皇MAI[MAU]PIA",
                        "PAU皇MAI[MAU]PAU",
                        "PAU皇MAI[PAU]PAI",
                        "PAU皇MAI[PAU]MAU",
                        "PAU皇MAI[PAU]MIA",
                        "PAU皇MAI[PAU]PIA",
                    ],
                ),
                true,
            );
        }
        #[test]
fn test_complicatedBoardSample_2() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::test_cases::complicatedBoardSample_2(),
                    serializePureOpponentMove,
                    &[
                        "ZA片ZE",
                        "KE皇LI[KE]KA",
                        "KE皇LI[KE]LA",
                        "KE皇LI[KE]LE",
                        "KE皇LI[KE]KI",
                        "KE皇LI[LE]KA",
                        "KE皇LI[LE]LA",
                        "KE皇LI[LE]KE",
                        "KE皇LI[LE]KI",
                        "KE皇LI[KI]KE",
                        "KE皇LI[KI]LE",
                        "KE皇[KI]KE",
                        "KE皇[KI]LE",
                        "KE皇[KI]LIKE",
                        "KE皇[KI]LILE",
                        "KE皇[KI]LIKI",
                        "KE皇[KI]KUKI",
                        "KE皇[KI]KUKO",
                        "KE皇[KI]LUKI",
                        "KE皇[KI]LUKO",
                        "KE皇[LE]KA",
                        "KE皇[LE]LA",
                        "KE皇[LE]NALA",
                        "KE皇[LE]NALE",
                        "KE皇[LE]KE",
                        "KE皇[LE]NELA",
                        "KE皇[LE]NELE",
                        "KE皇[LE]KI",
                        "KE皇[LE]LIKE",
                        "KE皇[LE]LILE",
                        "KE皇[LE]LIKI",
                        "KE皇[LE]NILE",
                        "KE皇[LA]KA",
                        "KE皇[LA]NALA",
                        "KE皇[LA]NALE",
                        "KE皇[LA]KE",
                        "KE皇[LA]LE",
                        "KE皇[LA]NELA",
                        "KE皇[LA]NELE",
                        "KE皇[KA]LA",
                        "KE皇[KA]KE",
                        "KE皇[KA]LE",
                        "NE片NI",
                        "NE片NINU",
                        "NE片NITI",
                        "NE片NILI",
                        "NE片NINE",
                        "TE片TI",
                        "XE片XI",
                        "CE片CI",
                        "ZI片ZU",
                        "ZI片ZE",
                        "ZI片XI",
                        "ZI片CI",
                        "ZI片ZO",
                        "ZI片ZO心PA",
                        "ZI片ZO心ME",
                        "ZI片ZO心CI",
                        "ZI片ZO心XU",
                        "ZI片ZA心ZE",
                        "ZI片ZA心ZI",
                        "ZI片ZA心ZU",
                        "ZI片ZA心ZO",
                        "ZI片ZA心XA",
                        "ZI片ZA心CA",
                        "ZI片ZA心MA",
                        "ZI片ZA心PA",
                        "ZI片ZA心TA",
                        "ZI片MI心MU",
                        "ZI片MI心ME",
                        "ZI片MI心MA",
                        "ZI片MI心PI",
                        "ZI片MI心CI",
                        "ZI片MI心XI",
                        "ZI片MI心ZI",
                        "ZI片MI心TI",
                        "ZI片TI",
                        "ZI片TI心ZI",
                        "ZI片TI心XI",
                        "ZI片TI心CI",
                        "ZI片TI心NI",
                        "MI片MO",
                        "MI片MA",
                        "MI片XI",
                        "KU片NY",
                        "KU片NEZU水",
                        "KU片NEKU",
                        "TU片XY心MU",
                        "TU片XY心NI",
                        "TU片XY心TU",
                        "TU片LY",
                        "TU片XEMU",
                        "TU片XETU",
                        "TU片KA",
                        "TU片LE",
                        "NO片NY",
                        "NO片NU",
                        "NO片NUNO",
                        "NO片NUNI",
                        "NO片NU心LU",
                        "NO片TO心ZO",
                        "NO片TO心NO",
                        "NO片TO心LO",
                        "NO片LO",
                        "NO片LOLY",
                        "NO片LOLU",
                        "NO片LO心NO",
                        "NO片LO心KO",
                        "TO片TYNY",
                        "TO片TYTO",
                        "CY片CAI",
                        "CY片CAICAU",
                        "CY片CAIMAI",
                        "CY片CAIXAI",
                        "CY片CAICY",
                        "MY片MAI",
                        "MY片MO",
                        "MY片PY",
                        "MY片CYCAI",
                        "MY片CYCO水",
                        "MY片CY心MY",
                        "MY片CY心PY",
                        "KAI片LAINAI",
                        "KAI片LAIKAI",
                        "KAI片LAI心LY",
                        "KAI片LAI心LO",
                        "KAI片KAU心KIA",
                        "KAI片KAU心KAI",
                        "KAI片KAU心KY",
                        "KAI片KY",
                        "KAI片KYLY",
                        "KAI片KY心KAI",
                        "KAI片KY心KO",
                        "ZAI片XAU",
                        "ZAI片XAUZIA",
                        "ZAI片XAUCAU",
                        "ZAI片XAUZAU",
                        "ZAI片XAUCAI",
                        "ZAI片XAUXAI",
                        "ZAI片XAUZAI",
                        "ZAI片ZAU",
                        "ZAI片ZAUZIA",
                        "ZAI片ZAUTIA",
                        "ZAI片ZAUXAU",
                        "ZAI片ZAUXAI",
                        "ZAI片ZAUZAI",
                        "ZAI片TAUZIA",
                        "ZAI片TAUTIA",
                        "ZAI片TAUZAU",
                        "ZAI片TAUZAI",
                        "ZAI片TAUNAI",
                        "ZAI片XAI",
                        "ZAI片XAICAU",
                        "ZAI片XAIXAU",
                        "ZAI片XAIZAU",
                        "ZAI片XAICAI",
                        "ZAI片XAIZAI",
                        "ZAI片TAIZAU",
                        "ZAI片TAIZAI",
                        "ZAI片TAINAI",
                        "ZAI片TAINY",
                        "ZAI片XYCAI",
                        "ZAI片XYXAI",
                        "ZAI片XYZAI",
                        "ZAI片XYCO",
                        "ZAI片XYZO",
                        "ZAI片ZYXAI",
                        "ZAI片ZYZAI",
                        "ZAI片ZYZO",
                        "ZAI片TYZAI",
                        "ZAI片TYNAI",
                        "ZAI片TYNY",
                        "ZAI片TYZO",
                        "KAU片KIA",
                        "KAU片KAI心KAU",
                        "KAU片KAI心KIA",
                        "KAU片KAI心KY",
                        "KAU片LAU心LIA",
                        "KAU片LAU心KAU",
                        "NAU片TIA",
                        "NAU片TIAZIA",
                        "NAU片TIAZAU",
                        "NAU片TIANAU",
                        "NAU片NIATIA",
                        "NAU片NIALIA",
                        "NAU片LIA",
                        "NAU片TAUZIA",
                        "NAU片TAUTIA",
                        "NAU片TAUZAU",
                        "NAU片TAUNAU",
                        "NAU片TAUNAI",
                        "NAU片LAULIA",
                        "NAU片LAUKIA",
                        "NAU片LAUNAU",
                        "NAU片LAUNAI",
                        "NAU片TAIZAU",
                        "NAU片TAINAU",
                        "NAU片TAINAI",
                        "NAU片TAINY",
                        "NAU片LAINAU",
                        "NAU片LAINAI",
                        "NAU片LAINY",
                        "NAU片LAIKY",
                        "TAU片ZIA",
                        "TAU片NIATAU",
                        "TAU片ZAIXAU",
                        "TAU片ZAITAU",
                        "TAU片NAI",
                        "TAU片NAI心ZIA",
                        "TAU片NAI心TAU",
                        "TAU片NAI心KO",
                        "TAU片NAI心LY",
                        "MAU片PIA",
                        "MAU片CIAMAU",
                        "MAU片CIAXAU",
                        "MAU片PAI",
                        "MAU片PAIMAU",
                        "MAU片CAI",
                        "MAU片CAI心PIA",
                        "MAU片CAI心MAU",
                        "MAU片CAI心XAU",
                        "XIA片CIAMIA",
                        "XIA片CIAXIA",
                        "XIA片CIA心CAU",
                        "XIA片CIA心CAI",
                        "XIA片ZIA",
                        "XIA片XAU",
                        "XIA片XAUCAU",
                        "XIA片XAUZAU",
                        "XIA片XAU心XIA",
                        "XIA片XAU心XAI",
                        "CIA片CAI",
                        "CIA片CAIPIA",
                        "CIA片CAIZIA",
                        "CIA片CAIZO水",
                        "CIA片CAIPO",
                        "CIA片PIA",
                        "CIA片ZIA",
                        /* 皇処之巫 */
                        "XY片CO水",
                        "XY片MU",
                        "XY片MUMO",
                        "XY片MU心PU",
                        "XY片MU心CU",
                        "XY片MU心XU",
                        "XY片MU心ZU",
                        "XY片PI",
                        "XY片XU",
                        "XY片XI",
                        "XY片XOXY",
                        "XY片XOXU",
                        "XY片XO心CO",
                        "XY片XO心MO",
                        "XY片XO心PO",
                        "XY片XO心ZO",
                        "XY片XAI",
                        "XY片XAIXAU",
                        "XY片XAIXY",
                        "XY片XAI心CAI",
                        "XY片XAU",
                        "XY片XAUXAI",
                        "XY片XAU心CAU",
                        "XY片XAU心ZAU",
                        "XY片XEXI",
                        "XY片XEXA",
                        "XY片XE心ZE",
                        "XY片CYCAI",
                        "XY片CYCO水",
                        "XY片CY心XY",
                        "XY片MYMAI",
                        "XY片MYMO",
                        "XY片MY心PY",
                        "XY片ZYZO水",
                        "XY片ZY心XY",
                        "XY片TAUTIA",
                        "XY片TAU心ZAU",
                        "XY片ZAIZAU",
                        "XY片ZAI心XAI",
                        "XY片MAUMIA",
                        "XY片MAUMAI",
                        "XY片MAU心PAU",
                        "XY片MAU心CAU",
                        "XY片MAU心XAU",
                        "XY片CAI",
                        "XY片CAI心CAU",
                        "XY片CAI心CO",
                        "XY片CAI心CU",
                        "XY片CAI心CI",
                        "XY片CAI心MAI",
                        "XY片CAI心PAI",
                        "XY片CAI心XAI",
                        "XY片CAI心PIA",
                        "XY片CAI心ZIA",
                        "XY片CAI心XAU",
                        "XY片CAI心PO",
                        "XY片CAI心ZO",
                        "XY片CAI心XY",
                        "XY片TY心XY",
                        "XY片TY心NY",
                        "XY片TY心LY",
                        "XY片TY心KY",
                        "XY片TY心XAU",
                        "XY片TY心NAI",
                        "XY片TY心PA",
                        "XY片TY心ME",
                        "XY片TY心CI",
                        "XY片TY心XU",
                        "XY片TY心ZO",
                        "XY片TY心LU",
                        "XY片TU心TI",
                        "XY片TU心ZU",
                        "XY片TU心XU",
                        "XY片TU心CU",
                        "XY片TU心MU",
                        "XY片TU心PU",
                        "XY片TU心NU",
                        "XY片TU心LU",
                        "XY片TU心CAI",
                        "XY片TU心XY",
                        "XY片TU心ZO",
                        "XY片TU心LY",
                        "XY片TU心KA",
                        "XY片TU心LE",
                        "XY片TU心NI",
                        "XY片ZO水",
                        "XY片ZO心ZU",
                        "XY片ZO心ZE",
                        "XY片ZO心CO",
                        "XY片ZO心MO",
                        "XY片ZO心PO",
                        "XY片ZO心CAI",
                        "XY片ZO心XY",
                        "XY片ZO心NAI",
                        "XY片ZO心PA",
                        "XY片ZO心ME",
                        "XY片ZO心CI",
                        "XY片ZO心XU",
                        "XY片ZO心NI",
                    ],
                ),
                true,
            );
        }
        #[test]
fn test_tamCornerSample() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::test_cases::tamCornerSample(),
                    serializePureOpponentMove,
                    &[
                        "CAI片XAI",
                        "CAI片CY",
                        "MAI片MAUMAI",
                        "PAI片PAUPAI",
                        "MAU片MIAMAU",
                        "MAU片PAUMAU",
                        "MAU片MAIMAU",
                        "PAU片MAUPAU",
                        "PAU片PAIPAU",
                        "MIA片MAUMIA",
                    ],
                ),
                true,
            );
        }
        #[test]
fn test_tamItselfIsNotTamHueSample() {
            assert_eq!(
                super::runTest(
                    super::not_from_hand_candidates,
                    &crate::tamItselfIsNotTamHueSample,
                    serializePureOpponentMove,
                    &crate::INITIAL_MOVES_NO_KUT_TAM,
                ),
                true,
            );
        }
    }

    mod empty_squares {
        use crate::serialize::*;
        #[test]
fn test_initialBoardSample() {
            assert_eq!(
                super::runTest(
                    crate::empty_squares,
                    &crate::initialBoardSample,
                    serializeCoord,
                    &[
                        "[1,2]", "[1,4]", "[1,6]", "[3,0]", "[3,1]", "[3,2]", "[3,3]", "[3,4]",
                        "[3,5]", "[3,6]", "[3,7]", "[3,8]", "[4,0]", "[4,1]", "[4,2]", "[4,3]",
                        "[4,5]", "[4,6]", "[4,7]", "[4,8]", "[5,0]", "[5,1]", "[5,2]", "[5,3]",
                        "[5,4]", "[5,5]", "[5,6]", "[5,7]", "[5,8]", "[7,2]", "[7,4]", "[7,6]",
                    ],
                ),
                true,
            );
        }
    }
    mod get_opponent_pieces_rotated {
        use crate::serialize::*;
        #[test]
fn test_initialBoardSample() {
            assert_eq!(
                super::runTest(
                    crate::get_opponent_pieces_rotated,
                    &crate::initialBoardSample,
                    serializeRotated,
                    &[
                        "[8,8] 黒筆↑",
                        "[8,7] 黒馬↑",
                        "[8,6] 黒車↑",
                        "[8,5] 黒将↑",
                        "[8,4] 赤王↑",
                        "[8,3] 赤将↑",
                        "[8,2] 赤車↑",
                        "[8,1] 赤馬↑",
                        "[8,0] 赤筆↑",
                        "[7,8] 赤巫↑",
                        "[7,7] 赤弓↑",
                        "[7,5] 赤虎↑",
                        "[7,3] 黒虎↑",
                        "[7,1] 黒弓↑",
                        "[7,0] 黒巫↑",
                        "[6,8] 黒兵↑",
                        "[6,7] 赤兵↑",
                        "[6,6] 黒兵↑",
                        "[6,5] 赤兵↑",
                        "[6,4] 赤船↑",
                        "[6,3] 赤兵↑",
                        "[6,2] 黒兵↑",
                        "[6,1] 赤兵↑",
                        "[6,0] 黒兵↑",
                        "[4,4] 皇",
                    ],
                ),
                true,
            );
        }
    }
    mod from_hand_candidates {
        use crate::serialize::*;
        #[test]
fn test_initialBoardSample() {
            assert_eq!(
                super::runTest(
                    crate::from_hand_candidates,
                    &crate::initialBoardSample,
                    serializePureOpponentMove,
                    &[],
                ),
                true,
            );
        }
        #[test]
fn test_simpleBoardSample_4() {
            assert_eq!(
                super::runTest(
                    crate::from_hand_candidates,
                    &crate::test_cases::simpleBoardSample_4(),
                    serializePureOpponentMove,
                    &[
                        "黒弓KA",
                        "黒弓LA",
                        "黒弓NA",
                        "黒弓TA",
                        "黒弓ZA",
                        "黒弓XA",
                        "黒弓CA",
                        "黒弓MA",
                        "黒弓PA",
                        "黒弓KE",
                        "黒弓LE",
                        "黒弓NE",
                        "黒弓TE",
                        "黒弓ZE",
                        "黒弓XE",
                        "黒弓CE",
                        "黒弓ME",
                        "黒弓PE",
                        "黒弓KI",
                        "黒弓LI",
                        "黒弓NI",
                        "黒弓TI",
                        "黒弓ZI",
                        "黒弓XI",
                        "黒弓CI",
                        "黒弓MI",
                        "黒弓KU",
                        "黒弓LU",
                        "黒弓NU",
                        "黒弓TU",
                        "黒弓ZU",
                        "黒弓XU",
                        "黒弓CU",
                        "黒弓MU",
                        "黒弓PU",
                        "黒弓KO",
                        "黒弓LO",
                        "黒弓NO",
                        "黒弓TO",
                        "黒弓ZO",
                        "黒弓XO",
                        "黒弓CO",
                        "黒弓MO",
                        "黒弓PO",
                        "黒弓KY",
                        "黒弓LY",
                        "黒弓NY",
                        "黒弓TY",
                        "黒弓ZY",
                        "黒弓XY",
                        "黒弓CY",
                        "黒弓MY",
                        "黒弓PY",
                        "黒弓KAI",
                        "黒弓LAI",
                        "黒弓NAI",
                        "黒弓TAI",
                        "黒弓ZAI",
                        "黒弓XAI",
                        "黒弓CAI",
                        "黒弓MAI",
                        "黒弓PAI",
                        "黒弓KAU",
                        "黒弓LAU",
                        "黒弓NAU",
                        "黒弓TAU",
                        "黒弓ZAU",
                        "黒弓XAU",
                        "黒弓CAU",
                        "黒弓MAU",
                        "黒弓PAU",
                        "黒弓KIA",
                        "黒弓LIA",
                        "黒弓NIA",
                        "黒弓TIA",
                        "黒弓ZIA",
                        "黒弓XIA",
                        "黒弓CIA",
                        "黒弓MIA",
                    ],
                ),
                true,
            );
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn rotatePieceOrNull(p: Option<Piece>) -> Option<Piece> {
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

fn rotateBoard(b: Board) -> Board {
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
            ans[i][j] = rotatePieceOrNull(b[8 - i][8 - j]);
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

const initialBoard: Board = [
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

pub struct PureGameState {
    f: Field,
    IA_is_down: bool,
    tam_itself_is_tam_hue: bool,
    opponent_has_just_moved_tam: bool,
}

pub struct Field {
    currentBoard: Board,
    hop1zuo1OfUpward: Vec<NonTam2PieceUpward>,
    hop1zuo1OfDownward: Vec<NonTam2PieceDownward>,
}

const initialBoardSample: PureGameState = PureGameState {
    IA_is_down: true,
    tam_itself_is_tam_hue: true,
    opponent_has_just_moved_tam: false,
    f: Field {
        hop1zuo1OfDownward: vec![],
        hop1zuo1OfUpward: vec![],
        currentBoard: initialBoard,
    },
};

const tamItselfIsNotTamHueSample: PureGameState = PureGameState {
    IA_is_down: true,
    tam_itself_is_tam_hue: false,
    opponent_has_just_moved_tam: false,
    f: Field {
        hop1zuo1OfDownward: vec![],
        hop1zuo1OfUpward: vec![],
        currentBoard: initialBoard,
    },
};

const simpleBoard: Board = [
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

mod test_cases {
    use super::*;
    pub fn complicatedBoard() -> Board {
        [
            [
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kaun1,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kua2,
                    side: Side::Upward,
                }),
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                    side: Side::Upward,
                }),
                None,
                None,
            ],
            [
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Dau2,
                    side: Side::Upward,
                }),
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Dau2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Uai1,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Dau2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Uai1,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Gua2,
                    side: Side::Upward,
                }),
            ],
            [
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kaun1,
                    side: Side::Downward,
                }),
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Maun1,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Maun1,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Io,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Gua2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Uai1,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Tuk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kua2,
                    side: Side::Upward,
                }),
            ],
            [
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Tuk2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Tuk2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Nuak1,
                    side: Side::Downward,
                }),
            ],
            [
                None,
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kua2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Uai1,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Tuk2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                None,
            ],
            [
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Nuak1,
                    side: Side::Downward,
                }),
                None,
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Maun1,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Gua2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Maun1,
                    side: Side::Upward,
                }),
            ],
            [
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kaun1,
                    side: Side::Upward,
                }),
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Gua2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kua2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Io,
                    side: Side::Downward,
                }),
                None,
            ],
            [
                None,
                None,
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
                None,
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
                None,
                Some(Piece::Tam2),
            ],
            [
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kaun1,
                    side: Side::Downward,
                }),
                None,
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                    side: Side::Upward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Dau2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                None,
                None,
            ],
        ]
    }

    pub fn tamCorner() -> Board {
        [
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
            ],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
            ],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::NonTam2Piece {
                    color: Color::Huok2,
                    prof: Profession::Kauk2,
                    side: Side::Downward,
                }),
                Some(Piece::Tam2),
            ],
        ]
    }

    pub fn simpleBoardSample_1() -> PureGameState {
        PureGameState {
            IA_is_down: true,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![],
                hop1zuo1OfUpward: vec![],
                currentBoard: simpleBoard,
            },
        }
    }
    pub fn simpleBoardSample_2() -> PureGameState {
        PureGameState {
            IA_is_down: false,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![],
                hop1zuo1OfUpward: vec![],
                currentBoard: simpleBoard,
            },
        }
    }

    pub fn simpleBoardSample_3() -> PureGameState {
        PureGameState {
            IA_is_down: true,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![],
                hop1zuo1OfUpward: vec![],
                currentBoard: rotateBoard(simpleBoard),
            },
        }
    }

    pub fn simpleBoardSample_4() -> PureGameState {
        PureGameState {
            IA_is_down: true,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![NonTam2PieceDownward {
                    color: Color::Huok2,
                    prof: Profession::Gua2,
                }],
                hop1zuo1OfUpward: vec![NonTam2PieceUpward {
                    color: Color::Kok1,
                    prof: Profession::Kauk2,
                }],
                currentBoard: rotateBoard(simpleBoard),
            },
        }
    }

    pub fn complicatedBoardSample_1() -> PureGameState {
        PureGameState {
            IA_is_down: true,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![],
                hop1zuo1OfUpward: vec![],
                currentBoard: complicatedBoard(),
            },
        }
    }

    pub fn complicatedBoardSample_2() -> PureGameState {
        PureGameState {
            IA_is_down: true,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![],
                hop1zuo1OfUpward: vec![],
                currentBoard: rotateBoard(complicatedBoard()),
            },
        }
    }

    pub fn tamCornerSample() -> PureGameState {
        PureGameState {
            IA_is_down: true,
            tam_itself_is_tam_hue: true,
            opponent_has_just_moved_tam: false,
            f: Field {
                hop1zuo1OfDownward: vec![],
                hop1zuo1OfUpward: vec![],
                currentBoard: tamCorner(),
            },
        }
    }
}
