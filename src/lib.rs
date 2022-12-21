#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::too_many_lines,
    clippy::non_ascii_literal,
    clippy::module_name_repetitions,
    clippy::use_self
)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use cetkaik_core::{IsAbsoluteBoard, IsField, PureMove_};

pub use cetkaik_core::absolute;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct CetkaikCore;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct CetkaikCompact;

pub trait CetkaikRepresentation {
    type Perspective: Copy + Eq;

    type AbsoluteCoord: Copy + Eq + core::fmt::Debug;
    type RelativeCoord: Copy + Eq;

    type AbsoluteBoard: Clone
        + core::fmt::Debug
        + IsAbsoluteBoard<PieceWithSide = Self::AbsolutePiece, Coord = Self::AbsoluteCoord>;
    type RelativeBoard: Copy;

    type AbsolutePiece: Copy + Eq;
    type RelativePiece: Copy + Eq;

    type AbsoluteField: Clone
        + core::fmt::Debug
        + IsField<
            PieceWithSide = Self::AbsolutePiece,
            Coord = Self::AbsoluteCoord,
            Side = Self::AbsoluteSide,
        >;
    type RelativeField;

    type AbsoluteSide: Copy + Eq + core::fmt::Debug + core::ops::Not;
    type RelativeSide: Copy + Eq;
    fn to_absolute_coord(coord: Self::RelativeCoord, p: Self::Perspective) -> Self::AbsoluteCoord;
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord>;
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece>;
    fn relative_clone_and_set(
        board: &Self::RelativeBoard,
        coord: Self::RelativeCoord,
        p: Option<Self::RelativePiece>,
    ) -> Self::RelativeBoard;
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece>;
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool;
    fn relative_tam2() -> Self::RelativePiece;
    fn absolute_tam2() -> Self::AbsolutePiece;
    fn is_upward(s: Self::RelativeSide) -> bool;
    fn match_on_piece_and_apply<U>(
        piece: Self::RelativePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(Profession, Self::RelativeSide) -> U,
    ) -> U;
    fn empty_squares_relative(current_board: &Self::RelativeBoard) -> Vec<Self::RelativeCoord>;
    fn empty_squares_absolute(current_board: &Self::AbsoluteBoard) -> Vec<Self::AbsoluteCoord>;
    fn hop1zuo1_of(
        side: Self::AbsoluteSide,
        field: &Self::AbsoluteField,
    ) -> Vec<cetkaik_core::ColorAndProf>;
    fn as_board_absolute(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard;
    fn as_board_mut_absolute(field: &mut Self::AbsoluteField) -> &mut Self::AbsoluteBoard;
    fn as_board_relative(field: &Self::RelativeField) -> &Self::RelativeBoard;
    fn is_water_relative(c: Self::RelativeCoord) -> bool;
    fn is_water_absolute(c: Self::AbsoluteCoord) -> bool;
    fn loop_over_one_side_and_tam(
        board: &Self::RelativeBoard,
        side: Self::RelativeSide,
        f_tam_or_piece: &mut dyn FnMut(Self::RelativeCoord, Option<Profession>),
    );
    fn to_relative_field(field: Self::AbsoluteField, p: Self::Perspective) -> Self::RelativeField;
    fn to_relative_side(side: Self::AbsoluteSide, p: Self::Perspective) -> Self::RelativeSide;
    fn get_one_perspective() -> Self::Perspective;
    fn absolute_distance(a: Self::AbsoluteCoord, b: Self::AbsoluteCoord) -> i32;
    fn absolute_same_direction(
        origin: Self::AbsoluteCoord,
        a: Self::AbsoluteCoord,
        b: Self::AbsoluteCoord,
    ) -> bool;
    fn to_cetkaikcore_absolute_side(a: Self::AbsoluteSide) -> cetkaik_core::absolute::Side;
    fn from_cetkaikcore_absolute_side(a: cetkaik_core::absolute::Side) -> Self::AbsoluteSide;

    fn has_prof_absolute(piece: Self::AbsolutePiece, prof: Profession) -> bool;
}

/// `cetkaik_core` クレートに基づいており、視点に依らない絶対座標での表現と、視点に依る相対座標への表現を正しく相互変換できる。
impl CetkaikRepresentation for CetkaikCore {
    type Perspective = crate::Perspective;

    type AbsoluteCoord = cetkaik_core::absolute::Coord;
    type RelativeCoord = cetkaik_core::relative::Coord;

    type AbsoluteBoard = cetkaik_core::absolute::Board;
    type RelativeBoard = cetkaik_core::relative::Board;

