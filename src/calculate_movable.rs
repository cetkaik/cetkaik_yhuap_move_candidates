use super::*;

pub fn eightNeighborhood(coord: Coord) -> Vec<Coord> {
    applyDeltas(
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

pub fn isTamHue(coord: Coord, board: Board, tam_itself_is_tam_hue: bool) -> bool {
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
    return eightNeighborhood(coord)
        .iter()
        .any(|[i, j]| board[*i][*j] == Some(Piece::Tam2));
}

fn applyDeltas(coord: Coord, deltas: &[[i32; 2]]) -> Vec<Coord> {
    let [i, j] = coord;
    return deltas
        .iter()
        .map(|[delta_x, delta_y]| [i as i32 + delta_x, j as i32 + delta_y])
        .filter(|[l, m]| 0 <= *l && *l <= 8 && 0 <= *m && *m <= 8)
        .map(|[l, m]| [l as usize, m as usize])
        .collect();
}

fn getBlockerDeltas(delta: [i32; 2]) -> Vec<[i32; 2]> {
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
    return ans;
}

fn applySingleDeltaIfNoIntervention(coord: Coord, delta: [i32; 2], board: Board) -> Vec<Coord> {
    let blocker: Vec<Coord> = applyDeltas(coord, &getBlockerDeltas(delta));

    // if nothing is blocking the way
    if blocker.iter().all(|[i, j]| board[*i][*j] == None) {
        return applyDeltas(coord, &[delta]);
    } else {
        return vec![];
    }
}

fn applySingleDeltaIfZeroOrOneIntervention(
    coord: Coord,
    delta: [i32; 2],
    board: Board,
) -> Vec<Coord> {
    let blocker: Vec<Coord> = applyDeltas(coord, &getBlockerDeltas(delta));

    // if no piece or a single piece is blocking the way
    if blocker
        .iter()
        .filter(|[i, j]| board[*i][*j] != None)
        .collect::<Vec<_>>()
        .len()
        <= 1
    {
        return applyDeltas(coord, &[delta]);
    } else {
        return vec![];
    }
}

fn applyDeltasIfNoIntervention(coord: Coord, deltas: &[[i32; 2]], board: Board) -> Vec<Coord> {
    let mut ans = vec![];
    for delta in deltas {
        ans.append(&mut applySingleDeltaIfNoIntervention(coord, *delta, board))
    }
    ans
}

fn applyDeltasIfZeroOrOneIntervention(
    coord: Coord,
    deltas: &[[i32; 2]],
    board: Board,
) -> Vec<Coord> {
    let mut ans = vec![];
    for delta in deltas {
        ans.append(&mut applySingleDeltaIfZeroOrOneIntervention(
            coord, *delta, board,
        ))
    }
    ans
}

pub fn calculateMovablePositions(
    coord: Coord,
    piece: Piece,
    board: Board,
    tam_itself_is_tam_hue: bool,
) -> MovablePositions {
    const UPLEFT: [[i32; 2]; 8] = [
        [-8, -8],
        [-7, -7],
        [-6, -6],
        [-5, -5],
        [-4, -4],
        [-3, -3],
        [-2, -2],
        [-1, -1],
    ];
    const UPRIGHT: [[i32; 2]; 8] = [
        [-8, 8],
        [-7, 7],
        [-6, 6],
        [-5, 5],
        [-4, 4],
        [-3, 3],
        [-2, 2],
        [-1, 1],
    ];
    const DOWNLEFT: [[i32; 2]; 8] = [
        [8, -8],
        [7, -7],
        [6, -6],
        [5, -5],
        [4, -4],
        [3, -3],
        [2, -2],
        [1, -1],
    ];
    const DOWNRIGHT: [[i32; 2]; 8] = [
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

    let (piece_prof, piece_color, piece_side) = match piece {
        Piece::Tam2 => {
            return MovablePositions {
                finite: eightNeighborhood(coord),
                infinite: vec![],
            }
        }
        Piece::NonTam2Piece { prof, color, side } => (prof, color, side),
    };

    if piece_prof == Profession::Io {
        return MovablePositions {
            finite: eightNeighborhood(coord),
            infinite: vec![],
        };
    }

    if isTamHue(coord, board, tam_itself_is_tam_hue) {
        match piece_prof {
           Profession::Uai1 => // General, 将, varxle
            return MovablePositions { finite: eightNeighborhood(coord), infinite: vec![] },
           Profession::Kaun1 =>
            return MovablePositions {
              finite: applyDeltas(coord, &[
                [-2, -2],
                [-2, 2],
                [2, 2],
                [2, -2]
              ]),
              infinite: vec![]
            }, // 車, vadyrd
          Profession::Kauk2 => // Pawn, 兵, elmer
            return MovablePositions  {
              finite: [
                &applyDeltas(coord, &[
                  [-1, 0],
                  [0, -1],
                  [0, 1],
                  [1, 0]
                ])[..],
                &applySingleDeltaIfNoIntervention(coord, [-2, 0], board)[..]
              ].concat(),
              infinite: vec![]
            },
          Profession::Nuak1 => // Vessel, 船, felkana
            return MovablePositions  {
              finite: [
                &applyDeltas(coord, &[
                  [0, -1],
                  [0, 1]
                ])[..],
                &applyDeltasIfNoIntervention(
                  coord,
                  &[
                    [0, -2],
                    [0, 2]
                  ],
                  board
                )[..]
              ].concat(),
              infinite: applyDeltasIfNoIntervention(coord, &[&UP[..], &DOWN[..]].concat(), board)
            },
          Profession::Gua2 | // Rook, 弓, gustuer
          Profession::Dau2 => // Tiger, 虎, stistyst
            return MovablePositions  {
              finite: vec![],
              infinite: applyDeltasIfNoIntervention(
                coord,
                &[&UPLEFT[..], &UPRIGHT[..], &DOWNLEFT[..], &DOWNRIGHT[..]].concat(),
                board
              )
            },
          Profession::Maun1 => {
            // Horse, 馬, dodor
            const deltas: [[i32; 2] ; 28] = [
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
            for delta in &deltas {
              let blocker_deltas: Vec<[i32; 2]> = getBlockerDeltas(*delta).into_iter().filter(
                |d|
                  /*
                   * remove [-1, 1], [-1, -1], [1, -1] and [1, 1], because
                   * pieces here will not prevent Tam2HueAMaun1 from moving.
                   */
                  !((d[0] == -1 || d[0] == 1) && (d[1] == -1 || d[1] == 1))
              ).collect();
              let blocker: Vec<Coord> = applyDeltas(coord, &blocker_deltas);
              // if nothing is blocking the way
              if blocker.iter().all(|[i, j]| board[*i][*j] == None) {
                inf.append(&mut applyDeltas(coord, &[*delta]));
              }
            }
            return MovablePositions  {
              finite: vec![],
              infinite: inf
            };
          }
          Profession::Kua2 => // Clerk, 筆, kua
            return MovablePositions  {
              finite: vec![],
              infinite: applyDeltasIfNoIntervention(
                coord,
                &[&UP[..], &DOWN[..], &LEFT[..], &RIGHT[..]].concat(),
                board
              )
            },
          Profession::Tuk2 => // Shaman, 巫, terlsk
            return MovablePositions {
              finite: vec![],
              infinite: applyDeltasIfZeroOrOneIntervention(
                coord,
                &[
                  &UP[..],
                  &DOWN[..],
                  &LEFT[..],
                  &RIGHT[..],
                  &UPLEFT[..],
                  &UPRIGHT[..],
                  &DOWNLEFT[..],
                  &DOWNRIGHT[..]
                ].concat(),
                board
              )
            },
          _ =>unreachable!()
        }
    } else {
        match piece_prof {
            Profession::Kauk2 => {
                return MovablePositions {
                    finite: applyDeltas(coord, &[[-1, 0]]),
                    infinite: vec![],
                }
            } // Pawn, 兵, elmer

            Profession::Kaun1 => {
                return MovablePositions {
                    finite: applyDeltas(coord, &[[-2, 0], [2, 0], [0, -2], [0, 2]]),
                    infinite: vec![],
                }
            } // 車, vadyrd

            Profession::Dau2 =>
            // Tiger, 虎, stistyst
            {
                return MovablePositions {
                    finite: applyDeltas(coord, &[[-1, -1], [-1, 1], [1, -1], [1, 1]]),
                    infinite: vec![],
                }
            }

            Profession::Maun1 =>
            // Horse, 馬, dodor
            {
                return MovablePositions {
                    finite: applyDeltas(coord, &[[-2, -2], [-2, 2], [2, 2], [2, -2]]),
                    infinite: vec![],
                }
            }

            Profession::Nuak1 =>
            // Vessel, 船, felkana
            {
                return MovablePositions {
                    finite: vec![],
                    infinite: applyDeltasIfNoIntervention(coord, &UP, board),
                }
            }

            Profession::Gua2 =>
            // Rook, 弓, gustuer
            {
                return MovablePositions {
                    finite: vec![],
                    infinite: applyDeltasIfNoIntervention(
                        coord,
                        &[&UP[..], &DOWN[..], &LEFT[..], &RIGHT[..]].concat(),
                        board,
                    ),
                }
            }

            Profession::Kua2 =>
            // Clerk, 筆, kua
            {
                return MovablePositions {
                    finite: applyDeltas(coord, &[[0, -1], [0, 1]]),
                    infinite: applyDeltasIfNoIntervention(
                        coord,
                        &[&UP[..], &DOWN[..]].concat(),
                        board,
                    ),
                }
            }

            Profession::Tuk2 =>
            // Shaman, 巫, terlsk
            {
                return MovablePositions {
                    finite: applyDeltas(coord, &[[-1, 0], [1, 0]]),
                    infinite: applyDeltasIfNoIntervention(
                        coord,
                        &[&LEFT[..], &RIGHT[..]].concat(),
                        board,
                    ),
                }
            }

            Profession::Uai1 =>
            // General, 将, varxle
            {
                return MovablePositions {
                    finite: applyDeltas(
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
