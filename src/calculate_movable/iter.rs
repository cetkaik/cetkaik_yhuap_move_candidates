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

pub fn apply_single_delta_if_zero_or_one_intervention(
    coord: Coord,
    delta: [i32; 2],
    board: Board,
) -> impl Iterator<Item = Coord> {
    let blocker = apply_deltas(coord, crate::get_blocker_deltas::ultrafast(delta));

    // if no piece or a single piece is blocking the way
    apply_deltas(
        coord,
        if blocker.filter(|[i, j]| board[*i][*j].is_some()).count() <= 1 {
            Some(delta)
        } else {
            None
        }
        .into_iter(),
    )
}

pub fn apply_deltas_if_zero_or_one_intervention(
    coord: Coord,
    deltas: &[[i32; 2]],
    board: Board,
) -> impl Iterator<Item = Coord> + '_ {
    deltas
        .iter()
        .copied()
        .flat_map(move |delta| apply_single_delta_if_zero_or_one_intervention(coord, delta, board))
}
