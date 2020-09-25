use super::*;

pub fn serialize_coord(coord: Coord) -> String {
    format!("[{},{}]", coord[0], coord[1])
}

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

pub fn serialize_color(color: Color) -> &'static str {
    match color {
        Color::Huok2 => "黒",
        Color::Kok1 => "赤",
    }
}

pub fn serialize_side(side: Side) -> &'static str {
    match side {
        Side::Upward => "↑",
        Side::Downward => "↓",
    }
}

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

pub fn serialize_pure_opponent_move(mov: PureOpponentMove) -> String {
    match mov {
        PureOpponentMove::InfAfterStep {
            src,
            step,
            planned_direction,
        } => format!(
            "{}片{}心{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(step),
            serialize_absolute_coord(planned_direction)
        ),
        PureOpponentMove::NonTamMoveFromHand { color, prof, dest } => format!(
            "{}{}{}",
            serialize_color(color),
            serialize_prof(prof),
            serialize_absolute_coord(dest)
        ),
        PureOpponentMove::NonTamMoveSrcDst {
            src,
            dest,
            is_water_entry_ciurl,
        } => format!(
            "{}片{}{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureOpponentMove::NonTamMoveSrcStepDstFinite {
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
        PureOpponentMove::TamMoveNoStep {
            src,
            first_dest,
            second_dest,
        } => format!(
            "{}皇[{}]{}",
            serialize_absolute_coord(src),
            serialize_absolute_coord(first_dest),
            serialize_absolute_coord(second_dest)
        ),
        PureOpponentMove::TamMoveStepsDuringFormer {
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
        PureOpponentMove::TamMoveStepsDuringLatter {
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