    type AbsolutePiece = cetkaik_core::absolute::Piece;
    type RelativePiece = cetkaik_core::relative::Piece;

    type AbsoluteSide = cetkaik_core::absolute::Side;
    type RelativeSide = cetkaik_core::relative::Side;

    type AbsoluteField = cetkaik_core::absolute::Field;
    type RelativeField = cetkaik_core::relative::Field;

    fn to_absolute_coord(coord: Self::RelativeCoord, p: Self::Perspective) -> Self::AbsoluteCoord {
        cetkaik_core::perspective::to_absolute_coord(coord, p)
    }
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord> {
        let [i, j] = coord;
        match (
            i.checked_add_signed(row_delta),
            j.checked_add_signed(col_delta),
        ) {
            (Some(l @ 0..=8), Some(m @ 0..=8)) => Some([l, m]),
            _ => None,
        }
    }
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece> {
        let [i, j] = coord;
        board[i][j]
    }
    fn relative_clone_and_set(
        board: &Self::RelativeBoard,
        coord: Self::RelativeCoord,
        p: Option<Self::RelativePiece>,
    ) -> Self::RelativeBoard {
        let [i, j] = coord;
        let mut new_board = *board;
        new_board[i][j] = p;
        new_board
    }
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece> {
        board.get(&coord).copied()
    }
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool {
        coord == [2, 2]
            || coord == [2, 6]
            || coord == [3, 3]
            || coord == [3, 5]
            || coord == [4, 4]
            || coord == [5, 3]
            || coord == [5, 5]
            || coord == [6, 2]
            || coord == [6, 6]
    }
    fn relative_tam2() -> Self::RelativePiece {
        cetkaik_core::relative::Piece::Tam2
    }
    fn absolute_tam2() -> Self::AbsolutePiece {
        cetkaik_core::absolute::Piece::Tam2
    }
    fn is_upward(s: Self::RelativeSide) -> bool {
        s == cetkaik_core::relative::Side::Upward
    }
    fn match_on_piece_and_apply<U>(
        piece: Self::RelativePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(Profession, Self::RelativeSide) -> U,
    ) -> U {
        match piece {
            Self::RelativePiece::Tam2 => f_tam(),
            Self::RelativePiece::NonTam2Piece {
                color: _,
                prof,
                side,
            } => f_piece(prof, side),
        }
    }
    fn empty_squares_relative(
        board: &cetkaik_core::relative::Board,
    ) -> Vec<cetkaik_core::relative::Coord> {
        let mut ans = vec![];
        for rand_i in 0..9 {
            for rand_j in 0..9 {
                let coord: cetkaik_core::relative::Coord = [rand_i, rand_j];
                if Self::relative_get(*board, coord).is_none() {
                    ans.push(coord);
                }
            }
        }
        ans
    }
    fn empty_squares_absolute(board: &cetkaik_core::absolute::Board) -> Vec<Self::AbsoluteCoord> {
        use absolute::Column::{C, K, L, M, N, P, T, X, Z};
        use absolute::Row::{A, AI, AU, E, I, IA, O, U, Y};
        let mut ans = vec![];
        for row in &[A, E, I, U, O, Y, AI, AU, IA] {
            for column in &[K, L, N, T, Z, X, C, M, P] {
                let coord = absolute::Coord(*row, *column);
                if Self::absolute_get(board, coord).is_none() {
                    ans.push(coord);
                }
            }
        }
        ans
    }
    fn hop1zuo1_of(
        side: Self::AbsoluteSide,
        field: &Self::AbsoluteField,
    ) -> Vec<cetkaik_core::ColorAndProf> {
        match side {
            absolute::Side::IASide => field.ia_side_hop1zuo1.clone(),
            absolute::Side::ASide => field.a_side_hop1zuo1.clone(),
        }
    }
    fn as_board_absolute(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard {
        &field.board
    }
    fn as_board_mut_absolute(field: &mut Self::AbsoluteField) -> &mut Self::AbsoluteBoard {
        &mut field.board
    }
    fn as_board_relative(field: &Self::RelativeField) -> &Self::RelativeBoard {
        &field.current_board
    }
    fn is_water_relative(c: Self::RelativeCoord) -> bool {
        cetkaik_core::relative::is_water(c)
    }
    fn is_water_absolute(c: Self::AbsoluteCoord) -> bool {
        cetkaik_core::absolute::is_water(c)
    }
    fn loop_over_one_side_and_tam(
        board: &Self::RelativeBoard,
        side: Self::RelativeSide,
        f_tam_or_piece: &mut dyn FnMut(Self::RelativeCoord, Option<Profession>),
    ) {
        for (rand_i, row) in board.iter().enumerate() {
            for (rand_j, &piece) in row.iter().enumerate() {
                let src = [rand_i, rand_j];
                if let Some(p) = piece {
                    match p {
                        Self::RelativePiece::Tam2 => f_tam_or_piece(src, None),
                        Self::RelativePiece::NonTam2Piece {
                            side: side_,
                            prof,
                            color: _,
                        } if side_ == side => f_tam_or_piece(src, Some(prof)),
                        Self::RelativePiece::NonTam2Piece { .. } => {}
                    }
                }
            }
        }
    }
    fn to_relative_field(field: Self::AbsoluteField, p: Self::Perspective) -> Self::RelativeField {
        cetkaik_core::perspective::to_relative_field(field, p)
    }
    fn to_relative_side(side: Self::AbsoluteSide, p: Self::Perspective) -> Self::RelativeSide {
        cetkaik_core::perspective::to_relative_side(side, p)
    }
    fn get_one_perspective() -> Self::Perspective {
        // arbitrary
        cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward
    }
    fn absolute_distance(a: Self::AbsoluteCoord, b: Self::AbsoluteCoord) -> i32 {
        cetkaik_core::absolute::distance(a, b)
    }
    fn absolute_same_direction(
        origin: Self::AbsoluteCoord,
        a: Self::AbsoluteCoord,
        b: Self::AbsoluteCoord,
    ) -> bool {
        cetkaik_core::absolute::same_direction(origin, a, b)
    }
    fn to_cetkaikcore_absolute_side(a: Self::AbsoluteSide) -> cetkaik_core::absolute::Side {
        a
    }
    fn from_cetkaikcore_absolute_side(a: cetkaik_core::absolute::Side) -> Self::AbsoluteSide {
        a
    }
    fn has_prof_absolute(piece: Self::AbsolutePiece, prof: Profession) -> bool {
        piece.has_prof(prof)
    }
}

/// `cetkaik_compact_representation` クレートに基づいており、視点を決め打ちして絶対座標=相対座標として表現する。
/// この impl においては、IAは常に一番下の行であり、初期状態でIA行を占有していたプレイヤーは駒が上向き（=あなた）である。
/// つまり、`Upward` は常に `IASide` へと読み替えられる。
impl CetkaikRepresentation for CetkaikCompact {
    type Perspective = cetkaik_compact_representation::Perspective;

