use alloc::vec::Vec;
use num::signum;

/// Lists the coordinates `[dx_block, dy_block]` that can block an attempt for a piece to move to `[dx, dy]`
/// ```
/// use cetkaik_yhuap_move_candidates::get_blocker_deltas::naive;
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
/// assert_eq_ignoring_order(&naive([6,6]), &vec![[1,1], [2,2], [3,3], [4,4], [5,5]]);
/// assert_eq_ignoring_order(&naive([-6,6]), &vec![[-1,1], [-2,2], [-3,3], [-4,4], [-5,5]]);
/// assert_eq_ignoring_order(&naive([-5,0]), &vec![[-1,0], [-2,0], [-3,0], [-4,0]]);
/// assert_eq_ignoring_order(&naive([0,5]), &vec![[0,1], [0,2], [0,3], [0,4]]);
/// assert_eq_ignoring_order(&naive([0,0]), &vec![]);
///
/// // Knight's move is never used in cetkaik, but this function works for those cases too
/// assert_eq_ignoring_order(&naive([-6,3]), &vec![[-2,1], [-4,2]]);
/// assert_eq_ignoring_order(&naive([-9,6]), &vec![[-3,2], [-6,4]]);
/// ```
#[must_use]
pub fn naive(delta: [isize; 2]) -> Vec<[isize; 2]> {
    /*
    We list the coordinates [dx_block, dy_block] that can block an attempt for a piece to move to [dx, dy].
    The criteria required for [dx_block, dy_block] to block the move are
    - the dot product with [dx, dy] is positive
    - the cross product with [dx, dy] is zero
    - abs(dx_block, dy_block) < abs(dx, dy)
    */
    let [dx, dy] = delta;

    let mut ans: Vec<[isize; 2]> = vec![];

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

/// Lists the coordinates `[dx_block, dy_block]` that can block an attempt for a piece to move to `[dx, dy]`
/// It is assumed that the absolute values of the components of `delta` never exceed 8.
/// ```
/// use cetkaik_yhuap_move_candidates::get_blocker_deltas::fast;
/// use std::collections::HashSet;
///
/// fn assert_eq_ignoring_order<T>(a: &[T], b: &[T])
/// where
///     T: Eq + core::hash::Hash + std::fmt::Debug,
/// {
///     let a: HashSet<_> = a.iter().collect();
///     let b: HashSet<_> = b.iter().collect();
///
///     assert_eq!(a, b);
/// }
/// assert_eq_ignoring_order(&fast([6,6]), &vec![[1,1], [2,2], [3,3], [4,4], [5,5]]);
/// assert_eq_ignoring_order(&fast([-6,6]), &vec![[-1,1], [-2,2], [-3,3], [-4,4], [-5,5]]);
/// assert_eq_ignoring_order(&fast([-5,0]), &vec![[-1,0], [-2,0], [-3,0], [-4,0]]);
/// assert_eq_ignoring_order(&fast([0,5]), &vec![[0,1], [0,2], [0,3], [0,4]]);
/// assert_eq_ignoring_order(&fast([0,0]), &vec![]);
///
/// // Knight's move is never used in cetkaik, but this function works for those cases too
/// assert_eq_ignoring_order(&fast([-6,3]), &vec![[-2,1], [-4,2]]);
/// assert_eq_ignoring_order(&fast([-9,6]), &vec![[-3,2], [-6,4]]);
/// ```
#[must_use]
pub fn fast(delta: [isize; 2]) -> Vec<[isize; 2]> {
    if let [0, 0] = delta {
        return vec![];
    }
    /*
    We list the coordinates [dx_block, dy_block] that can block an attempt for a piece to move to [dx, dy].
    Let [qx, qy] = [dx, dy] / gcd(dx, dy), and then the criteria required for [dx_block, dy_block] to block the move are
    - [dx_block, dy_block] = mult * [qx, qy]
    - abs^2(dx_block, dy_block) < abs^2(dx, dy)
    */
    let [dx, dy] = delta;
    let d_length = dx * dx + dy * dy;
    let g = num::integer::gcd(dx, dy);
    let qx = dx / g;
    let qy = dy / g;

    let mut ans: Vec<[isize; 2]> = vec![];

    for mult in 1.. {
        let dx_block = mult * qx;
        let dy_block = mult * qy;
        let d_block_length = dx_block * dx_block + dy_block * dy_block;
        if core::cmp::max(dx_block.abs(), dy_block.abs()) > 8 || d_block_length >= d_length {
            break;
        }
        ans.push([dx_block, dy_block]);
    }

    ans
}

pub struct Blocker {
    d_length: isize,
    qx: isize,
    qy: isize,
    mult: isize,
}

impl Iterator for Blocker {
    type Item = [isize; 2];

    #[inline]
    fn next(&mut self) -> Option<[isize; 2]> {
        let dx_block = self.mult * self.qx;
        let dy_block = self.mult * self.qy;
        let d_block_length = dx_block * dx_block + dy_block * dy_block;
        if d_block_length >= self.d_length {
            return None;
        }
        self.mult += 1;

        Some([dx_block, dy_block])
    }
}

/// Lists the coordinates `[dx_block, dy_block]` that can block an attempt for a piece to move to `[dx, dy]`
/// It is assumed that
/// - the direction is either horizontal, vertical, or 45 degrees diagonal
/// - the input vector is not zero
/// ```
/// use cetkaik_yhuap_move_candidates::get_blocker_deltas::ultrafast;
/// use std::collections::HashSet;
///
/// fn assert_eq_ignoring_order<T>(a: &[T], b: &[T])
/// where
///     T: Eq + core::hash::Hash + std::fmt::Debug,
/// {
///     let a: HashSet<_> = a.iter().collect();
///     let b: HashSet<_> = b.iter().collect();
///
///     assert_eq!(a, b);
/// }
/// assert_eq_ignoring_order(&ultrafast([6,6]).collect::<Vec<_>>(), &vec![[1,1], [2,2], [3,3], [4,4], [5,5]]);
/// assert_eq_ignoring_order(&ultrafast([-6,6]).collect::<Vec<_>>(), &vec![[-1,1], [-2,2], [-3,3], [-4,4], [-5,5]]);
/// assert_eq_ignoring_order(&ultrafast([-5,0]).collect::<Vec<_>>(), &vec![[-1,0], [-2,0], [-3,0], [-4,0]]);
/// assert_eq_ignoring_order(&ultrafast([0,5]).collect::<Vec<_>>(), &vec![[0,1], [0,2], [0,3], [0,4]]);
/// ```
pub fn ultrafast(delta: [isize; 2]) -> impl Iterator<Item = [isize; 2]> {
    let [dx, dy] = delta;
    let d_length = dx * dx + dy * dy;
    let qx = signum(dx);
    let qy = signum(dy);

    Blocker {
        d_length,
        qx,
        qy,
        mult: 1,
    }
}

#[test]
fn naive_vs_fast() {
    use std::collections::HashSet;

    fn assert_eq_ignoring_order<T>(a: &[T], b: &[T])
    where
        T: Eq + core::hash::Hash + std::fmt::Debug,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        assert_eq!(a, b);
    }

    for dx in -8..=8 {
        for dy in -8..=8 {
            assert_eq_ignoring_order(&naive([dx, dy]), &fast([dx, dy]));
        }
    }
}

