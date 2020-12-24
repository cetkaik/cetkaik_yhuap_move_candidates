use super::{absolute, Color, Profession};
use cetkaik_core::{serialize_color, serialize_prof};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl PureMove {
    /// Serializes [`PureMove`](./enum.PureMove.html) in textual form.
    /// # Examples
    /// ```
    /// use cetkaik_yhuap_move_candidates::*;
    /// use cetkaik_yhuap_move_candidates::pure_move::*;
    /// use cetkaik_core::*;
    ///
    /// assert_eq!(PureMove::InfAfterStep {
    ///     src: (absolute::Row::A, absolute::Column::Z),
    ///     step: (absolute::Row::E, absolute::Column::T),
    ///     planned_direction: (absolute::Row::E, absolute::Column::N)
    /// }.serialize(), "ZA片TE心NE");
    ///
    /// assert_eq!(PureMove::NonTamMoveFromHand {
    ///     color: Color::Huok2,
    ///     prof: Profession::Gua2,
    ///     dest: (absolute::Row::IA, absolute::Column::L)
    /// }.serialize(), "黒弓LIA");
    ///
    /// assert_eq!(PureMove::NonTamMoveSrcDst {
    ///     src: (absolute::Row::A, absolute::Column::Z),
    ///     dest: (absolute::Row::E, absolute::Column::N),
    ///     is_water_entry_ciurl: true
    /// }.serialize(), "ZA片NE水");
    ///
    /// assert_eq!(PureMove::NonTamMoveSrcStepDstFinite {
    ///     src: (absolute::Row::A, absolute::Column::Z),
    ///     step: (absolute::Row::E, absolute::Column::T),
    ///     dest: (absolute::Row::E, absolute::Column::N),
    ///     is_water_entry_ciurl: false
    /// }.serialize(), "ZA片TENE");
    ///
    /// // Note that [] denotes the first destination.
    /// // Since the first destination is neither the stepping square nor the final square,
    /// // it is not to be written in the standard notation.
    /// // Hence this additional information is denoted by [].
    /// assert_eq!(PureMove::TamMoveStepsDuringFormer {
    ///     src: (absolute::Row::E, absolute::Column::K),
    ///     step: (absolute::Row::I, absolute::Column::L),
    ///     first_dest: (absolute::Row::I, absolute::Column::K),
    ///     second_dest: (absolute::Row::E, absolute::Column::L)
    /// }.serialize(), "KE皇LI[KI]LE");
    ///
    /// assert_eq!(PureMove::TamMoveNoStep {
    ///     src: (absolute::Row::E, absolute::Column::K),
    ///     first_dest: (absolute::Row::I, absolute::Column::K),
    ///     second_dest: (absolute::Row::E, absolute::Column::K)
    /// }.serialize(), "KE皇[KI]KE");
    ///
    /// assert_eq!(PureMove::TamMoveStepsDuringLatter {
    ///     src: (absolute::Row::E, absolute::Column::K),
    ///     first_dest: (absolute::Row::I, absolute::Column::K),
    ///     step: (absolute::Row::I, absolute::Column::L),
    ///     second_dest: (absolute::Row::E, absolute::Column::L)
    /// }.serialize(), "KE皇[KI]LILE");
    /// ```
    #[must_use]
    pub fn serialize(self) -> String {
        match self {
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
}
