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

pub fn apply_deltas_if_zero_or_one_intervention(
    coord: Coord,
    deltas: &[[i32; 2]],
    board: Board,
) -> Vec<Coord> {
    let mut ans = vec![];
    for delta in deltas {
        ans.extend(&mut iter::apply_single_delta_if_zero_or_one_intervention(
            coord, *delta, board,
        ));
    }
    ans
}