#[test]
fn naive_vs_ultrafast() {
    use std::collections::HashSet;

    fn assert_eq_ignoring_order<T>(a: &[T], b: &[T])
    where
        T: Eq + core::hash::Hash + std::fmt::Debug,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        assert_eq!(a, b);
    }

    for [dx, dy] in [
        [1, 0],
        [2, 0],
        [3, 0],
        [4, 0],
        [5, 0],
        [6, 0],
        [7, 0],
        [8, 0],
        [9, 0],
        [-1, 0],
        [-2, 0],
        [-3, 0],
        [-4, 0],
        [-5, 0],
        [-6, 0],
        [-7, 0],
        [-8, 0],
        [-9, 0],
        [0, 1],
        [0, 2],
        [0, 3],
        [0, 4],
        [0, 5],
        [0, 6],
        [0, 7],
        [0, 8],
        [0, 9],
        [0, -1],
        [0, -2],
        [0, -3],
        [0, -4],
        [0, -5],
        [0, -6],
        [0, -7],
        [0, -8],
        [0, -9],
        [1, 1],
        [2, 2],
        [3, 3],
        [4, 4],
        [5, 5],
        [6, 6],
        [7, 7],
        [8, 8],
        [9, 9],
        [-1, 1],
        [-2, 2],
        [-3, 3],
        [-4, 4],
        [-5, 5],
        [-6, 6],
        [-7, 7],
        [-8, 8],
        [-9, 9],
        [1, -1],
        [2, -2],
        [3, -3],
        [4, -4],
        [5, -5],
        [6, -6],
        [7, -7],
        [8, -8],
        [9, -9],
        [-1, -1],
        [-2, -2],
        [-3, -3],
        [-4, -4],
        [-5, -5],
        [-6, -6],
        [-7, -7],
        [-8, -8],
        [-9, -9],
    ] {
        assert_eq_ignoring_order(&naive([dx, dy]), &ultrafast([dx, dy]).collect::<Vec<_>>());
    }
}
