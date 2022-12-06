use alloc::vec::Vec;

use super::{Board, Color, Coord, MovablePositions, NonTam2PieceUpward, Piece, Profession, Side};

pub mod vec {
    use super::{iter, Board, Coord, Vec};
    pub fn eight_neighborhood(coord: Coord) -> Vec<Coord> {
        apply_deltas(
            coord,
            &[
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, -1],
                [0, 1],
                [1, -1],
                [1, 0],
                [1, 1],
            ],
        )
    }
    pub fn apply_deltas(coord: Coord, deltas: &[[i32; 2]]) -> Vec<Coord> {
        let [i, j] = coord;
        deltas
            .iter()
            .map(|[delta_x, delta_y]| {
                [
                    i32::try_from(i).unwrap() + delta_x,
                    i32::try_from(j).unwrap() + delta_y,
                ]
            })
            .filter_map(|[l, m]| {
                if (0..=8).contains(&l) && (0..=8).contains(&m) {
                    Some([usize::try_from(l).unwrap(), usize::try_from(m).unwrap()])
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn apply_single_delta_if_no_intervention(
        coord: Coord,
        delta: [i32; 2],
        board: Board,
    ) -> Vec<Coord> {
        let mut blocker = iter::apply_deltas(coord, crate::get_blocker_deltas::ultrafast(delta));

        // if nothing is blocking the way
        if blocker.all(|[i, j]| board[i][j].is_none()) {
            apply_deltas(coord, &[delta])
        } else {
            vec![]
        }
    }

    pub fn apply_deltas_if_no_intervention(
        coord: Coord,
        deltas: &[[i32; 2]],
        board: Board,
    ) -> Vec<Coord> {
        iter::apply_deltas_if_no_intervention(coord, deltas, board).collect()
    }
}

pub mod iter {
    use super::{Board, Coord};
    pub fn eight_neighborhood(coord: Coord) -> impl Iterator<Item = Coord> {
        apply_deltas(
            coord,
            [
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, -1],
                [0, 1],
                [1, -1],
                [1, 0],
                [1, 1],
            ]
            .into_iter(),
        )
    }
    pub fn apply_deltas(
        coord: Coord,
        deltas: impl Iterator<Item = [i32; 2]>,
    ) -> impl Iterator<Item = Coord> {
        let [i, j] = coord;
        deltas
            .map(move |[delta_x, delta_y]| {
                [
                    i32::try_from(i).unwrap() + delta_x,
                    i32::try_from(j).unwrap() + delta_y,
                ]
            })
            .filter_map(|[l, m]| {
                if (0..=8).contains(&l) && (0..=8).contains(&m) {
                    Some([usize::try_from(l).unwrap(), usize::try_from(m).unwrap()])
                } else {
                    None
                }
            })
    }

    pub fn apply_single_delta_if_no_intervention(
        coord: Coord,
        delta: [i32; 2],
        board: Board,
    ) -> impl Iterator<Item = Coord> {
        let mut blocker = apply_deltas(coord, crate::get_blocker_deltas::ultrafast(delta));

        // if nothing is blocking the way
        apply_deltas(
            coord,
            if blocker.all(|[i, j]| board[i][j].is_none()) {
                Some(delta)
            } else {
                None
            }
            .into_iter(),
        )
    }

    pub fn apply_deltas_if_no_intervention(
        coord: Coord,
        deltas: &[[i32; 2]],
        board: Board,
    ) -> impl Iterator<Item = Coord> + '_ {
        deltas
            .iter()
            .copied()
            .flat_map(move |delta| apply_single_delta_if_no_intervention(coord, delta, board))
    }
}

pub fn is_tam_hue(coord: Coord, board: Board, tam_itself_is_tam_hue: bool) -> bool {
    // unconditionally TamHue
    if coord == [2, 2]
        || coord == [2, 6]
        || coord == [3, 3]
        || coord == [3, 5]
        || coord == [4, 4]
        || coord == [5, 3]
        || coord == [5, 5]
        || coord == [6, 2]
        || coord == [6, 6]
    {
        return true;
    }

    if tam_itself_is_tam_hue && board[coord[0]][coord[1]] == Some(Piece::Tam2) {
        return true;
    }

    // is Tam2 available at any neighborhood?
    iter::eight_neighborhood(coord).any(|[i, j]| board[i][j] == Some(Piece::Tam2))
}

fn apply_single_delta_if_zero_or_one_intervention(
    coord: Coord,
    delta: [i32; 2],
    board: Board,
) -> Vec<Coord> {
    let blocker = iter::apply_deltas(coord, crate::get_blocker_deltas::ultrafast(delta));

    // if no piece or a single piece is blocking the way
    if blocker.filter(|[i, j]| board[*i][*j].is_some()).count() <= 1 {
        vec::apply_deltas(coord, &[delta])
    } else {
        vec![]
    }
}

fn apply_deltas_if_zero_or_one_intervention(
    coord: Coord,
    deltas: &[[i32; 2]],
    board: Board,
) -> Vec<Coord> {
    let mut ans = vec![];
    for delta in deltas {
        ans.append(&mut apply_single_delta_if_zero_or_one_intervention(
            coord, *delta, board,
        ));
    }
    ans
}

/// Returns the list of all possible locations that a piece can move to / step on.
/// # Examples
/// ```
/// use cetkaik_yhuap_move_candidates::*;
/// use cetkaik_core::*;
/// use std::collections::HashSet;
///
/// fn assert_eq_ignoring_order<T>(a: &[T], b: &[T])
/// where
///     T: Eq + core::hash::Hash + std::fmt::Debug,
/// {
///     let a: HashSet<_> = a.iter().collect();
///     let b: HashSet<_> = b.iter().collect();
///
///     assert_eq!(a, b)
/// }
///
/// let MovablePositions { finite, infinite } =
///     calculate_movable_positions_for_either_side(
///         [2, 0], /* if, at [2,0], */
///         relative::Piece::NonTam2Piece {
///             color: Color::Huok2,
///             prof: Profession::Kua2,
///             side: relative::Side::Downward,
///         }, /* a black Kua2 belonging to the opponent exists, */
///         [
///             [
///                 Some(relative::Piece::NonTam2Piece {
///                     color: Color::Huok2,
///                     prof: Profession::Gua2,
///                     side: relative::Side::Downward,
///                 }), /* while the opponent's Gua2 is in [0,0] and */
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///             ],
///             [None, None, None, None, None, None, None, None, None],
///             [None, None, None, None, None, None, None, None, None],
///             [None, None, None, None, None, None, None, None, None],
///             [None, None, None, None, None, None, None, None, None],
///             [None, None, None, None, None, None, None, None, None],
///             [
///                 Some(relative::Piece::NonTam2Piece {
///                     color: Color::Huok2,
///                     prof: Profession::Kauk2,
///                     side: relative::Side::Upward,
///                 }), /* your Kauk2 in [6,0], */
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///                 None,
///             ],
///             [None, None, None, None, None, None, None, None, None],
///             [None, None, None, None, None, None, None, None, None],
///         ],
///         false
///     );
///
/// /* then the opponent's Gua2 can either move one step to the side, */
/// assert_eq_ignoring_order(&finite, &vec![[2, 1]]);
///
/// /* or it can run to anywhere from [0,0] to [6,0].
///  * Note that you need two calls to this function in order to handle stepping. */
/// assert_eq_ignoring_order(&infinite, &vec![[3, 0], [4, 0], [5, 0], [6, 0], [1, 0], [0, 0]]);
/// ```
#[must_use]
pub fn calculate_movable_positions_for_either_side(
    coord: Coord,
    piece: Piece,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> MovablePositions {
    match piece {
        Piece::Tam2 => calculate_movable_positions_for_tam(coord),
        Piece::NonTam2Piece {
            prof,
            color: _,
            side,
        } => {
            calculate_movable_positions_for_nontam(coord, prof, board, tam_itself_is_tam_hue, side)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TamOrUpwardPiece {
    Tam2,
    NonTam2Piece { color: Color, prof: Profession },
}

impl From<NonTam2PieceUpward> for TamOrUpwardPiece {
    fn from(piece: NonTam2PieceUpward) -> TamOrUpwardPiece {
        TamOrUpwardPiece::NonTam2Piece {
            color: piece.color,
            prof: piece.prof,
        }
    }
}

impl From<TamOrUpwardPiece> for Piece {
    fn from(p: TamOrUpwardPiece) -> Piece {
        match p {
            TamOrUpwardPiece::Tam2 => Piece::Tam2,
            TamOrUpwardPiece::NonTam2Piece { color, prof } => Piece::NonTam2Piece {
                color,
                prof,
                side: Side::Upward,
            },
        }
    }
}

pub fn calculate_movable_positions_for_tam(coord: Coord) -> MovablePositions {
    MovablePositions {
        finite: vec::eight_neighborhood(coord),
        infinite: vec![],
    }
}

pub fn calculate_movable_positions_for_nontam(
    coord: Coord,
    prof: Profession,
    board: Board,
    tam_itself_is_tam_hue: bool,
    side: Side,
) -> MovablePositions {
    const DIAGONAL: [[i32; 2]; 32] = [
        // UP_LEFT:
        [-8, -8],
        [-7, -7],
        [-6, -6],
        [-5, -5],
        [-4, -4],
        [-3, -3],
        [-2, -2],
        [-1, -1],
        // UP_RIGHT:
        [-8, 8],
        [-7, 7],
        [-6, 6],
        [-5, 5],
        [-4, 4],
        [-3, 3],
        [-2, 2],
        [-1, 1],
        // DOWN_LEFT:
        [8, -8],
        [7, -7],
        [6, -6],
        [5, -5],
        [4, -4],
        [3, -3],
        [2, -2],
        [1, -1],
        // DOWN_RIGHT:
        [8, 8],
        [7, 7],
        [6, 6],
        [5, 5],
        [4, 4],
        [3, 3],
        [2, 2],
        [1, 1],
    ];
    const UP: [[i32; 2]; 8] = [
        [-1, 0],
        [-2, 0],
        [-3, 0],
        [-4, 0],
        [-5, 0],
        [-6, 0],
        [-7, 0],
        [-8, 0],
    ];
    const DOWN: [[i32; 2]; 8] = [
        [1, 0],
        [2, 0],
        [3, 0],
        [4, 0],
        [5, 0],
        [6, 0],
        [7, 0],
        [8, 0],
    ];
    const LEFT_RIGHT: [[i32; 2]; 16] = [
        [0, -1],
        [0, -2],
        [0, -3],
        [0, -4],
        [0, -5],
        [0, -6],
        [0, -7],
        [0, -8],
        [0, 1],
        [0, 2],
        [0, 3],
        [0, 4],
        [0, 5],
        [0, 6],
        [0, 7],
        [0, 8],
    ];

    let piece_prof = prof;
    if is_tam_hue(coord, board, tam_itself_is_tam_hue) {
        match piece_prof {
           Profession::Io | Profession::Uai1 => // General, 将, varxle
            MovablePositions { finite: vec::eight_neighborhood(coord), infinite: vec![] },
            Profession::Kaun1 =>
            MovablePositions {
              finite: vec::apply_deltas(coord, &[
                [-2, -2],
                [-2, 2],
                [2, 2],
                [2, -2]
              ]),
              infinite: vec![]
            }, // 車, vadyrd
            Profession::Kauk2 => // Pawn, 兵, elmer
            MovablePositions  {
              finite: [
                &vec::apply_deltas(coord, &[
                  [-1, 0],
                  [0, -1],
                  [0, 1],
                  [1, 0]
                ])[..],
                &vec::apply_single_delta_if_no_intervention(coord,  if side == Side::Upward {[-2, 0]} else {[2,0]}, board)[..]
              ].concat(),
              infinite: vec![]
            },
            Profession::Nuak1 => // Vessel, 船, felkana
            MovablePositions  {
              finite: [
                &vec::apply_deltas(coord, &[
                  [0, -1],
                  [0, 1]
                ])[..],
                &vec::apply_deltas_if_no_intervention(
                  coord,
                  &[
                    [0, -2],
                    [0, 2]
                  ],
                  board
                )[..]
              ].concat(),
              infinite: vec::apply_deltas_if_no_intervention(coord, &[&UP[..], &DOWN[..]].concat(), board)
            },
            Profession::Gua2 | // Rook, 弓, gustuer
            Profession::Dau2 => // Tiger, 虎, stistyst
               MovablePositions {
                finite: vec![],
                infinite: vec::apply_deltas_if_no_intervention(
                    coord,
                    &DIAGONAL,
                    board
                )
              },
              Profession::Maun1 => {
                // Horse, 馬, dodor
                const HORSE_DELTAS: [[i32; 2] ; 28] = [
                  [-8, -8],
                  [-7, -7],
                  [-6, -6],
                  [-5, -5],
                  [-4, -4],
                  [-3, -3],
                  [-2, -2],
                  [-8, 8],
                  [-7, 7],
                  [-6, 6],
                  [-5, 5],
                  [-4, 4],
                  [-3, 3],
                  [-2, 2],
                  [8, -8],
                  [7, -7],
                  [6, -6],
                  [5, -5],
                  [4, -4],
                  [3, -3],
                  [2, -2],
                  [8, 8],
                  [7, 7],
                  [6, 6],
                  [5, 5],
                  [4, 4],
                  [3, 3],
                  [2, 2]
                ];
                let mut inf: Vec<Coord> = vec![];
                for delta in &HORSE_DELTAS {
                  let blocker_deltas = crate::get_blocker_deltas::ultrafast(*delta).filter(
                    |d|
                      /*
                       * remove [-1, 1], [-1, -1], [1, -1] and [1, 1], because
                       * pieces here will not prevent Tam2HueAMaun1 from moving.
                       */
                      !((d[0] == -1 || d[0] == 1) && (d[1] == -1 || d[1] == 1))
                  );
                  let mut blocker = iter::apply_deltas(coord, blocker_deltas);
                  // if nothing is blocking the way
                  if blocker.all(|[i, j]| board[i][j].is_none()) {
                    inf.append(&mut vec::apply_deltas(coord, &[*delta]));
                  }
                }
                MovablePositions  {
                  finite: vec![],
                  infinite: inf
                }
              }
              Profession::Kua2 => // Clerk, 筆, kua
              MovablePositions  {
               finite: vec![],
               infinite: vec::apply_deltas_if_no_intervention(
                 coord,
                 &[&UP[..], &DOWN[..], &LEFT_RIGHT[..]].concat(),
                 board
               )
             },
           Profession::Tuk2 => // Shaman, 巫, terlsk
              MovablePositions {
               finite: vec![],
               infinite: apply_deltas_if_zero_or_one_intervention(
                 coord,
                 &[
                   &UP[..],
                   &DOWN[..],
                   &LEFT_RIGHT[..],
                   &DIAGONAL[..]
                 ].concat(),
                 board
               )
             },
         }
    } else {
        match piece_prof {
            Profession::Io => MovablePositions {
                finite: vec::eight_neighborhood(coord),
                infinite: vec![],
            },
            Profession::Kauk2 => MovablePositions {
                finite: vec::apply_deltas(
                    coord,
                    &[if side == Side::Upward {
                        [-1, 0]
                    } else {
                        [1, 0]
                    }],
                ),
                infinite: vec![],
            }, // Pawn, 兵, elmer
            Profession::Kaun1 => MovablePositions {
                finite: vec::apply_deltas(coord, &[[-2, 0], [2, 0], [0, -2], [0, 2]]),
                infinite: vec![],
            }, // 車, vadyrd

            Profession::Dau2 =>
            // Tiger, 虎, stistyst
            {
                MovablePositions {
                    finite: vec::apply_deltas(coord, &[[-1, -1], [-1, 1], [1, -1], [1, 1]]),
                    infinite: vec![],
                }
            }

            Profession::Maun1 =>
            // Horse, 馬, dodor
            {
                MovablePositions {
                    finite: vec::apply_deltas(coord, &[[-2, -2], [-2, 2], [2, 2], [2, -2]]),
                    infinite: vec![],
                }
            }
            Profession::Nuak1 =>
            // Vessel, 船, felkana
            {
                MovablePositions {
                    finite: vec![],
                    infinite: vec::apply_deltas_if_no_intervention(
                        coord,
                        if side == Side::Upward { &UP } else { &DOWN },
                        board,
                    ),
                }
            }
            Profession::Gua2 =>
            // Rook, 弓, gustuer
            {
                MovablePositions {
                    finite: vec![],
                    infinite: vec::apply_deltas_if_no_intervention(
                        coord,
                        &[&UP[..], &DOWN[..], &LEFT_RIGHT[..]].concat(),
                        board,
                    ),
                }
            }
            Profession::Kua2 =>
            // Clerk, 筆, kua
            {
                MovablePositions {
                    finite: vec::apply_deltas(coord, &[[0, -1], [0, 1]]),
                    infinite: vec::apply_deltas_if_no_intervention(
                        coord,
                        &[&UP[..], &DOWN[..]].concat(),
                        board,
                    ),
                }
            }

            Profession::Tuk2 =>
            // Shaman, 巫, terlsk
            {
                MovablePositions {
                    finite: vec::apply_deltas(coord, &[[-1, 0], [1, 0]]),
                    infinite: vec::apply_deltas_if_no_intervention(coord, &LEFT_RIGHT, board),
                }
            }

            Profession::Uai1 =>
            // General, 将, varxle
            {
                MovablePositions {
                    finite: vec::apply_deltas(
                        coord,
                        &[
                            [-1, -1],
                            if side == Side::Upward {
                                [-1, 0]
                            } else {
                                [1, 0]
                            },
                            [-1, 1],
                            [0, -1],
                            [0, 1],
                            [1, -1],
                            [1, 1],
                        ],
                    ),
                    infinite: vec![],
                }
            }
        }
    }
}
