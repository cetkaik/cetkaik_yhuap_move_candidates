use cetkaik_traits::{CetkaikRepresentation, IsBoard};

pub fn eight_neighborhood<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
) -> impl Iterator<Item = T::RelativeCoord> {
    apply_deltas::<T>(
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
pub fn apply_deltas<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    deltas: impl Iterator<Item = [isize; 2]>,
) -> impl Iterator<Item = T::RelativeCoord> {
    deltas.filter_map(move |[delta_x, delta_y]| T::add_delta(coord, delta_x, delta_y))
}

pub fn apply_single_delta_if_no_intervention<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    delta: [isize; 2],
    board: T::RelativeBoard,
) -> impl Iterator<Item = T::RelativeCoord> {
    let mut blocker = apply_deltas::<T>(coord, crate::get_blocker_deltas::ultrafast(delta));

    // if nothing is blocking the way
    apply_deltas::<T>(
        coord,
        if blocker.all(|coord| board.peek(coord).is_none()) {
            Some(delta)
        } else {
            None
        }
        .into_iter(),
    )
}

pub fn apply_deltas_if_no_intervention<'a, T: CetkaikRepresentation + 'a>(
    coord: T::RelativeCoord,
    deltas: &'a [[isize; 2]],
    board: T::RelativeBoard,
) -> impl Iterator<Item = T::RelativeCoord> + '_
where
    <T as CetkaikRepresentation>::RelativeBoard: 'a,
    <T as CetkaikRepresentation>::RelativeCoord: 'a,
{
    deltas
        .iter()
        .copied()
        .flat_map(move |delta| apply_single_delta_if_no_intervention::<T>(coord, delta, board))
}

pub fn apply_single_delta_if_zero_or_one_intervention<T: CetkaikRepresentation>(
    coord: T::RelativeCoord,
    delta: [isize; 2],
    board: T::RelativeBoard,
) -> impl Iterator<Item = T::RelativeCoord> {
    let blocker = apply_deltas::<T>(coord, crate::get_blocker_deltas::ultrafast(delta));

    // if no piece or a single piece is blocking the way
    apply_deltas::<T>(
        coord,
        if blocker.filter(|block| board.peek(*block).is_some()).count() <= 1 {
            Some(delta)
        } else {
            None
        }
        .into_iter(),
    )
}

pub fn apply_deltas_if_zero_or_one_intervention<'a, T: CetkaikRepresentation + 'a>(
    coord: T::RelativeCoord,
    deltas: &'a [[isize; 2]],
    board: T::RelativeBoard,
) -> impl Iterator<Item = T::RelativeCoord> + '_
where
    <T as CetkaikRepresentation>::RelativeBoard: 'a,
    <T as CetkaikRepresentation>::RelativeCoord: 'a,
{
    deltas.iter().copied().flat_map(move |delta| {
        apply_single_delta_if_zero_or_one_intervention::<T>(coord, delta, board)
    })
}
