use super::{
    rotate_board, rotate_coord, Board, Color, Coord, MovablePositions, NonTam2PieceUpward, Piece,
    Profession, Side,
};

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
    eight_neighborhood(coord)
        .iter()
        .any(|[i, j]| board[*i][*j] == Some(Piece::Tam2))
}

fn apply_deltas(coord: Coord, deltas: &[[i32; 2]]) -> Vec<Coord> {
    let [i, j] = coord;
    deltas
        .iter()
        .map(|[delta_x, delta_y]| [i as i32 + delta_x, j as i32 + delta_y])
        .filter_map(|[l, m]| {
            if 0 <= l && l <= 8 && 0 <= m && m <= 8 {
                Some([l as usize, m as usize])
            } else {
                None
            }
        })
        .collect()
}

fn get_blocker_deltas(delta: [i32; 2]) -> Vec<[i32; 2]> {
    /* blocking occurs only when there exists [dx_block, dy_block] such that
    - the dot product with [dx, dy] is positive
    - the cross product with [dx, dy] is zero
    - abs(dx_block, dy_block) < abs(dx, dy)
    */
    let [dx, dy] = delta;

    let mut ans: Vec<[i32; 2]> = vec![];

    for dx_block in -8..=8 {
        for dy_block in -8..=8 {
            if dx * dy_block - dy * dx_block != 0 {
                continue;
            } // cross product must be zero
            if dx * dx_block + dy * dy_block <= 0 {
                continue;
            } // cross product must be positive
            if dx_block * dx_block + dy_block * dy_block >= dx * dx + dy * dy {
                continue;
            }
            // must be strictly small in absolute value

            ans.push([dx_block, dy_block]);
        }
    }
    ans
}

fn apply_single_delta_if_no_intervention(
    coord: Coord,
    delta: [i32; 2],
    board: Board,
) -> Vec<Coord> {
    let blocker: Vec<Coord> = apply_deltas(coord, &get_blocker_deltas(delta));

    // if nothing is blocking the way
    if blocker.iter().all(|[i, j]| board[*i][*j] == None) {
        apply_deltas(coord, &[delta])
    } else {
        vec![]
    }
}

fn apply_single_delta_if_zero_or_one_intervention(
    coord: Coord,
    delta: [i32; 2],
    board: Board,
) -> Vec<Coord> {
    let blocker: Vec<Coord> = apply_deltas(coord, &get_blocker_deltas(delta));

    // if no piece or a single piece is blocking the way
    if blocker
        .iter()
        .filter(|[i, j]| board[*i][*j] != None)
        .count()
        <= 1
    {
        apply_deltas(coord, &[delta])
    } else {
        vec![]
    }
}

fn apply_deltas_if_no_intervention(coord: Coord, deltas: &[[i32; 2]], board: Board) -> Vec<Coord> {
    let mut ans = vec![];
    for delta in deltas {
        ans.append(&mut apply_single_delta_if_no_intervention(
            coord, *delta, board,
        ))
    }
    ans
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
        ))
    }
    ans
}

