use cetkaik_traits::{CetkaikRepresentation, IsBoard};

use super::{iter, Vec};
pub fn eight_neighborhood<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
) -> Vec<T::RelativeCoord> {
    apply_deltas::<T>(
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

pub fn apply_deltas<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    deltas: &[[isize; 2]],
) -> Vec<T::RelativeCoord> {
    deltas
        .iter()
        .filter_map(move |[delta_x, delta_y]| T::add_delta(coord, *delta_x, *delta_y))
        .collect()
}

pub fn apply_single_delta_if_no_intervention<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    delta: [isize; 2],
    board: T::RelativeBoard,
) -> Vec<T::RelativeCoord> {
    let mut blocker = iter::apply_deltas::<T>(coord, crate::get_blocker_deltas::ultrafast(delta));

    // if nothing is blocking the way
    if blocker.all(|block| board.peek(block).is_none()) {
        apply_deltas::<T>(coord, &[delta])
    } else {
        vec![]
    }
}

pub fn apply_deltas_if_no_intervention<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    deltas: &[[isize; 2]],
    board: T::RelativeBoard,
) -> Vec<T::RelativeCoord> {
    iter::apply_deltas_if_no_intervention::<T>(coord, deltas, board).collect()
}

pub fn apply_deltas_if_zero_or_one_intervention<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    deltas: &[[isize; 2]],
    board: T::RelativeBoard,
) -> Vec<T::RelativeCoord> {
    iter::apply_deltas_if_zero_or_one_intervention::<T>(coord, deltas, board).collect()
}
