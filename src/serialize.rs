use super::*;
use cetkaik_core::{serialize_prof, serialize_color};

/// Serializes [`Coord`](../type.Coord.html) in JSON-style.
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
///
/// assert_eq!(serialize_coord([5,6]), "[5,6]")
/// ```
pub fn serialize_coord(coord: Coord) -> String {
    format!("[{},{}]", coord[0], coord[1])
}

/// Serializes [`absolute::Coord`](../type.absolute::Coord.html).
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
/// use cetkaik_core::*;
///
/// assert_eq!(serialize_absolute_coord((absolute::Row::E, absolute::Column::N)), "NE");
/// assert_eq!(serialize_absolute_coord((absolute::Row::AU, absolute::Column::Z)), "ZAU");
/// ```
///
pub fn serialize_absolute_coord(coord: absolute::Coord) -> String {
    let (row, column) = coord;
    format!(
        "{}{}",
        match column {
            absolute::Column::K => "K",
            absolute::Column::L => "L",
            absolute::Column::M => "M",
            absolute::Column::N => "N",
            absolute::Column::P => "P",
            absolute::Column::Z => "Z",
            absolute::Column::X => "X",
            absolute::Column::C => "C",
            absolute::Column::T => "T",
        },
        match row {
            absolute::Row::A => "A",
            absolute::Row::E => "E",
            absolute::Row::I => "I",
            absolute::Row::O => "O",
            absolute::Row::U => "U",
            absolute::Row::Y => "Y",
            absolute::Row::IA => "IA",
            absolute::Row::AI => "AI",
            absolute::Row::AU => "AU",
        }
    )
}

/// Serializes [`PureMove`](../enum.PureMove.html) in textual form.
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
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
            serialize_absolute_coord(src),
            serialize_absolute_coord(step),
            serialize_absolute_coord(planned_direction)
        ),
        PureMove::NonTamMoveFromHand { color, prof, dest } => format!(
            "{}{}{}",
            serialize_color(color),
            serialize_prof(prof),
            serialize_absolute_coord(dest)
        ),
        PureMove::NonTamMoveSrcDst {
            src,
            dest,
            is_water_entry_ciurl,
        } => format!(
            "{}片{}{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureMove::NonTamMoveSrcStepDstFinite {
            src,
            dest,
            is_water_entry_ciurl,
            step,
        } => format!(
            "{}片{}{}{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(step),
            serialize_absolute_coord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureMove::TamMoveNoStep {
            src,
            first_dest,
            second_dest,
        } => format!(
            "{}皇[{}]{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(first_dest),
            serialize_absolute_coord(second_dest)
        ),
        PureMove::TamMoveStepsDuringFormer {
            src,
            first_dest,
            second_dest,
            step,
        } => format!(
            "{}皇{}[{}]{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(step),
            serialize_absolute_coord(first_dest),
            serialize_absolute_coord(second_dest)
        ),
        PureMove::TamMoveStepsDuringLatter {
            src,
            first_dest,
            second_dest,
            step,
        } => format!(
            "{}皇[{}]{}{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(first_dest),
            serialize_absolute_coord(step),
            serialize_absolute_coord(second_dest)
        ),
    }
}