/// Returns the list of all possible locations that a piece can move to / step on.
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cetkaik_core::*;
/// assert_eq!(
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
///     ),
///     MovablePositions {
///         /* then the opponent's Gua2 can either move one step to the side, */
///         finite: vec![[2, 1]],
///         infinite: vec![[3, 0], [4, 0], [5, 0], [6, 0], [1, 0], [0, 0]]
///         /* or it can run to anywhere from [0,0] to [6,0].
///          * Note that you need two calls to this function in order to handle stepping. */
///     }
/// );
/// ```
pub fn calculate_movable_positions_for_either_side(
    coord: Coord,
    piece: Piece,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> MovablePositions {
    match piece {
        Piece::Tam2 => {
            calculate_movable_positions(coord, TamOrUpwardPiece::Tam2, board, tam_itself_is_tam_hue)
        }
        Piece::NonTam2Piece {
            prof,
            color,
            side: Side::Upward,
        } => calculate_movable_positions(
            coord,
            TamOrUpwardPiece::NonTam2Piece { prof, color },
            board,
            tam_itself_is_tam_hue,
        ),
        Piece::NonTam2Piece {
            prof,
            color,
            side: Side::Downward,
        } => {
            let MovablePositions { finite, infinite } = calculate_movable_positions(
                rotate_coord(coord),
                TamOrUpwardPiece::NonTam2Piece { prof, color },
                rotate_board(board),
                tam_itself_is_tam_hue,
            );

            MovablePositions {
                finite: finite.into_iter().map(rotate_coord).collect(),
                infinite: infinite.into_iter().map(rotate_coord).collect(),
            }
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

pub fn calculate_movable_positions(
    coord: Coord,
    piece: TamOrUpwardPiece,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> MovablePositions {
    const UP_LEFT: [[i32; 2]; 8] = [
        [-8, -8],
        [-7, -7],
        [-6, -6],
        [-5, -5],
        [-4, -4],
        [-3, -3],
        [-2, -2],
        [-1, -1],
    ];
    const UP_RIGHT: [[i32; 2]; 8] = [
        [-8, 8],
        [-7, 7],
        [-6, 6],
        [-5, 5],
        [-4, 4],
        [-3, 3],
        [-2, 2],
        [-1, 1],
    ];
    const DOWN_LEFT: [[i32; 2]; 8] = [
        [8, -8],
        [7, -7],
        [6, -6],
        [5, -5],
        [4, -4],
        [3, -3],
        [2, -2],
        [1, -1],
    ];
    const DOWN_RIGHT: [[i32; 2]; 8] = [
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
    const LEFT: [[i32; 2]; 8] = [
        [0, -1],
        [0, -2],
        [0, -3],
        [0, -4],
        [0, -5],
        [0, -6],
        [0, -7],
        [0, -8],
    ];
    const RIGHT: [[i32; 2]; 8] = [
        [0, 1],
        [0, 2],
        [0, 3],
        [0, 4],
        [0, 5],
        [0, 6],
        [0, 7],
        [0, 8],
    ];

    let piece_prof = match piece {
        TamOrUpwardPiece::Tam2 => {
            return MovablePositions {
                finite: eight_neighborhood(coord),
                infinite: vec![],
            }
        }
        TamOrUpwardPiece::NonTam2Piece { prof, color: _ } => prof,
    };

    if piece_prof == Profession::Io {
        return MovablePositions {
            finite: eight_neighborhood(coord),
            infinite: vec![],
        };
    }

    if is_tam_hue(coord, board, tam_itself_is_tam_hue) {
        match piece_prof {
           Profession::Uai1 => // General, 将, varxle
            MovablePositions { finite: eight_neighborhood(coord), infinite: vec![] },
           Profession::Kaun1 =>
            MovablePositions {
              finite: apply_deltas(coord, &[
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
                &apply_deltas(coord, &[
                  [-1, 0],
                  [0, -1],
                  [0, 1],
                  [1, 0]
                ])[..],
                &apply_single_delta_if_no_intervention(coord, [-2, 0], board)[..]
              ].concat(),
              infinite: vec![]
            },
          Profession::Nuak1 => // Vessel, 船, felkana
            MovablePositions  {
              finite: [
                &apply_deltas(coord, &[
                  [0, -1],
                  [0, 1]
                ])[..],
                &apply_deltas_if_no_intervention(
                  coord,
                  &[
                    [0, -2],
                    [0, 2]
                  ],
                  board
                )[..]
              ].concat(),
              infinite: apply_deltas_if_no_intervention(coord, &[&UP[..], &DOWN[..]].concat(), board)
            },
          Profession::Gua2 | // Rook, 弓, gustuer
          Profession::Dau2 => // Tiger, 虎, stistyst
             MovablePositions  {
              finite: vec![],
              infinite: apply_deltas_if_no_intervention(
                  coord,
                  &[&UP_LEFT[..], &UP_RIGHT[..], &DOWN_LEFT[..], &DOWN_RIGHT[..]].concat(),
                  board
              )
            },
          Profession::Maun1 => {
            // Horse, 馬, dodor
            const DELTAS: [[i32; 2] ; 28] = [
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
            for delta in &DELTAS {
              let blocker_deltas: Vec<[i32; 2]> = get_blocker_deltas(*delta).into_iter().filter(
                |d|
                  /*
                   * remove [-1, 1], [-1, -1], [1, -1] and [1, 1], because
                   * pieces here will not prevent Tam2HueAMaun1 from moving.
                   */
                  !((d[0] == -1 || d[0] == 1) && (d[1] == -1 || d[1] == 1))
              ).collect();
              let blocker: Vec<Coord> = apply_deltas(coord, &blocker_deltas);
              // if nothing is blocking the way
              if blocker.iter().all(|[i, j]| board[*i][*j] == None) {
                inf.append(&mut apply_deltas(coord, &[*delta]));
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
              infinite: apply_deltas_if_no_intervention(
                coord,
                &[&UP[..], &DOWN[..], &LEFT[..], &RIGHT[..]].concat(),
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
                  &LEFT[..],
                  &RIGHT[..],
                  &UP_LEFT[..],
                  &UP_RIGHT[..],
                  &DOWN_LEFT[..],
                  &DOWN_RIGHT[..]
                ].concat(),
                board
              )
            },
          _ =>unreachable!()
        }
    } else {
        match piece_prof {
            Profession::Kauk2 => MovablePositions {
                finite: apply_deltas(coord, &[[-1, 0]]),
                infinite: vec![],
            }, // Pawn, 兵, elmer

            Profession::Kaun1 => MovablePositions {
                finite: apply_deltas(coord, &[[-2, 0], [2, 0], [0, -2], [0, 2]]),
                infinite: vec![],
            }, // 車, vadyrd

            Profession::Dau2 =>
            // Tiger, 虎, stistyst
            {
                MovablePositions {
                    finite: apply_deltas(coord, &[[-1, -1], [-1, 1], [1, -1], [1, 1]]),
                    infinite: vec![],
                }
            }

            Profession::Maun1 =>
            // Horse, 馬, dodor
            {
                MovablePositions {
                    finite: apply_deltas(coord, &[[-2, -2], [-2, 2], [2, 2], [2, -2]]),
                    infinite: vec![],
                }
            }

            Profession::Nuak1 =>
            // Vessel, 船, felkana
            {
                MovablePositions {
                    finite: vec![],
                    infinite: apply_deltas_if_no_intervention(coord, &UP, board),
                }
            }

            Profession::Gua2 =>
            // Rook, 弓, gustuer
            {
                MovablePositions {
                    finite: vec![],
                    infinite: apply_deltas_if_no_intervention(
                        coord,
                        &[&UP[..], &DOWN[..], &LEFT[..], &RIGHT[..]].concat(),
                        board,
                    ),
                }
            }

            Profession::Kua2 =>
            // Clerk, 筆, kua
            {
                MovablePositions {
                    finite: apply_deltas(coord, &[[0, -1], [0, 1]]),
                    infinite: apply_deltas_if_no_intervention(
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
                    finite: apply_deltas(coord, &[[-1, 0], [1, 0]]),
                    infinite: apply_deltas_if_no_intervention(
                        coord,
                        &[&LEFT[..], &RIGHT[..]].concat(),
                        board,
                    ),
                }
            }

            Profession::Uai1 =>
            // General, 将, varxle
            {
                MovablePositions {
                    finite: apply_deltas(
                        coord,
                        &[[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 1]],
                    ),
                    infinite: vec![],
                }
            }

            _ => unreachable!(),
        }
    }
}