    type AbsoluteCoord = cetkaik_compact_representation::Coord;
    type RelativeCoord = cetkaik_compact_representation::Coord;

    type AbsoluteBoard = cetkaik_compact_representation::Board;
    type RelativeBoard = cetkaik_compact_representation::Board;

    type AbsolutePiece = cetkaik_compact_representation::PieceWithSide;
    type RelativePiece = cetkaik_compact_representation::PieceWithSide;

    type AbsoluteField = cetkaik_compact_representation::Field;
    type RelativeField = cetkaik_compact_representation::Field;

    type AbsoluteSide = cetkaik_core::absolute::Side;
    type RelativeSide = cetkaik_core::absolute::Side; // ここも absolute
    fn to_absolute_coord(coord: Self::RelativeCoord, _p: Self::Perspective) -> Self::AbsoluteCoord {
        coord
    }
    fn add_delta(
        coord: Self::RelativeCoord,
        row_delta: isize,
        col_delta: isize,
    ) -> Option<Self::RelativeCoord> {
        cetkaik_compact_representation::Coord::add_delta(coord, row_delta, col_delta)
    }
    fn relative_get(
        board: Self::RelativeBoard,
        coord: Self::RelativeCoord,
    ) -> Option<Self::RelativePiece> {
        use cetkaik_core::IsBoard;
        board.peek(coord)
    }
    fn relative_clone_and_set(
        board: &Self::RelativeBoard,
        coord: Self::RelativeCoord,
        p: Option<Self::RelativePiece>,
    ) -> Self::RelativeBoard {
        use cetkaik_core::IsBoard;
        let mut new_board = *board;
        new_board.put(coord, p);
        new_board
    }
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece> {
        use cetkaik_core::IsBoard;
        board.peek(coord)
    }
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool {
        Self::RelativeCoord::is_tam_hue_by_default(coord)
    }
    fn relative_tam2() -> Self::RelativePiece {
        unsafe { cetkaik_compact_representation::PieceWithSide::new_unchecked(0o300) }
    }
    fn absolute_tam2() -> Self::AbsolutePiece {
        unsafe { cetkaik_compact_representation::PieceWithSide::new_unchecked(0o300) }
    }
    fn is_upward(s: Self::RelativeSide) -> bool {
        s == cetkaik_core::absolute::Side::IASide
    }
    fn match_on_piece_and_apply<U>(
        piece: Self::RelativePiece,
        f_tam: &dyn Fn() -> U,
        f_piece: &dyn Fn(Profession, Self::RelativeSide) -> U,
    ) -> U {
        match piece.prof_and_side() {
            cetkaik_compact_representation::MaybeTam2::Tam2 => f_tam(),
            cetkaik_compact_representation::MaybeTam2::NotTam2((prof, side)) => f_piece(prof, side),
        }
    }
    fn empty_squares_relative(board: &Self::RelativeBoard) -> Vec<Self::RelativeCoord> {
        let mut ans = vec![];
        for rand_i in 0..9 {
            for rand_j in 0..9 {
                let coord: Self::RelativeCoord = Self::RelativeCoord::new(rand_i, rand_j).unwrap();
                if Self::relative_get(*board, coord).is_none() {
                    ans.push(coord);
                }
            }
        }
        ans
    }
    fn empty_squares_absolute(board: &Self::RelativeBoard) -> Vec<Self::RelativeCoord> {
        Self::empty_squares_relative(board)
    }

