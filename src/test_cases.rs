use super::*;

pub fn complicated_board() -> Board {
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

pub fn tam_corner() -> Board {
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

pub fn simple_board_sample_1() -> PureGameState {
    PureGameState {
        ia_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: SIMPLE_BOARD,
        },
    }
}

pub fn simple_board_sample_2() -> PureGameState {
    PureGameState {
        ia_is_down: false,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: SIMPLE_BOARD,
        },
    }
}

pub fn simple_board_sample_3() -> PureGameState {
    PureGameState {
        ia_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: rotate_board(SIMPLE_BOARD),
        },
    }
}

pub fn simple_board_sample_4() -> PureGameState {
    PureGameState {
        ia_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![NonTam2PieceDownward {
                color: Color::Huok2,
                prof: Profession::Gua2,
            }],
            hop1zuo1of_upward: vec![NonTam2PieceUpward {
                color: Color::Kok1,
                prof: Profession::Kauk2,
            }],
            current_board: rotate_board(SIMPLE_BOARD),
        },
    }
}

pub fn complicated_board_sample_1() -> PureGameState {
    PureGameState {
        ia_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: complicated_board(),
        },
    }
}

pub fn complicated_board_sample_2() -> PureGameState {
    PureGameState {
        ia_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: rotate_board(complicated_board()),
        },
    }
}

pub fn tam_corner_sample() -> PureGameState {
    PureGameState {
        ia_is_down: true,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: tam_corner(),
        },
    }
}
