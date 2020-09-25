use super::*;

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

/// Serializes [`AbsoluteCoord`](../type.AbsoluteCoord.html).
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
///
/// assert_eq!(serialize_absolute_coord((AbsoluteRow::E, AbsoluteColumn::N)), "NE");
/// assert_eq!(serialize_absolute_coord((AbsoluteRow::AU, AbsoluteColumn::Z)), "ZAU");
/// ```
///
pub fn serialize_absolute_coord(coord: AbsoluteCoord) -> String {
    let (row, column) = coord;
    format!(
        "{}{}",
        match column {
            AbsoluteColumn::K => "K",
            AbsoluteColumn::L => "L",
            AbsoluteColumn::M => "M",
            AbsoluteColumn::N => "N",
            AbsoluteColumn::P => "P",
            AbsoluteColumn::Z => "Z",
            AbsoluteColumn::X => "X",
            AbsoluteColumn::C => "C",
            AbsoluteColumn::T => "T",
        },
        match row {
            AbsoluteRow::A => "A",
            AbsoluteRow::E => "E",
            AbsoluteRow::I => "I",
            AbsoluteRow::O => "O",
            AbsoluteRow::U => "U",
            AbsoluteRow::Y => "Y",
            AbsoluteRow::IA => "IA",
            AbsoluteRow::AI => "AI",
            AbsoluteRow::AU => "AU",
        }
    )
}

/// Serializes [`Profession`](../enum.Profession.html).
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
///
/// assert_eq!(serialize_prof(Profession::Nuak1), "船");
/// assert_eq!(serialize_prof(Profession::Kaun1), "車");
/// ```
///
pub fn serialize_prof(prof: Profession) -> &'static str {
    match prof {
        Profession::Nuak1 => "船",
        Profession::Kauk2 => "兵",
        Profession::Gua2 => "弓",
        Profession::Kaun1 => "車",
        Profession::Dau2 => "虎",
        Profession::Maun1 => "馬",
        Profession::Kua2 => "筆",
        Profession::Tuk2 => "巫",
        Profession::Uai1 => "将",
        Profession::Io => "王",
    }
}

/// Serializes [`Color`](../enum.Color.html).
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
///
/// assert_eq!(serialize_color(Color::Kok1), "赤");
/// assert_eq!(serialize_color(Color::Huok2), "黒");
/// ```
///
pub fn serialize_color(color: Color) -> &'static str {
    match color {
        Color::Huok2 => "黒",
        Color::Kok1 => "赤",
    }
}

fn serialize_side(side: Side) -> &'static str {
    match side {
        Side::Upward => "↑",
        Side::Downward => "↓",
    }
}

/// Serializes [`Piece`](../enum.Piece.html).
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
///
/// assert_eq!(serialize_piece(Piece::Tam2), "皇");
/// assert_eq!(serialize_piece(Piece::NonTam2Piece {
///     prof: Profession::Uai1,
///     color: Color::Kok1,
///     side: Side::Downward
/// }), "赤将↓");
/// ```
pub fn serialize_piece(p: Piece) -> String {
    match p {
        Piece::Tam2 => "皇".to_string(),
        Piece::NonTam2Piece { prof, color, side } => format!(
            "{}{}{}",
            serialize_color(color),
            serialize_prof(prof),
            serialize_side(side)
        ),
    }
}

/// Serializes [`PureMove`](../enum.PureMove.html) in textual form.
/// # Examples
/// ```
/// use cerke_rust::*;
/// use cerke_rust::serialize::*;
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::InfAfterStep {
///     src: (AbsoluteRow::A, AbsoluteColumn::Z),
///     step: (AbsoluteRow::E, AbsoluteColumn::T),
///     planned_direction: (AbsoluteRow::E, AbsoluteColumn::N)
/// }), "ZA片TE心NE");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::NonTamMoveFromHand {
///     color: Color::Huok2,
///     prof: Profession::Gua2,
///     dest: (AbsoluteRow::IA, AbsoluteColumn::L)
/// }), "黒弓LIA");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::NonTamMoveSrcDst {
///     src: (AbsoluteRow::A, AbsoluteColumn::Z),
///     dest: (AbsoluteRow::E, AbsoluteColumn::N),
///     is_water_entry_ciurl: true
/// }), "ZA片NE水");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::NonTamMoveSrcStepDstFinite {
///     src: (AbsoluteRow::A, AbsoluteColumn::Z),
///     step: (AbsoluteRow::E, AbsoluteColumn::T),
///     dest: (AbsoluteRow::E, AbsoluteColumn::N),
///     is_water_entry_ciurl: false
/// }), "ZA片TENE");
///
/// // Note that [] denotes the first destination.
/// // Since the first destination is neither the stepping square nor the final square,
/// // it is not to be written in the standard notation.
/// // Hence this additional information is denoted by [].
/// assert_eq!(serialize_pure_opponent_move(PureMove::TamMoveStepsDuringFormer {
///     src: (AbsoluteRow::E, AbsoluteColumn::K),
///     step: (AbsoluteRow::I, AbsoluteColumn::L),
///     first_dest: (AbsoluteRow::I, AbsoluteColumn::K),
///     second_dest: (AbsoluteRow::E, AbsoluteColumn::L)
/// }), "KE皇LI[KI]LE");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::TamMoveNoStep {
///     src: (AbsoluteRow::E, AbsoluteColumn::K),
///     first_dest: (AbsoluteRow::I, AbsoluteColumn::K),
///     second_dest: (AbsoluteRow::E, AbsoluteColumn::K)
/// }), "KE皇[KI]KE");
///
/// assert_eq!(serialize_pure_opponent_move(PureMove::TamMoveStepsDuringLatter {
///     src: (AbsoluteRow::E, AbsoluteColumn::K),
///     first_dest: (AbsoluteRow::I, AbsoluteColumn::K),
///     step: (AbsoluteRow::I, AbsoluteColumn::L),
///     second_dest: (AbsoluteRow::E, AbsoluteColumn::L)
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