    fn hop1zuo1_of(
        side: Self::AbsoluteSide,
        field: &Self::AbsoluteField,
    ) -> Vec<cetkaik_core::ColorAndProf> {
        match side {
            absolute::Side::ASide => field
                .to_hop1zuo1()
                .a_side_hop1zuo1_color_and_prof()
                .collect(),
            absolute::Side::IASide => field
                .to_hop1zuo1()
                .ia_side_hop1zuo1_color_and_prof()
                .collect(),
        }
    }
    fn as_board_absolute(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard {
        field.as_board()
    }
    fn as_board_mut_absolute(field: &mut Self::AbsoluteField) -> &mut Self::AbsoluteBoard {
        field.as_board_mut()
    }
    fn as_board_relative(field: &Self::RelativeField) -> &Self::RelativeBoard {
        field.as_board()
    }
    fn is_water_relative(c: Self::RelativeCoord) -> bool {
        cetkaik_compact_representation::Coord::is_water(c)
    }
    fn is_water_absolute(c: Self::AbsoluteCoord) -> bool {
        cetkaik_compact_representation::Coord::is_water(c)
    }
    fn loop_over_one_side_and_tam(
        board: &Self::RelativeBoard,
        side: Self::RelativeSide,
        f_tam_or_piece: &mut dyn FnMut(Self::RelativeCoord, Option<Profession>),
    ) {
        let fun = |(src, piece): (Self::RelativeCoord, Self::RelativePiece)| match piece
            .prof_and_side()
        {
            cetkaik_compact_representation::MaybeTam2::Tam2 => f_tam_or_piece(src, None),
            cetkaik_compact_representation::MaybeTam2::NotTam2((prof, _)) => {
                f_tam_or_piece(src, Some(prof));
            }
        };
        match side {
            absolute::Side::ASide => board.a_side_and_tam().for_each(fun),
            absolute::Side::IASide => board.ia_side_and_tam().for_each(fun),
        }
    }

    fn to_relative_field(field: Self::AbsoluteField, _p: Self::Perspective) -> Self::RelativeField {
        field
    }

    fn to_relative_side(side: Self::AbsoluteSide, _p: Self::Perspective) -> Self::RelativeSide {
        side
    }
    fn get_one_perspective() -> Self::Perspective {
        // the sole choice available
        cetkaik_compact_representation::Perspective::IaIsDownAndPointsUpward
    }

    fn absolute_distance(a: Self::AbsoluteCoord, b: Self::AbsoluteCoord) -> i32 {
        cetkaik_compact_representation::Coord::distance(a, b)
    }

    fn absolute_same_direction(
        origin: Self::AbsoluteCoord,
        a: Self::AbsoluteCoord,
        b: Self::AbsoluteCoord,
    ) -> bool {
        cetkaik_compact_representation::Coord::same_direction(origin, a, b)
    }
    fn to_cetkaikcore_absolute_side(a: Self::AbsoluteSide) -> cetkaik_core::absolute::Side {
        a
    }
    fn from_cetkaikcore_absolute_side(a: cetkaik_core::absolute::Side) -> Self::AbsoluteSide {
        a
    }

    fn has_prof_absolute(piece: Self::AbsolutePiece, prof: Profession) -> bool {
        use cetkaik_compact_representation::MaybeTam2;
        match piece.prof() {
            MaybeTam2::Tam2 => false,
            MaybeTam2::NotTam2(self_prof) => prof == self_prof,
        }
    }
}

/// # Example
///
/// Using `cetkaik_core`:
/// ```
/// use cetkaik_yhuap_move_candidates::from_hop1zuo1_candidates_vec;
/// use cetkaik_core::*;
/// use cetkaik_core::absolute::Field;
/// use cetkaik_core::ColorAndProf;
/// use cetkaik_core::absolute::Coord;
/// use cetkaik_core::absolute::Column::*;
/// use cetkaik_core::absolute::Row::*;
/// use cetkaik_yhuap_move_candidates::CetkaikCore;
/// use std::collections::HashSet;
///
/// // There are eighty unoccupied squares on the board, and `IASide` has two pieces in hop1zuo1
/// let vec = from_hop1zuo1_candidates_vec::<CetkaikCore>(
///     cetkaik_core::absolute::Side::IASide,
///     &Field {
///         a_side_hop1zuo1: vec![ColorAndProf {
///             color: Color::Huok2,
///             prof: Profession::Gua2,
///         }],
///         ia_side_hop1zuo1: vec![ColorAndProf {
///             color: Color::Kok1,
///             prof: Profession::Kauk2,
///         }, ColorAndProf {
///             color: Color::Huok2,
///             prof: Profession::Nuak1,
///         }],
///         board: vec![
///             (Coord(AU, C), absolute::Piece::NonTam2Piece {
///             color: Color::Kok1,
///             prof: Profession::Nuak1,
///             side: absolute::Side::IASide
///         })
///         ]
///         .into_iter()
///         .collect(),
///     }
/// );
///
/// assert_eq!(vec.len(), 80 * 2)
///
/// ```
#[must_use]
pub fn from_hop1zuo1_candidates_vec<T: CetkaikRepresentation>(
    whose_turn: T::AbsoluteSide,
    field: &T::AbsoluteField,
) -> Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>> {
    T::hop1zuo1_of(whose_turn, field)
        .into_iter()
        .flat_map(|cetkaik_core::ColorAndProf { color, prof }| {
            T::empty_squares_absolute(T::as_board_absolute(field))
                .into_iter()
                .map(move |dest| cetkaik_core::PureMove_::NonTamMoveFromHopZuo {
                    color,
                    prof,
                    dest,
                })
        })
        .collect()
}

mod calculate_movable;
pub use calculate_movable::calculate_movable_positions_for_either_side;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct MovablePositions<T> {
    pub finite: Vec<T>,
    pub infinite: Vec<T>,
}

fn empty_neighbors_of<T: CetkaikRepresentation>(
    board: T::RelativeBoard,
    c: T::RelativeCoord,
) -> impl Iterator<Item = T::RelativeCoord> {
    calculate_movable::iter::eight_neighborhood::<T>(c)
        .filter(move |coord| T::relative_get(board, *coord).is_none())
}

fn can_get_occupied_by_non_tam<T: CetkaikRepresentation>(
    side: T::RelativeSide,
    dest: T::RelativeCoord,
    board: T::RelativeBoard,
    tam_itself_is_tam_hue: bool,
) -> bool {
    /* Intentionally does not verify whether the piece itself is of opponent */
    let is_protected_by_opponent_tam_hue_auai = |side: T::RelativeSide, coord: T::RelativeCoord| {
        calculate_movable::vec::eight_neighborhood::<T>(coord)
            .into_iter()
            .filter(|ab| {
                T::relative_get(board, *ab).map_or(false, |piece| {
                    T::match_on_piece_and_apply(piece, &|| false, &|piece_prof, piece_side| {
                        piece_prof == Profession::Uai1
                            && piece_side != side
                            && calculate_movable::is_tam_hue::<T>(*ab, board, tam_itself_is_tam_hue)
                    })
                })
            })
            .count()
            > 0
    };

    T::relative_get(board, dest).map_or(true, |piece| {
        T::match_on_piece_and_apply(
            piece,
            &|| false, /* Tam2 can never be taken */
            &|_, piece_side| {
                piece_side != side /* cannot take your own piece */ &&
        !is_protected_by_opponent_tam_hue_auai(
            side,
            dest
        )
            }, /* must not be protected by tam2 hue a uai1 */
        )
    })
}

fn is_ciurl_required<T: CetkaikRepresentation>(
    dest: T::RelativeCoord,
    moving_piece_prof: Profession,
    src: T::RelativeCoord,
) -> bool {
    T::is_water_relative(dest)
        && !T::is_water_relative(src)
        && !matches!(moving_piece_prof, Profession::Nuak1)
}

/// Note that 皇再来 (tam2 ty sak2) is explicitly allowed, since its filtering / handling is the job of `cetkaik_full_state_transition`.
/// # Example
/// ```
/// use cetkaik_yhuap_move_candidates::not_from_hop1zuo1_candidates_vec;
/// use cetkaik_yhuap_move_candidates::Config;
/// use cetkaik_yhuap_move_candidates::CetkaikCore;
/// use cetkaik_core::*;
/// use cetkaik_core::absolute::*;
/// use cetkaik_core::absolute::Row::*;
/// use cetkaik_core::absolute::Column::*;
/// use cetkaik_core::PureMove_::*;
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
/// // 船一つ
/// assert_eq_ignoring_order(
///     &not_from_hop1zuo1_candidates_vec::<CetkaikCore>(
///         &Config {
///             allow_kut2tam2: false,
///         },
///         false,
///         absolute::Side::IASide,
///         &absolute::Field {
///              a_side_hop1zuo1: vec![],
///              ia_side_hop1zuo1: vec![],
///              board: vec![
///                  (absolute::Coord(AU, C), absolute::Piece::NonTam2Piece {
///                      color: Color::Kok1,
///                      prof: Profession::Nuak1,
///                      side: absolute::Side::IASide
///                  })
///             ]
///             .into_iter()
///             .collect()
///         }
///     ),
///     &[
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(A, C), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(E, C), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(I, C), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(U, C), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(O, C), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(Y, C), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: absolute::Coord(AU, C), dest: absolute::Coord(AI, C), is_water_entry_ciurl: false },
///     ]
/// );
///
///
/// // 弓が色々踏む
/// assert_eq_ignoring_order(
///     &not_from_hop1zuo1_candidates_vec::<CetkaikCore>(
///         &Config {
///             allow_kut2tam2: false,
///         },
///         false,
///         absolute::Side::IASide,
///         &absolute::Field {
///              a_side_hop1zuo1: vec![],
///              ia_side_hop1zuo1: vec![],
///              board: vec![
///                  (absolute::Coord(AI, L), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Gua2,
///                      side: absolute::Side::IASide
///                  }),
///                  (absolute::Coord(I, L), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Kauk2,
///                      side: absolute::Side::ASide
///                  }),
///                  (absolute::Coord(I, N), absolute::Piece::NonTam2Piece {
///                      color: Color::Kok1,
///                      prof: Profession::Uai1,
///                      side: absolute::Side::ASide
///                  }),
///                  (absolute::Coord(AI, N), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Kaun1,
///                      side: absolute::Side::ASide
///                  }),
///                  (absolute::Coord(AU, L), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Kauk2,
///                      side: absolute::Side::ASide
///                  }),
///             ]
///             .into_iter()
///             .collect()
///         }
///     ),
///     &[
///         // 左
///         NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(AI, K), is_water_entry_ciurl: false },
///
///         // 下
///         NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(AU, L), is_water_entry_ciurl: false },
///
///         // 下のち左
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, K) },
///
///         // 下のち下
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(IA, L) },
///         
///         // 下のち右
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, N) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, T) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, Z) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, X) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, C) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, M) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AU, P) },
///
///         // 下のち上
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(AI, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(Y, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(O, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AU, L), planned_direction: Coord(U, L) },
///
///         // 右
///         NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(AI, N), is_water_entry_ciurl: false },
///
///         // 右のち左上
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(Y, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(O, K) },
///
///         // 右のち右上
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(Y, T) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(O, Z) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(U, X) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(I, C) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(E, M) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(A, P) },
///
///         // 右のち左下
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(AU, L) },
///
///         // 右のち右下
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(AU, T) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(IA, Z) },
///
///         // 上
///         NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(Y, L), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(O, L), is_water_entry_ciurl: false },
///         NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(U, L), is_water_entry_ciurl: false },
///         // LI は皇処之将に守られているので取れない
///
///         // 上のち上
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(E, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(A, L) },
///
///         // 上のち下
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(U, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(O, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(Y, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(AI, L) },
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(AU, L) },
///
///         // 上のち右
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(I, N) },
///
///         // 上のち左
///         InfAfterStep { src: Coord(AI, L), step: Coord(I, L), planned_direction: Coord(I, K) }
///     ]
/// );
/// ```
#[must_use]
pub fn not_from_hop1zuo1_candidates_vec<T: CetkaikRepresentation>(
    config: &Config,
    tam_itself_is_tam_hue: bool,
    whose_turn: T::AbsoluteSide,
    f: &T::AbsoluteField,
) -> Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>> {
    let perspective = T::get_one_perspective();
    not_from_hop1zuo1_candidates_::<T>(
        T::to_relative_side(whose_turn, perspective),
        *config,
        perspective,
        tam_itself_is_tam_hue,
        &T::to_relative_field((*f).clone(), perspective),
    )
}

