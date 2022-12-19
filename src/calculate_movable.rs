use alloc::vec::Vec;

use crate::{CetkaikCore, CetkaikRepresentation};

use super::{Board, Coord, MovablePositions, Piece, Profession};

pub mod iter;
pub mod vec;

pub fn is_tam_hue<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    board: T::RelativeBoard,
    tam_itself_is_tam_hue: bool,
) -> bool {
    if T::is_tam_hue_by_default(coord) {
        return true;
    }

    if tam_itself_is_tam_hue && T::relative_get(board, coord) == Some(T::tam2()) {
        return true;
    }

    // is Tam2 available at any neighborhood?
    iter::eight_neighborhood::<T>(coord)
        .any(|coord| T::relative_get(board, coord) == Some(T::tam2()))
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
) -> MovablePositions<Coord> {
    CetkaikCore::match_on_piece_and_apply(
        piece,
        &|| calculate_movable_positions_for_tam::<CetkaikCore>(coord),
        &|prof, side| {
            calculate_movable_positions_for_nontam::<CetkaikCore>(
                coord,
                prof,
                board,
                tam_itself_is_tam_hue,
                side,
            )
        },
    )
}

pub fn calculate_movable_positions_for_tam<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
) -> MovablePositions<T::RelativeCoord> {
    MovablePositions {
        finite: vec::eight_neighborhood::<T>(coord),
        infinite: vec![],
    }
}

pub fn calculate_movable_positions_for_nontam<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    prof: Profession,
    board: T::RelativeBoard,
    tam_itself_is_tam_hue: bool,
    side: T::RelativeSide,
) -> MovablePositions<T::RelativeCoord> {
    const DIAGONAL: [[isize; 2]; 32] = [
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
    const UP: [[isize; 2]; 8] = [
        [-1, 0],
        [-2, 0],
        [-3, 0],
        [-4, 0],
        [-5, 0],
        [-6, 0],
        [-7, 0],
        [-8, 0],
    ];
    const DOWN: [[isize; 2]; 8] = [
        [1, 0],
        [2, 0],
        [3, 0],
        [4, 0],
        [5, 0],
        [6, 0],
        [7, 0],
        [8, 0],
    ];
    const LEFT_RIGHT: [[isize; 2]; 16] = [
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
    if is_tam_hue::<T>(coord, board, tam_itself_is_tam_hue) {
        match piece_prof {
           Profession::Io | Profession::Uai1 => // General, 将, varxle
            MovablePositions { finite: vec::eight_neighborhood::<T>(coord), infinite: vec![] },
            Profession::Kaun1 =>
            MovablePositions {
              finite: vec::apply_deltas::<T>(coord, &[
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
                &vec::apply_deltas::<T>(coord, &[
                  [-1, 0],
                  [0, -1],
                  [0, 1],
                  [1, 0]
                ])[..],
                &vec::apply_single_delta_if_no_intervention::<T>(coord,  if T::is_upward(side) {[-2, 0]} else {[2,0]}, board)[..]
              ].concat(),
              infinite: vec![]
            },
            Profession::Nuak1 => // Vessel, 船, felkana
            MovablePositions  {
              finite: [
                &vec::apply_deltas::<T>(coord, &[
                  [0, -1],
                  [0, 1]
                ])[..],
                &vec::apply_deltas_if_no_intervention::<T>(
                  coord,
                  &[
                    [0, -2],
                    [0, 2]
                  ],
                  board
                )[..]
              ].concat(),
              infinite: vec::apply_deltas_if_no_intervention::<T>(coord, &[&UP[..], &DOWN[..]].concat(), board)
            },
            Profession::Gua2 | // Rook, 弓, gustuer
            Profession::Dau2 => // Tiger, 虎, stistyst
               MovablePositions {
                finite: vec![],
                infinite: vec::apply_deltas_if_no_intervention::<T>(
                    coord,
                    &DIAGONAL,
                    board
                )
              },
              Profession::Maun1 => {
                // Horse, 馬, dodor
                const HORSE_DELTAS: [[isize; 2] ; 28] = [
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
                let mut inf: Vec<T::RelativeCoord> = vec![];
                for delta in &HORSE_DELTAS {
                  let blocker_deltas = crate::get_blocker_deltas::ultrafast(*delta).filter(
                    |d|
                      /*
                       * remove [-1, 1], [-1, -1], [1, -1] and [1, 1], because
                       * pieces here will not prevent Tam2HueAMaun1 from moving.
                       */
                      !((d[0] == -1 || d[0] == 1) && (d[1] == -1 || d[1] == 1))
                  );
                  let mut blocker = iter::apply_deltas::<T>(coord, blocker_deltas);
                  // if nothing is blocking the way
                  if blocker.all(|block| T::relative_get(board, block).is_none()) {
                    inf.append(&mut vec::apply_deltas::<T>(coord, &[*delta]));
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
               infinite: vec::apply_deltas_if_no_intervention::<T>(
                 coord,
                 &[&UP[..], &DOWN[..], &LEFT_RIGHT[..]].concat(),
                 board
               )
             },
           Profession::Tuk2 => // Shaman, 巫, terlsk
              MovablePositions {
               finite: vec![],
               infinite: vec::apply_deltas_if_zero_or_one_intervention::<T>(
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
                finite: vec::eight_neighborhood::<T>(coord),
                infinite: vec![],
            },
            Profession::Kauk2 => MovablePositions {
                finite: vec::apply_deltas::<T>(
                    coord,
                    &[if T::is_upward(side) { [-1, 0] } else { [1, 0] }],
                ),
                infinite: vec![],
            }, // Pawn, 兵, elmer
            Profession::Kaun1 => MovablePositions {
                finite: vec::apply_deltas::<T>(coord, &[[-2, 0], [2, 0], [0, -2], [0, 2]]),
                infinite: vec![],
            }, // 車, vadyrd

            Profession::Dau2 =>
            // Tiger, 虎, stistyst
            {
                MovablePositions {
                    finite: vec::apply_deltas::<T>(coord, &[[-1, -1], [-1, 1], [1, -1], [1, 1]]),
                    infinite: vec![],
                }
            }

            Profession::Maun1 =>
            // Horse, 馬, dodor
            {
                MovablePositions {
                    finite: vec::apply_deltas::<T>(coord, &[[-2, -2], [-2, 2], [2, 2], [2, -2]]),
                    infinite: vec![],
                }
            }
            Profession::Nuak1 =>
            // Vessel, 船, felkana
            {
                MovablePositions {
                    finite: vec![],
                    infinite: vec::apply_deltas_if_no_intervention::<T>(
                        coord,
                        if T::is_upward(side) { &UP } else { &DOWN },
                        board,
                    ),
                }
            }
            Profession::Gua2 =>
            // Rook, 弓, gustuer
            {
                MovablePositions {
                    finite: vec![],
                    infinite: vec::apply_deltas_if_no_intervention::<T>(
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
                    finite: vec::apply_deltas::<T>(coord, &[[0, -1], [0, 1]]),
                    infinite: vec::apply_deltas_if_no_intervention::<T>(
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
                    finite: vec::apply_deltas::<T>(coord, &[[-1, 0], [1, 0]]),
                    infinite: vec::apply_deltas_if_no_intervention::<T>(coord, &LEFT_RIGHT, board),
                }
            }

            Profession::Uai1 =>
            // General, 将, varxle
            {
                MovablePositions {
                    finite: vec::apply_deltas::<T>(
                        coord,
                        &[
                            [-1, -1],
                            if T::is_upward(side) { [-1, 0] } else { [1, 0] },
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
