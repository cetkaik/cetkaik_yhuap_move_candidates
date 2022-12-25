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

use cetkaik_fundamental::{AbsoluteSide, PureMove_};
use cetkaik_traits::CetkaikRepresentation;

/// # Example
///
/// Using `cetkaik_naive_representation`:
/// ```
/// use cetkaik_yhuap_move_candidates::from_hop1zuo1_candidates_vec;
/// use cetkaik_naive_representation::*;
/// use cetkaik_naive_representation::absolute::Field;
/// use cetkaik_fundamental::*;
/// use cetkaik_naive_representation::absolute::Coord;
/// use cetkaik_naive_representation::absolute::Column::*;
/// use cetkaik_naive_representation::absolute::Row::*;
/// use cetkaik_naive_representation::CetkaikNaive;
/// use cetkaik_naive_representation::absolute::Board;
/// use std::collections::HashSet;
///
/// // There are eighty unoccupied squares on the board, and `IASide` has two pieces in hop1zuo1
/// let vec = from_hop1zuo1_candidates_vec::<CetkaikNaive>(
///     cetkaik_fundamental::AbsoluteSide::IASide,
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
///         board: Board(vec![
///             (Coord(AU, C), absolute::Piece::NonTam2Piece {
///             color: Color::Kok1,
///             prof: Profession::Nuak1,
///             side: AbsoluteSide::IASide
///         })
///         ]
///         .into_iter()
///         .collect()),
///     }
/// );
///
/// assert_eq!(vec.len(), 80 * 2)
///
/// ```
#[must_use]
pub fn from_hop1zuo1_candidates_vec<T: CetkaikRepresentation>(
    whose_turn: AbsoluteSide,
    field: &T::AbsoluteField,
) -> Vec<PureMove_<T::AbsoluteCoord>> {
    T::hop1zuo1_of(whose_turn, field)
        .into_iter()
        .flat_map(|cetkaik_fundamental::ColorAndProf { color, prof }| {
            T::empty_squares_absolute(T::as_board_absolute(field))
                .into_iter()
                .map(move |dest| PureMove_::NonTamMoveFromHopZuo { color, prof, dest })
        })
        .collect()
}