fn candidates_tam2<T: CetkaikRepresentation>(
    src: T::RelativeCoord,
    f: &T::RelativeField,
    perspective: T::Perspective,
    ans: &mut Vec<PureMove_<T::AbsoluteCoord>>,
) {
    let candidates: Vec<T::RelativeCoord> = calculate_movable::vec::eight_neighborhood::<T>(src);
    for tentative_dest in candidates {
        let dest_piece = T::relative_get(*T::as_board_relative(f), tentative_dest);

        /* avoid self-occlusion */
        let subtracted_board = T::relative_clone_and_set(T::as_board_relative(f), src, None);
        if dest_piece.is_none() {
            /* empty square; first move is completed without stepping */
            let fst_dst: T::RelativeCoord = tentative_dest;
            ans.append(&mut calculate_movable::iter::eight_neighborhood::<T>(fst_dst).flat_map(|neighbor| {
                            /* if the neighbor is empty, that is the second destination */
                            let snd_dst: T::RelativeCoord = neighbor;
                            if T::relative_get(*T::as_board_relative(f), neighbor).is_none() /* the neighbor is utterly occupied */ ||
                                neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                vec![cetkaik_core::PureMove_::TamMoveNoStep {
                                    second_dest: T::to_absolute_coord(snd_dst, perspective),
                                    first_dest: T::to_absolute_coord(fst_dst, perspective),
                                    src: T::to_absolute_coord(src, perspective),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: T::RelativeCoord = neighbor;
                                empty_neighbors_of::<T>(subtracted_board, step)
                                    .flat_map(|snd_dst| {
                                    vec![cetkaik_core::PureMove_::TamMoveStepsDuringLatter {
                                        first_dest: T::to_absolute_coord(fst_dst, perspective),
                                        second_dest: T::to_absolute_coord(snd_dst, perspective),
                                        src: T::to_absolute_coord(src, perspective),
                                        step: T::to_absolute_coord(step, perspective),
                                    }].into_iter()
                                }).collect::<Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>>>().into_iter()
                            }
                        }).collect::<Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>>>());
        } else {
            /* not an empty square: must complete the first move */
            let step = tentative_dest;
            ans.append(
                &mut empty_neighbors_of::<T>(subtracted_board, step)
                    .flat_map(|fst_dst| {
                        let v = empty_neighbors_of::<T>(subtracted_board, fst_dst);
                        v.flat_map(move |snd_dst| {
                            vec![cetkaik_core::PureMove_::TamMoveStepsDuringFormer {
                                first_dest: T::to_absolute_coord(fst_dst, perspective),
                                second_dest: T::to_absolute_coord(snd_dst, perspective),
                                src: T::to_absolute_coord(src, perspective),
                                step: T::to_absolute_coord(step, perspective),
                            }]
                            .into_iter()
                        })
                        .collect::<Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>>>()
                        .into_iter()
                    })
                    .collect::<Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>>>(),
            );
        }
    }
}

fn append_possible_movement_of_a_piece<T: CetkaikRepresentation>(
    side: T::RelativeSide,
    config: Config2,
    prof: Profession,
    src: T::RelativeCoord,
    field: &T::RelativeField,
    perspective: T::Perspective,
    ans: &mut Vec<PureMove_<T::AbsoluteCoord>>,
) {
    let MovablePositions { finite, infinite } =
        calculate_movable::calculate_movable_positions_for_nontam::<T>(
            src,
            prof,
            *T::as_board_relative(field),
            config.tam_itself_is_tam_hue,
            side,
        );

    let candidates: Vec<T::RelativeCoord> = [&finite[..], &infinite[..]].concat();
    for tentative_dest in candidates {
        let dest_piece = T::relative_get(*T::as_board_relative(field), tentative_dest);

        let candidates_when_stepping = || {
            let step = tentative_dest; // tentative_dest becomes the position on which the stepping occurs

            let perspective = perspective;
            let tam_itself_is_tam_hue: bool = config.tam_itself_is_tam_hue;
            /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
            let subtracted_board =
                T::relative_clone_and_set(T::as_board_relative(field), src, None); /* must remove the piece to prevent self-occlusion */

            let MovablePositions { finite, infinite } =
                calculate_movable::calculate_movable_positions_for_nontam::<T>(
                    step,
                    prof,
                    subtracted_board,
                    tam_itself_is_tam_hue,
                    side,
                );

            let candidates = finite.into_iter();
            let candidates_inf = infinite.into_iter();

            let candidates_abs = candidates
                .flat_map(|final_dest| {
                    if can_get_occupied_by_non_tam::<T>(
                        side,
                        final_dest,
                        subtracted_board,
                        tam_itself_is_tam_hue,
                    ) {
                        vec![cetkaik_core::PureMove_::NonTamMoveSrcStepDstFinite {
                            src: T::to_absolute_coord(src, perspective),
                            step: T::to_absolute_coord(step, perspective),
                            dest: T::to_absolute_coord(final_dest, perspective),
                            is_water_entry_ciurl: is_ciurl_required::<T>(final_dest, prof, src),
                        }]
                        .into_iter()
                    } else {
                        vec![].into_iter()
                    }
                })
                .collect::<Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>>>();
            let candidates_inf_abs = candidates_inf
                .flat_map(|planned_dest| {
                    if !can_get_occupied_by_non_tam::<T>(
                        side,
                        planned_dest,
                        subtracted_board,
                        tam_itself_is_tam_hue,
                    ) {
                        return vec![].into_iter();
                        // retry
                    }
                    let obj: cetkaik_core::PureMove_<T::AbsoluteCoord> =
                        cetkaik_core::PureMove_::InfAfterStep {
                            src: T::to_absolute_coord(src, perspective),
                            step: T::to_absolute_coord(step, perspective),
                            planned_direction: T::to_absolute_coord(planned_dest, perspective),
                        };
                    vec![obj].into_iter()
                })
                .collect::<Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>>>();
            [&candidates_abs[..], &candidates_inf_abs[..]].concat()
        };
        if let Some(piece) = dest_piece {
            let mut a = T::match_on_piece_and_apply(
                piece,
                &|| {
                    // if allowed by config, allow stepping on Tam2;
                    if config.allow_kut2tam2 {
                        candidates_when_stepping()
                    } else {
                        vec![]
                    }
                },
                &|prof, side_| {
                    if side_ == side {
                        candidates_when_stepping()
                    } else {
                        // opponent's piece; stepping and taking both attainable

                        // except when protected by tam2 hue a uai1
                        if can_get_occupied_by_non_tam::<T>(
                            side,
                            tentative_dest,
                            *T::as_board_relative(field),
                            config.tam_itself_is_tam_hue,
                        ) {
                            [
                                &[cetkaik_core::PureMove_::NonTamMoveSrcDst {
                                    src: T::to_absolute_coord(src, perspective),
                                    dest: T::to_absolute_coord(tentative_dest, perspective),
                                    is_water_entry_ciurl: is_ciurl_required::<T>(
                                        tentative_dest,
                                        prof,
                                        src,
                                    ),
                                }][..],
                                &candidates_when_stepping()[..],
                            ]
                            .concat()
                        } else {
                            candidates_when_stepping()
                        }
                    }
                },
            );
            ans.append(&mut a);
        } else {
            // cannot step
            ans.append(&mut vec![cetkaik_core::PureMove_::NonTamMoveSrcDst {
                src: T::to_absolute_coord(src, perspective),
                dest: T::to_absolute_coord(tentative_dest, perspective),
                is_water_entry_ciurl: is_ciurl_required::<T>(tentative_dest, prof, src),
            }]);
        }
    }
}

fn not_from_hop1zuo1_candidates_<T: CetkaikRepresentation>(
    side: T::RelativeSide,
    config: Config,
    perspective: T::Perspective,
    tam_itself_is_tam_hue: bool,
    field: &T::RelativeField,
) -> Vec<cetkaik_core::PureMove_<T::AbsoluteCoord>> {
    let mut ans = vec![];
    T::loop_over_one_side_and_tam(T::as_board_relative(field), side, &mut |src, maybe_prof| {
        match maybe_prof {
            None => candidates_tam2::<T>(src, field, perspective, &mut ans),
            Some(prof) => append_possible_movement_of_a_piece::<T>(
                side,
                Config2 {
                    tam_itself_is_tam_hue,
                    allow_kut2tam2: config.allow_kut2tam2,
                },
                prof,
                src,
                field,
                perspective,
                &mut ans,
            ),
        }
    });
    ans
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    pub allow_kut2tam2: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Config2 {
    pub allow_kut2tam2: bool,
    pub tam_itself_is_tam_hue: bool,
}

#[cfg(test)]
mod tests;

pub use cetkaik_core::perspective::{to_absolute_coord, Perspective};
pub use cetkaik_core::{Color, Profession};

/// According to <https://github.com/cetkaik/cetkaik_yhuap_move_candidates/pull/7>,
/// this function was a bottleneck that accounted for roughly fifty percent of all the runtime
/// in an attempt to implement an AI for this game.
/// Hence, this module contains multiple implementations of this functions and test that they are equal.
#[allow(clippy::similar_names)]
pub mod get_blocker_deltas;
