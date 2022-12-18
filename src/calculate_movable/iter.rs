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
    deltas: impl Iterator<Item = [isize; 2]>,
) -> impl Iterator<Item = Coord> {
    deltas.filter_map(move |[delta_x, delta_y]| crate::add_delta(coord, delta_x, delta_y))
}

pub fn apply_single_delta_if_no_intervention(
    coord: Coord,
    delta: [isize; 2],
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
    deltas: &[[isize; 2]],
    board: Board,
) -> impl Iterator<Item = Coord> + '_ {
    deltas
        .iter()
        .copied()
        .flat_map(move |delta| apply_single_delta_if_no_intervention(coord, delta, board))
}

pub fn apply_single_delta_if_zero_or_one_intervention(
    coord: Coord,
    delta: [isize; 2],
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
    deltas: &[[isize; 2]],
    board: Board,
) -> impl Iterator<Item = Coord> + '_ {
    deltas
        .iter()
        .copied()
        .flat_map(move |delta| apply_single_delta_if_zero_or_one_intervention(coord, delta, board))
}