mod calculate_movable;
pub use calculate_movable::calculate_movable_positions_for_either_side;
pub use calculate_movable::is_tam_hue_relative;

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
        .filter(move |coord| board.peek(*coord).is_none())
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
                board.peek(*ab).map_or(false, |piece| {
                    piece.match_on_piece_and_apply(&|| false, &|_, piece_prof, piece_side| {
                        piece_prof == Profession::Uai1
                            && piece_side != side
                            && calculate_movable::is_tam_hue_relative::<T>(
                                *ab,
                                board,
                                tam_itself_is_tam_hue,
                            )
                    })
                })
            })
            .count()
            > 0
    };

    board.peek(dest).map_or(true, |piece| {
        piece.match_on_piece_and_apply(
            &|| false, /* Tam2 can never be taken */
            &|_, _, piece_side| {
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
/// use cetkaik_naive_representation::CetkaikNaive;
/// use cetkaik_naive_representation::*;
/// use cetkaik_naive_representation::absolute::*;
/// use cetkaik_naive_representation::absolute::Row::*;
/// use cetkaik_naive_representation::absolute::Column::*;
/// use cetkaik_fundamental::*;
/// use cetkaik_fundamental::PureMove_::*;
/// use PureMove_::*;
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
///     &not_from_hop1zuo1_candidates_vec::<CetkaikNaive>(
///         &Config {
///             allow_kut2tam2: false,
///         },
///         false,
///         AbsoluteSide::IASide,
///         &absolute::Field {
///              a_side_hop1zuo1: vec![],
///              ia_side_hop1zuo1: vec![],
///              board: Board(vec![
///                  (absolute::Coord(AU, C), absolute::Piece::NonTam2Piece {
///                      color: Color::Kok1,
///                      prof: Profession::Nuak1,
///                      side: AbsoluteSide::IASide
///                  })
///             ]
///             .into_iter()
///             .collect())
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
///     &not_from_hop1zuo1_candidates_vec::<CetkaikNaive>(
///         &Config {
///             allow_kut2tam2: false,
///         },
///         false,
///         AbsoluteSide::IASide,
///         &absolute::Field {
///              a_side_hop1zuo1: vec![],
///              ia_side_hop1zuo1: vec![],
///              board: Board(vec![
///                  (absolute::Coord(AI, L), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Gua2,
///                      side: AbsoluteSide::IASide
///                  }),
///                  (absolute::Coord(I, L), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Kauk2,
///                      side: AbsoluteSide::ASide
///                  }),
///                  (absolute::Coord(I, N), absolute::Piece::NonTam2Piece {
///                      color: Color::Kok1,
///                      prof: Profession::Uai1,
///                      side: AbsoluteSide::ASide
///                  }),
///                  (absolute::Coord(AI, N), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Kaun1,
///                      side: AbsoluteSide::ASide
///                  }),
///                  (absolute::Coord(AU, L), absolute::Piece::NonTam2Piece {
///                      color: Color::Huok2,
///                      prof: Profession::Kauk2,
///                      side: AbsoluteSide::ASide
///                  }),
///             ]
///             .into_iter()
///             .collect())
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
    whose_turn: AbsoluteSide,
    f: &T::AbsoluteField,
) -> Vec<PureMove_<T::AbsoluteCoord>> {
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
        let dest_piece = T::as_board_relative(f).peek(tentative_dest);

        /* avoid self-occlusion */
        let mut subtracted_board = *T::as_board_relative(f);
        subtracted_board.put(src, None);
        if dest_piece.is_none() {
            /* empty square; first move is completed without stepping */
            let fst_dst: T::RelativeCoord = tentative_dest;
            ans.append(&mut calculate_movable::iter::eight_neighborhood::<T>(fst_dst).flat_map(|neighbor| {
                            /* if the neighbor is empty, that is the second destination */
                            let snd_dst: T::RelativeCoord = neighbor;
                            if T::as_board_relative(f).peek(neighbor).is_none() /* the neighbor is utterly occupied */ ||
                                neighbor == src
                            /* the neighbor is occupied by yourself, which means it is actually empty */
                            {
                                vec![PureMove_::TamMoveNoStep {
                                    second_dest: T::to_absolute_coord(snd_dst, perspective),
                                    first_dest: T::to_absolute_coord(fst_dst, perspective),
                                    src: T::to_absolute_coord(src, perspective),
                                }].into_iter()
                            } else {
                                /* if not, step from there */
                                let step: T::RelativeCoord = neighbor;
                                empty_neighbors_of::<T>(subtracted_board, step)
                                    .flat_map(|snd_dst| {
                                    vec![PureMove_::TamMoveStepsDuringLatter {
                                        first_dest: T::to_absolute_coord(fst_dst, perspective),
                                        second_dest: T::to_absolute_coord(snd_dst, perspective),
                                        src: T::to_absolute_coord(src, perspective),
                                        step: T::to_absolute_coord(step, perspective),
                                    }].into_iter()
                                }).collect::<Vec<PureMove_<T::AbsoluteCoord>>>().into_iter()
                            }
                        }).collect::<Vec<PureMove_<T::AbsoluteCoord>>>());
        } else {
            /* not an empty square: must complete the first move */
            let step = tentative_dest;
            ans.append(
                &mut empty_neighbors_of::<T>(subtracted_board, step)
                    .flat_map(|fst_dst| {
                        let v = empty_neighbors_of::<T>(subtracted_board, fst_dst);
                        v.flat_map(move |snd_dst| {
                            vec![PureMove_::TamMoveStepsDuringFormer {
                                first_dest: T::to_absolute_coord(fst_dst, perspective),
                                second_dest: T::to_absolute_coord(snd_dst, perspective),
                                src: T::to_absolute_coord(src, perspective),
                                step: T::to_absolute_coord(step, perspective),
                            }]
                            .into_iter()
                        })
                        .collect::<Vec<PureMove_<T::AbsoluteCoord>>>()
                        .into_iter()
                    })
                    .collect::<Vec<PureMove_<T::AbsoluteCoord>>>(),
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
        let dest_piece = T::as_board_relative(field).peek(tentative_dest);

        let candidates_when_stepping = || {
            let step = tentative_dest; // tentative_dest becomes the position on which the stepping occurs

            let perspective = perspective;
            let tam_itself_is_tam_hue: bool = config.tam_itself_is_tam_hue;
            /* now, to decide the final position, we must remove the piece to prevent self-occlusion */
            let mut subtracted_board = *T::as_board_relative(field);
            subtracted_board.put(src, None); /* must remove the piece to prevent self-occlusion */

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
                        vec![PureMove_::NonTamMoveSrcStepDstFinite {
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
                .collect::<Vec<PureMove_<T::AbsoluteCoord>>>();
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
                    let obj: PureMove_<T::AbsoluteCoord> = PureMove_::InfAfterStep {
                        src: T::to_absolute_coord(src, perspective),
                        step: T::to_absolute_coord(step, perspective),
                        planned_direction: T::to_absolute_coord(planned_dest, perspective),
                    };
                    vec![obj].into_iter()
                })
                .collect::<Vec<PureMove_<T::AbsoluteCoord>>>();
            [&candidates_abs[..], &candidates_inf_abs[..]].concat()
        };
        if let Some(piece) = dest_piece {
            let mut a = piece.match_on_piece_and_apply(
                &|| {
                    // if allowed by config, allow stepping on Tam2;
                    if config.allow_kut2tam2 {
                        candidates_when_stepping()
                    } else {
                        vec![]
                    }
                },
                &|_color, prof, side_| {
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
                                &[PureMove_::NonTamMoveSrcDst {
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
            ans.append(&mut vec![PureMove_::NonTamMoveSrcDst {
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
) -> Vec<PureMove_<T::AbsoluteCoord>> {
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

pub use cetkaik_fundamental::{Color, Profession};
use cetkaik_traits::IsBoard;
use cetkaik_traits::IsPieceWithSide;

/// According to <https://github.com/cetkaik/cetkaik_yhuap_move_candidates/pull/7>,
/// this function was a bottleneck that accounted for roughly fifty percent of all the runtime
/// in an attempt to implement an AI for this game.
/// Hence, this module contains multiple implementations of this functions and test that they are equal.
#[allow(clippy::similar_names)]
pub mod get_blocker_deltas;
