use super::*;

pub fn complicatedBoard() -> Board {
    [
        [
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kaun1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kua2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            None,
            None,
        ],
        [
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Dau2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Dau2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Uai1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Dau2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Uai1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Gua2,
                side: Side::Upward,
            }),
        ],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kaun1,
                side: Side::Downward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Maun1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Maun1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Io,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Gua2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Uai1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Tuk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kua2,
                side: Side::Upward,
            }),
        ],
        [
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Tuk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Tuk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Nuak1,
                side: Side::Downward,
            }),
        ],
        [
            None,
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kua2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Uai1,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Tuk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            None,
        ],
        [
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Nuak1,
                side: Side::Downward,
            }),
            None,
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Maun1,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Gua2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Maun1,
                side: Side::Upward,
            }),
        ],
        [
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kaun1,
                side: Side::Upward,
            }),
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Gua2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kua2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Io,
                side: Side::Downward,
            }),
            None,
        ],
        [
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            None,
            Some(Piece::Tam2),
        ],
        [
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kaun1,
                side: Side::Downward,
            }),
            None,
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Kok1,
                prof: Profession::Kauk2,
                side: Side::Upward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Dau2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            None,
            None,
        ],
    ]
}

pub fn tamCorner() -> Board {
    [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
        ],
        [
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
        ],
        [
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::NonTam2Piece {
                color: Color::Huok2,
                prof: Profession::Kauk2,
                side: Side::Downward,
            }),
            Some(Piece::Tam2),
        ],
    ]
}

pub fn simpleBoardSample_1() -> PureGameState {
    PureGameState {
        IA_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![],
            hop1zuo1OfUpward: vec![],
            currentBoard: simpleBoard,
        },
    }
}

pub fn simpleBoardSample_2() -> PureGameState {
    PureGameState {
        IA_is_down: false,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![],
            hop1zuo1OfUpward: vec![],
            currentBoard: simpleBoard,
        },
    }
}

pub fn simpleBoardSample_3() -> PureGameState {
    PureGameState {
        IA_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![],
            hop1zuo1OfUpward: vec![],
            currentBoard: rotateBoard(simpleBoard),
        },
    }
}

pub fn simpleBoardSample_4() -> PureGameState {
    PureGameState {
        IA_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![NonTam2PieceDownward {
                color: Color::Huok2,
                prof: Profession::Gua2,
            }],
            hop1zuo1OfUpward: vec![NonTam2PieceUpward {
                color: Color::Kok1,
                prof: Profession::Kauk2,
            }],
            currentBoard: rotateBoard(simpleBoard),
        },
    }
}

pub fn complicatedBoardSample_1() -> PureGameState {
    PureGameState {
        IA_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![],
            hop1zuo1OfUpward: vec![],
            currentBoard: complicatedBoard(),
        },
    }
}

pub fn complicatedBoardSample_2() -> PureGameState {
    PureGameState {
        IA_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![],
            hop1zuo1OfUpward: vec![],
            currentBoard: rotateBoard(complicatedBoard()),
        },
    }
}

pub fn tamCornerSample() -> PureGameState {
    PureGameState {
        IA_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1OfDownward: vec![],
            hop1zuo1OfUpward: vec![],
            currentBoard: tamCorner(),
        },
    }
}
