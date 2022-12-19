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

pub struct CetkaikCore;

pub struct CetkaikCompact;

pub trait CetkaikRepresentation {
    type Perspective;

    type AbsoluteCoord;
    type RelativeCoord: Copy;

    type AbsoluteBoard;
    type RelativeBoard: Copy;

    type AbsolutePiece: Copy + Eq;
    type RelativePiece: Copy + Eq;

    type AbsoluteField;

    type AbsoluteSide: Copy + Eq;
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
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece>;
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool;
    fn tam2() -> Self::RelativePiece;
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
    ) -> Vec<(Color, Profession)>;
    fn as_board(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard;
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
    fn tam2() -> Self::RelativePiece {
        cetkaik_core::relative::Piece::Tam2
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
            Piece::Tam2 => f_tam(),
            Piece::NonTam2Piece {
                color: _,
                prof,
                side,
            } => f_piece(prof, side),
        }
    }
    fn empty_squares_relative(board: &cetkaik_core::relative::Board) -> Vec<Coord> {
        let mut ans = vec![];
        for rand_i in 0..9 {
            for rand_j in 0..9 {
                let coord: Coord = [rand_i, rand_j];
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
    type AbsoluteField = cetkaik_core::absolute::Field;
    fn hop1zuo1_of(
        side: Self::AbsoluteSide,
        field: &Self::AbsoluteField,
    ) -> Vec<(Color, Profession)> {
        match side {
            absolute::Side::IASide => field.ia_side_hop1zuo1.iter(),
            absolute::Side::ASide => field.a_side_hop1zuo1.iter(),
        }
        .copied()
        .map(|absolute::NonTam2Piece { color, prof }| (color, prof))
        .collect()
    }
    fn as_board(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard {
        &field.board
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
        board.peek(coord)
    }
    fn absolute_get(
        board: &Self::AbsoluteBoard,
        coord: Self::AbsoluteCoord,
    ) -> Option<Self::AbsolutePiece> {
        board.peek(coord)
    }
    fn is_tam_hue_by_default(coord: Self::RelativeCoord) -> bool {
        Self::RelativeCoord::is_tam_hue_by_default(coord)
    }
    fn tam2() -> Self::RelativePiece {
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
    ) -> Vec<(Color, Profession)> {
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
    fn as_board(field: &Self::AbsoluteField) -> &Self::AbsoluteBoard {
        field.as_board()
    }
}

/// # Example
///
/// Using `cetkaik_core`:
/// ```
/// use cetkaik_yhuap_move_candidates::from_hop1zuo1_candidates_vec;
/// use cetkaik_core::*;
/// use cetkaik_core::absolute::Field;
/// use cetkaik_core::absolute::NonTam2Piece;
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
///         a_side_hop1zuo1: vec![NonTam2Piece {
///             color: Color::Huok2,
///             prof: Profession::Gua2,
///         }],
///         ia_side_hop1zuo1: vec![NonTam2Piece {
///             color: Color::Kok1,
///             prof: Profession::Kauk2,
///         }, NonTam2Piece {
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
        .flat_map(|(color, prof)| {
            T::empty_squares_absolute(T::as_board(field))
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

fn can_get_occupied_by<T: CetkaikRepresentation>(
    side: T::RelativeSide,
    dest: T::RelativeCoord,
    piece_to_move: T::RelativePiece,
    board: T::RelativeBoard,
    tam_itself_is_tam_hue: bool,
) -> bool {
    if piece_to_move == T::tam2() {
        /* It is allowed to enter an empty square */
        T::relative_get(board, dest).is_none()
    } else {
        can_get_occupied_by_non_tam::<T>(side, dest, board, tam_itself_is_tam_hue)
    }
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

const fn is_ciurl_required(dest: Coord, moving_piece_prof: Profession, src: Coord) -> bool {
    is_water(dest) && !is_water(src) && !matches!(moving_piece_prof, Profession::Nuak1)
}

/// Note that 皇再来 (tam2 ty sak2) is explicitly allowed, since its filtering / handling is the job of `cetkaik_full_state_transition`.
#[must_use]
pub fn not_from_hop1zuo1_candidates2(
    config: &Config,
    tam_itself_is_tam_hue: bool,
    whose_turn: absolute::Side,
    f: &absolute::Field,
) -> Vec<cetkaik_core::PureMove_<absolute::Coord>> {
    let perspective = match whose_turn {
        absolute::Side::IASide => cetkaik_core::perspective::Perspective::IaIsUpAndPointsDownward,
        absolute::Side::ASide => cetkaik_core::perspective::Perspective::IaIsDownAndPointsUpward,
    };
    not_from_hop1zuo1_candidates_(
        config,
        perspective,
        tam_itself_is_tam_hue,
        &cetkaik_core::perspective::to_relative_field(f.clone(), perspective),
    )
}

fn not_from_hop1zuo1_candidates_(
    config: &Config,
    perspective: Perspective,
    tam_itself_is_tam_hue: bool,
    f: &cetkaik_core::relative::Field,
) -> Vec<cetkaik_core::PureMove_<<CetkaikCore as CetkaikRepresentation>::AbsoluteCoord>> {
    let mut ans = vec![];
    for rand_i in 0..9 {
        for rand_j in 0..9 {
            let src = [rand_i, rand_j];
            let piece = f.current_board[rand_i][rand_j];
            if let Some(p) = piece {
                match p {
                    Piece::Tam2 => {
                        let candidates: Vec<<CetkaikCore as CetkaikRepresentation>::RelativeCoord> =
                            calculate_movable::vec::eight_neighborhood::<CetkaikCore>(src);
                        for tentative_dest in candidates {
                            let dest_piece = f.current_board[tentative_dest[0]][tentative_dest[1]];

                            /* avoid self-occlusion */
                            let mut subtracted_board = f.current_board;
                            subtracted_board[src[0]][src[1]] = None;
                            if dest_piece.is_none() {
                                /* empty square; first move is completed without stepping */
                                let fst_dst: Coord = tentative_dest;
                                ans.append(&mut calculate_movable::iter::eight_neighborhood::<CetkaikCore>(fst_dst).flat_map(|neighbor| {
                            /* if the neighbor is empty, that is the second destination */
                            let snd_dst: Coord = neighbor;
                            if f.current_board[neighbor[0]][neighbor[1]].is_none() /* the neighbor is utterly occupied */ ||
                                neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                vec![cetkaik_core::PureMove_::TamMoveNoStep {
                                    second_dest: to_absolute_coord(snd_dst, perspective),
                                    first_dest: to_absolute_coord(fst_dst, perspective),
                                    src: to_absolute_coord(src, perspective),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: Coord = neighbor;
                                empty_neighbors_of::<CetkaikCore>(subtracted_board, step)
                                    .flat_map(|snd_dst| {
                                    vec![cetkaik_core::PureMove_::TamMoveStepsDuringLatter {
                                        first_dest: to_absolute_coord(fst_dst, perspective),
                                        second_dest: to_absolute_coord(snd_dst, perspective),
                                        src: to_absolute_coord(src, perspective),
                                        step: to_absolute_coord(step, perspective),
                                    }].into_iter()
                                }).collect::<Vec<cetkaik_core::PureMove_<absolute::Coord>>>().into_iter()
                            }
                        }).collect::<Vec<cetkaik_core::PureMove_<absolute::Coord>>>());
                            } else {
                                /* not an empty square: must complete the first move */
                                let step = tentative_dest;
                                ans.append(
                                    &mut empty_neighbors_of::<CetkaikCore>(subtracted_board, step)
                                        .flat_map(|fst_dst| {
                                            let v = empty_neighbors_of::<CetkaikCore>(
                                                subtracted_board,
                                                fst_dst,
                                            );
                                            v.flat_map(move |snd_dst| {
                                                vec![cetkaik_core::PureMove_::TamMoveStepsDuringFormer {
                                                    first_dest: to_absolute_coord(
                                                        fst_dst,
                                                        perspective,
                                                    ),
                                                    second_dest: to_absolute_coord(
                                                        snd_dst,
                                                        perspective,
                                                    ),
                                                    src: to_absolute_coord(
                                                        src,
                                                        perspective,
                                                    ),
                                                    step: to_absolute_coord(
                                                        step,
                                                        perspective,
                                                    ),
                                                }]
                                                .into_iter()
                                            })
                                            .collect::<Vec<cetkaik_core::PureMove_<absolute::Coord>>>()
                                            .into_iter()
                                        })
                                        .collect::<Vec<cetkaik_core::PureMove_<absolute::Coord>>>(),
                                );
                            }
                        }
                    }
                    Piece::NonTam2Piece {
                        side: Side::Downward,
                        prof,
                        color,
                    } => {
                        let MovablePositions { finite, infinite } =
                            calculate_movable::calculate_movable_positions_for_nontam::<CetkaikCore>(
                                src,
                                prof,
                                f.current_board,
                                tam_itself_is_tam_hue,
                                Side::Downward,
                            );

                        let candidates: Vec<Coord> = [&finite[..], &infinite[..]].concat();
                        for tentative_dest in candidates {
                            let dest_piece = f.current_board[tentative_dest[0]][tentative_dest[1]];

                            let candidates_when_stepping = || {
                                let step = tentative_dest; // tentative_dest becomes the position on which the stepping occurs

                                let perspective = perspective;
                                let tam_itself_is_tam_hue: bool = tam_itself_is_tam_hue;
                                /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
                                let mut subtracted_board = f.current_board;
                                subtracted_board[src[0]][src[1]] = None; /* must remove the piece to prevent self-occlusion */

                                let MovablePositions { finite, infinite } =
                                    calculate_movable::calculate_movable_positions_for_nontam::<
                                        CetkaikCore,
                                    >(
                                        step,
                                        prof,
                                        subtracted_board,
                                        tam_itself_is_tam_hue,
                                        Side::Downward,
                                    );

                                let candidates = finite.into_iter();
                                let candidates_inf = infinite.into_iter();
                                [
                                    &candidates
                                        .flat_map(|final_dest| {
                                            if can_get_occupied_by::<CetkaikCore>(
                                                Side::Downward,
                                                final_dest,
                                                Piece::NonTam2Piece {
                                                    color,
                                                    prof,
                                                    side: Side::Downward,
                                                },
                                                subtracted_board,
                                                tam_itself_is_tam_hue,
                                            ) {
                                                vec![cetkaik_core::PureMove_::NonTamMoveSrcStepDstFinite {
                                                    src: to_absolute_coord(src, perspective),
                                                    step: to_absolute_coord(step, perspective),
                                                    dest: to_absolute_coord(
                                                        final_dest,
                                                        perspective,
                                                    ),
                                                    is_water_entry_ciurl: is_ciurl_required(
                                                        final_dest, prof, src,
                                                    ),
                                                }]
                                                .into_iter()
                                            } else {
                                                vec![].into_iter()
                                            }
                                        })
                                        .collect::<Vec<cetkaik_core::PureMove_<absolute::Coord>>>()[..],
                                    &candidates_inf
                                        .flat_map(|planned_dest| {
                                            if !can_get_occupied_by::<CetkaikCore>(
                                                Side::Downward,
                                                planned_dest,
                                                Piece::NonTam2Piece {
                                                    color,
                                                    prof,
                                                    side: Side::Downward,
                                                },
                                                subtracted_board,
                                                tam_itself_is_tam_hue,
                                            ) {
                                                return vec![].into_iter();
                                                // retry
                                            }
                                            let obj: cetkaik_core::PureMove_<absolute::Coord> = cetkaik_core::PureMove_::InfAfterStep {
                                                src: to_absolute_coord(src, perspective),
                                                step: to_absolute_coord(step, perspective),
                                                planned_direction: to_absolute_coord(
                                                    planned_dest,
                                                    perspective,
                                                ),
                                            };
                                            vec![obj].into_iter()
                                        })
                                        .collect::<Vec<cetkaik_core::PureMove_<absolute::Coord>>>()[..],
                                ]
                                .concat()
                            };
                            match dest_piece {
                                None => {
                                    // cannot step
                                    ans.append(&mut vec![
                                        cetkaik_core::PureMove_::NonTamMoveSrcDst {
                                            src: to_absolute_coord(src, perspective),
                                            dest: to_absolute_coord(tentative_dest, perspective),
                                            is_water_entry_ciurl: is_ciurl_required(
                                                tentative_dest,
                                                prof,
                                                src,
                                            ),
                                        },
                                    ]);
                                }
                                Some(Piece::Tam2) => {
                                    // if allowed by config, allow stepping on Tam2;
                                    if config.allow_kut2tam2 {
                                        ans.append(&mut candidates_when_stepping());
                                    } else {
                                        ans.append(&mut vec![]);
                                    }
                                }
                                Some(Piece::NonTam2Piece {
                                    side: Side::Upward,
                                    color: _,
                                    prof: _,
                                }) => {
                                    // opponent's piece; stepping and taking both attainable

                                    // except when protected by tam2 hue a uai1
                                    if can_get_occupied_by::<CetkaikCore>(
                                        Side::Downward,
                                        tentative_dest,
                                        Piece::NonTam2Piece {
                                            color,
                                            prof,
                                            side: Side::Downward,
                                        },
                                        f.current_board,
                                        tam_itself_is_tam_hue,
                                    ) {
                                        ans.append(
                                            &mut [
                                                &[cetkaik_core::PureMove_::NonTamMoveSrcDst {
                                                    src: to_absolute_coord(src, perspective),
                                                    dest: to_absolute_coord(
                                                        tentative_dest,
                                                        perspective,
                                                    ),
                                                    is_water_entry_ciurl: is_ciurl_required(
                                                        tentative_dest,
                                                        prof,
                                                        src,
                                                    ),
                                                }][..],
                                                &candidates_when_stepping()[..],
                                            ]
                                            .concat(),
                                        );
                                    } else {
                                        ans.append(&mut candidates_when_stepping());
                                    }
                                }
                                Some(_) => {
                                    ans.append(&mut candidates_when_stepping());
                                }
                            }
                        }
                    }
                    Piece::NonTam2Piece {
                        side: Side::Upward, ..
                    } => {}
                }
            }
        }
    }

    ans
}

use cetkaik_core::relative::{is_water, Coord, Piece, Side};

pub use cetkaik_core::absolute;

pub mod pure_move;

pub struct Config {
    pub allow_kut2tam2: bool,
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
