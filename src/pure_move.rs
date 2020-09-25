use super::*;
use cetkaik_core::{serialize_prof, serialize_color};

#[derive(Clone, Copy)]
pub enum PureMove {
    NonTamMoveSrcDst {
        src: absolute::Coord,
        dest: absolute::Coord,
        is_water_entry_ciurl: bool,
    },

    NonTamMoveSrcStepDstFinite {
        src: absolute::Coord,
        step: absolute::Coord,
        dest: absolute::Coord,
        is_water_entry_ciurl: bool,
    },
    InfAfterStep {
        src: absolute::Coord,
        step: absolute::Coord,
        planned_direction: absolute::Coord,
    },
    NonTamMoveFromHand {
        color: Color,
        prof: Profession,
        dest: absolute::Coord,
    },
    TamMoveNoStep {
        src: absolute::Coord,
        first_dest: absolute::Coord,
        second_dest: absolute::Coord,
    },
    TamMoveStepsDuringFormer {
        src: absolute::Coord,
        step: absolute::Coord,
        first_dest: absolute::Coord,
        second_dest: absolute::Coord,
    },
    TamMoveStepsDuringLatter {
        src: absolute::Coord,
        step: absolute::Coord,
        first_dest: absolute::Coord,
        second_dest: absolute::Coord,
    },
}

/// Serializes [`PureMove`](./enum.PureMove.html) in textual form.
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::pure_move::*;
/// use cetkaik_core::*;
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::InfAfterStep {
///     src: (absolute::Row::A, absolute::Column::Z),
///     step: (absolute::Row::E, absolute::Column::T),
///     planned_direction: (absolute::Row::E, absolute::Column::N)
/// }), "ZA片TE心NE");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::NonTamMoveFromHand {
///     color: Color::Huok2,
///     prof: Profession::Gua2,
///     dest: (absolute::Row::IA, absolute::Column::L)
/// }), "黒弓LIA");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::NonTamMoveSrcDst {
///     src: (absolute::Row::A, absolute::Column::Z),
///     dest: (absolute::Row::E, absolute::Column::N),
///     is_water_entry_ciurl: true
/// }), "ZA片NE水");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::NonTamMoveSrcStepDstFinite {
///     src: (absolute::Row::A, absolute::Column::Z),
///     step: (absolute::Row::E, absolute::Column::T),
///     dest: (absolute::Row::E, absolute::Column::N),
///     is_water_entry_ciurl: false
/// }), "ZA片TENE");
///
/// // Note that [] denotes the first destination.
/// // Since the first destination is neither the stepping square nor the final square,
/// // it is not to be written in the standard notation.
/// // Hence this additional information is denoted by [].
/// assert_eq!(serialize_pure_opponent_move(PureMove::TamMoveStepsDuringFormer {
///     src: (absolute::Row::E, absolute::Column::K),
///     step: (absolute::Row::I, absolute::Column::L),
///     first_dest: (absolute::Row::I, absolute::Column::K),
///     second_dest: (absolute::Row::E, absolute::Column::L)
/// }), "KE皇LI[KI]LE");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::TamMoveNoStep {
///     src: (absolute::Row::E, absolute::Column::K),
///     first_dest: (absolute::Row::I, absolute::Column::K),
///     second_dest: (absolute::Row::E, absolute::Column::K)
/// }), "KE皇[KI]KE");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::TamMoveStepsDuringLatter {
///     src: (absolute::Row::E, absolute::Column::K),
///     first_dest: (absolute::Row::I, absolute::Column::K),
///     step: (absolute::Row::I, absolute::Column::L),
///     second_dest: (absolute::Row::E, absolute::Column::L)
/// }), "KE皇[KI]LILE");
/// ```
pub fn serialize_pure_opponent_move(mov: PureMove) -> String {
    match mov {
        PureMove::InfAfterStep {
            src,
            step,
            planned_direction,
        } => format!(
            "{}片{}心{}",
            absolute::serialize_coord(src),
            absolute::serialize_coord(step),
            absolute::serialize_coord(planned_direction)
        ),
        PureMove::NonTamMoveFromHand { color, prof, dest } => format!(
            "{}{}{}",
            serialize_color(color),
            serialize_prof(prof),
            absolute::serialize_coord(dest)
        ),
        PureMove::NonTamMoveSrcDst {
            src,
            dest,
            is_water_entry_ciurl,
        } => format!(
            "{}片{}{}",
            absolute::serialize_coord(src),
            absolute::serialize_coord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureMove::NonTamMoveSrcStepDstFinite {
            src,
            dest,
            is_water_entry_ciurl,
            step,
        } => format!(
            "{}片{}{}{}",
            absolute::serialize_coord(src),
            absolute::serialize_coord(step),
            absolute::serialize_coord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureMove::TamMoveNoStep {
            src,
            first_dest,
            second_dest,
        } => format!(
            "{}皇[{}]{}",
            absolute::serialize_coord(src),
            absolute::serialize_coord(first_dest),
            absolute::serialize_coord(second_dest)
        ),
        PureMove::TamMoveStepsDuringFormer {
            src,
            first_dest,
            second_dest,
            step,
        } => format!(
            "{}皇{}[{}]{}",
            absolute::serialize_coord(src),
            absolute::serialize_coord(step),
            absolute::serialize_coord(first_dest),
            absolute::serialize_coord(second_dest)
        ),
        PureMove::TamMoveStepsDuringLatter {
            src,
            first_dest,
            second_dest,
            step,
        } => format!(
            "{}皇[{}]{}{}",
            absolute::serialize_coord(src),
            absolute::serialize_coord(first_dest),
            absolute::serialize_coord(step),
            absolute::serialize_coord(second_dest)
        ),
    }
}
