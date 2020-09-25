use super::*;

pub fn serializeCoord(coord: Coord) -> String {
    format!("[{},{}]", coord[0], coord[1])
}

pub fn serializeAbsoluteCoord(coord: AbsoluteCoord) -> String {
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

pub fn serializeProf(prof: Profession) -> &'static str {
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

pub fn serializeColor(color: Color) -> &'static str {
    match color {
        Color::Huok2 => "黒",
        Color::Kok1 => "赤",
    }
}

pub fn serializeSide(side: Side) -> &'static str {
    match side {
        Side::Upward => "↑",
        Side::Downward => "↓",
    }
}

pub fn serializePiece(p: Piece) -> String {
    match p {
        Piece::Tam2 => "皇".to_string(),
        Piece::NonTam2Piece { prof, color, side } => format!(
            "{}{}{}",
            serializeColor(color),
            serializeProf(prof),
            serializeSide(side)
        ),
    }
}

pub fn serializeRotated(r: Rotated) -> String {
    format!(
        "{}{}",
        serializeCoord(r.rotated_coord),
        serializePiece(r.rotated_piece)
    )
}

pub fn serializePureOpponentMove(mov: PureOpponentMove) -> String {
    match mov {
        PureOpponentMove::InfAfterStep {
            src,
            step,
            plannedDirection,
        } => format!(
            "{}片{}心{}",
            serializeAbsoluteCoord(src),
            serializeAbsoluteCoord(step),
            serializeAbsoluteCoord(plannedDirection)
        ),
        PureOpponentMove::NonTamMoveFromHand { color, prof, dest } => format!(
            "{}{}{}",
            serializeColor(color),
            serializeProf(prof),
            serializeAbsoluteCoord(dest)
        ),
        PureOpponentMove::PotentialWaterEntry(
            PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcDst {
                src,
                dest,
                is_water_entry_ciurl,
            },
        ) => format!(
            "{}片{}{}",
            serializeAbsoluteCoord(src),
            serializeAbsoluteCoord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureOpponentMove::PotentialWaterEntry(
            PureOpponentMoveWithPotentialWaterEntry::NonTamMoveSrcStepDstFinite {
                src,
                dest,
                is_water_entry_ciurl,
                step,
            },
        ) => format!(
            "{}片{}{}{}",
            serializeAbsoluteCoord(src),
            serializeAbsoluteCoord(step),
            serializeAbsoluteCoord(dest),
            if is_water_entry_ciurl { "水" } else { "" }
        ),
        PureOpponentMove::TamMoveNoStep {
            src,
            firstDest,
            secondDest,
        } => format!(
            "{}皇{}{}",
            serializeAbsoluteCoord(src),
            serializeAbsoluteCoord(firstDest),
            serializeAbsoluteCoord(secondDest)
        ),
        PureOpponentMove::TamMoveStepsDuringFormer {
            src,
            firstDest,
            secondDest,
            step,
        } => format!(
            "{}皇{}[{}]{}",
            serializeAbsoluteCoord(src),
            serializeAbsoluteCoord(step),
            serializeAbsoluteCoord(firstDest),
            serializeAbsoluteCoord(secondDest)
        ),
        PureOpponentMove::TamMoveStepsDuringLatter {
            src,
            firstDest,
            secondDest,
            step,
        } => format!(
            "{}皇[{}]{}{}",
            serializeAbsoluteCoord(src),
            serializeAbsoluteCoord(firstDest),
            serializeAbsoluteCoord(step),
            serializeAbsoluteCoord(secondDest)
        ),
    }
}
