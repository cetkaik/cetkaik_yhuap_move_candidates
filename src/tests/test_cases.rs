use super::*;

pub const INITIAL_BOARD_SAMPLE: PureGameState = PureGameState {
    perspective: Perspective::IaIsDownAndPointsUpward,
    tam_itself_is_tam_hue: true,
    opponent_has_just_moved_tam: false,
    f: Field {
        hop1zuo1of_downward: vec![],
        hop1zuo1of_upward: vec![],
        current_board: INITIAL_BOARD,
    },
};

pub const TAM_ITSELF_IS_NOT_TAM_HUE_SAMPLE: PureGameState = PureGameState {
    perspective: Perspective::IaIsDownAndPointsUpward,
    tam_itself_is_tam_hue: false,
    opponent_has_just_moved_tam: false,
    f: Field {
        hop1zuo1of_downward: vec![],
        hop1zuo1of_upward: vec![],
        current_board: INITIAL_BOARD,
    },
};

pub const SIMPLE_BOARD: cetkaik_core::relative::Board = [
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kua2,
            side: Side::Downward,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Upward,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
];

pub const fn complicated_board() -> cetkaik_core::relative::Board {
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

pub const fn tam_corner() -> cetkaik_core::relative::Board {
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

pub const fn simple_board_sample_1() -> PureGameState {
    PureGameState {
        perspective: Perspective::IaIsDownAndPointsUpward,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: SIMPLE_BOARD,
        },
    }
}

pub const fn simple_board_sample_2() -> PureGameState {
    PureGameState {
        perspective: Perspective::IaIsUpAndPointsDownward,
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
        perspective: Perspective::IaIsDownAndPointsUpward,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: rotate_board(SIMPLE_BOARD),
        },
    }
}

use cetkaik_core::relative::{NonTam2PieceDownward, rotate_board};

pub fn simple_board_sample_4() -> PureGameState {
    use cetkaik_core::relative::NonTam2PieceUpward;
    PureGameState {
        perspective: Perspective::IaIsDownAndPointsUpward,
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

pub const fn complicated_board_sample_1() -> PureGameState {
    PureGameState {
        perspective: Perspective::IaIsDownAndPointsUpward,
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
        perspective: Perspective::IaIsDownAndPointsUpward,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: rotate_board(complicated_board()),
        },
    }
}

pub const fn tam_corner_sample() -> PureGameState {
    PureGameState {
        perspective: Perspective::IaIsDownAndPointsUpward,
        tam_itself_is_tam_hue: true,
        opponent_has_just_moved_tam: false,
        f: Field {
            hop1zuo1of_downward: vec![],
            hop1zuo1of_upward: vec![],
            current_board: tam_corner(),
        },
    }
}

const INITIAL_BOARD: cetkaik_core::relative::Board = [
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kua2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Maun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kaun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Uai1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Io,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Uai1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kaun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Maun1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kua2,
            side: Side::Downward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Tuk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Gua2,
            side: Side::Downward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Dau2,
            side: Side::Downward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Dau2,
            side: Side::Downward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Gua2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Tuk2,
            side: Side::Downward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Nuak1,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kauk2,
            side: Side::Downward,
        }),
    ],
    [None, None, None, None, None, None, None, None, None],
    [
        None,
        None,
        None,
        None,
        Some(Piece::Tam2),
        None,
        None,
        None,
        None,
    ],
    [None, None, None, None, None, None, None, None, None],
    [
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
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Nuak1,
            side: Side::Upward,
        }),
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
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Tuk2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Gua2,
            side: Side::Upward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Dau2,
            side: Side::Upward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Dau2,
            side: Side::Upward,
        }),
        None,
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Gua2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Tuk2,
            side: Side::Upward,
        }),
    ],
    [
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kua2,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Maun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Kaun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Kok1,
            prof: Profession::Uai1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Io,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Uai1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kaun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Maun1,
            side: Side::Upward,
        }),
        Some(Piece::NonTam2Piece {
            color: Color::Huok2,
            prof: Profession::Kua2,
            side: Side::Upward,
        }),
    ],
];

pub const INITIAL_MOVES_NO_KUT_TAM: [&str; 240] = [
    "KA片LAKA",
    "KA片KE心KA", /* 黒筆 */
    "LA片TIXO水",
    "LA片TILO",
    "LA片TILA", /* 黒馬 */
    "NA片NIKO",
    "NA片ZANA",
    "NA片KANA", /* 黒車 */
    "TA片ZE",
    "TA片NE", /* 黒将不踏 */
    "TA片NANE",
    "TA片TENE",
    "TA片TEZE",
    "TA片ZAZE",
    "TA片ZATA",
    "TA片NATA", /* 黒将踏 */
    "ZA片ZE",   /* 赤王不踏 */
    "ZA片XECE",
    "ZA片XACE", /* 赤王踏而CE */
    "ZA片TEZA",
    "ZA片TAZA",
    "ZA片XAZA",
    "ZA片XEZA", /* 赤王踏而ZA */
    "ZA片TEZE",
    "ZA片TAZE",
    "ZA片XAZE",
    "ZA片XEZE", /* 赤王踏而ZE */
    "ZA片TENE",
    "ZA片TANE", /* 赤王踏而NE */
    "XA片CE",
    "XA片ZE", /* 赤将不踏 */
    "XA片XECE",
    "XA片XEZE",
    "XA片CACE",
    "XA片CAXA",
    "XA片ZAZE",
    "XA片ZAXA", /* 赤将踏 */
    "CA片CIPO",
    "CA片PACA",
    "CA片ZACA", /* 赤車 */
    "MA片XIMO",
    "MA片XITO水",
    "MA片XIMA", /* 赤馬 */
    "PA片MAPA",
    "PA片PE心PA", /* 赤筆 */
    "KE片KIKU",
    "KE片KIKE",
    "KE片KAKE",
    "KE片LE心NE",
    "KE片LE心KE", /* 赤巫 */
    "LE片NE",      /* 赤弓不踏 */
    "LE片LI心LU",
    "LE片LI心LO",
    "LE片LI心LY",
    "LE片LI心LAI",
    "LE片LI心LE", /* 赤弓踏前 */
    "LE片LA心LE", /* 赤弓踏後 */
    "LE片TE心ZE",
    "LE片TE心NE",
    "LE片TE心LE", /* 赤弓踏左 */
    "LE片KE心LE",
    "LE片KE心NE", /* 赤弓踏右 */
    "TE片ZIXU",
    "TE片ZITU",
    "TE片ZITE", /* 赤虎踏船 */
    "TE片NI心TU",
    "TE片NI心KO",
    "TE片NI心LU",
    "TE片NI心TE", /* 赤虎踏兵 */
    "TE片ZATE",    /* 赤虎踏王 */
    "TE片NATE",    /* 赤虎踏車 */
    "XE片ZIXU",
    "XE片ZITU",
    "XE片ZIXE", /* 黒虎踏船 */
    "XE片CI心PO",
    "XE片CI心MU",
    "XE片CI心XU",
    "XE片CI心XE", /* 黒虎踏水 */
    "XE片ZAXE",    /* 黒虎踏王 */
    "XE片CAXE",    /* 黒虎踏車 */
    "ME片CE",      /* 黒弓不踏 */
    "ME片MI心MU",
    "ME片MI心MO",
    "ME片MI心MY",
    "ME片MI心MAI",
    "ME片MI心ME", /* 黒弓踏前 */
    "ME片MA心ME", /* 黒弓踏後 */
    "ME片XE心CE",
    "ME片XE心ME",
    "ME片XE心ZE", /* 黒弓踏右 */
    "ME片PE心ME",
    "ME片PE心CE", /* 黒弓踏左 */
    "PE片PIPU",
    "PE片PIPE",
    "PE片PAPE",
    "PE片ME心PE",
    "PE片ME心CE", /* 黒巫 */
    "KI片KU",
    "LI片LU",
    "TI片TU",
    "ZI片ZU",
    "XI片XU",
    "MI片MU",
    "PI片PU", /* 兵 */
    "NI片NU",
    "NI片TITU",
    "NI片LILU",
    "NI片NE",
    "NI片NO水", /* 皇処之兵 */
    "CI片CU",
    "CI片MIMU",
    "CI片XIXU",
    "CI片CE",
    "CI片CO水", /* 皇処之兵 */
    /* 皇 */
    "ZO皇[XY]ZO",
    "ZO皇[XY]XO",
    "ZO皇[XY]CO",
    "ZO皇[XY]ZY",
    "ZO皇[XY]CY",
    "ZO皇[XY]ZAITY",
    "ZO皇[XY]ZAIZY",
    "ZO皇[XY]ZAIXY",
    "ZO皇[XY]ZAIZAU",
    "ZO皇[XY]XAIZY",
    "ZO皇[XY]XAIXY",
    "ZO皇[XY]XAICY",
    "ZO皇[XY]XAIZAU",
    "ZO皇[XY]XAICAU",
    "ZO皇[XY]CAIXY",
    "ZO皇[XY]CAICY",
    "ZO皇[XY]CAIMY",
    "ZO皇[XY]CAICAU",
    "ZO皇[ZY]TO",
    "ZO皇[ZY]ZO",
    "ZO皇[ZY]XO",
    "ZO皇[ZY]TY",
    "ZO皇[ZY]XY",
    "ZO皇[ZY]TAINY",
    "ZO皇[ZY]TAITY",
    "ZO皇[ZY]TAIZY",
    "ZO皇[ZY]TAINAU",
    "ZO皇[ZY]TAIZAU",
    "ZO皇[ZY]ZAITY",
    "ZO皇[ZY]ZAIZY",
    "ZO皇[ZY]ZAIXY",
    "ZO皇[ZY]ZAIZAU",
    "ZO皇[ZY]XAIZY",
    "ZO皇[ZY]XAIXY",
    "ZO皇[ZY]XAICY",
    "ZO皇[ZY]XAIZAU",
    "ZO皇[ZY]XAICAU",
    "ZO皇[TY]NO",
    "ZO皇[TY]TO",
    "ZO皇[TY]ZO",
    "ZO皇[TY]NY",
    "ZO皇[TY]ZY",
    "ZO皇[TY]NAILY",
    "ZO皇[TY]NAINY",
    "ZO皇[TY]NAITY",
    "ZO皇[TY]NAINAU",
    "ZO皇[TY]TAINY",
    "ZO皇[TY]TAITY",
    "ZO皇[TY]TAIZY",
    "ZO皇[TY]TAINAU",
    "ZO皇[TY]TAIZAU",
    "ZO皇[TY]ZAITY",
    "ZO皇[TY]ZAIZY",
    "ZO皇[TY]ZAIXY",
    "ZO皇[TY]ZAIZAU",
    "ZO皇[XO]ZU",
    "ZO皇[XO]XU",
    "ZO皇[XO]CU",
    "ZO皇[XO]ZO",
    "ZO皇[XO]CO",
    "ZO皇[XO]ZY",
    "ZO皇[XO]XY",
    "ZO皇[XO]CY",
    "ZO皇[TO]NU",
    "ZO皇[TO]TU",
    "ZO皇[TO]ZU",
    "ZO皇[TO]NO",
    "ZO皇[TO]ZO",
    "ZO皇[TO]NY",
    "ZO皇[TO]TY",
    "ZO皇[TO]ZY",
    "ZO皇[XU]ZU",
    "ZO皇[XU]CU",
    "ZO皇[XU]ZO",
    "ZO皇[XU]XO",
    "ZO皇[XU]CO",
    "ZO皇[XU]ZIZE",
    "ZO皇[XU]ZITU",
    "ZO皇[XU]ZIZU",
    "ZO皇[XU]ZIXU",
    "ZO皇[XU]XIZE",
    "ZO皇[XU]XICE",
    "ZO皇[XU]XIZU",
    "ZO皇[XU]XIXU",
    "ZO皇[XU]XICU",
    "ZO皇[XU]CICE",
    "ZO皇[XU]CIXU",
    "ZO皇[XU]CICU",
    "ZO皇[XU]CIMU",
    "ZO皇[ZU]TU",
    "ZO皇[ZU]XU",
    "ZO皇[ZU]TO",
    "ZO皇[ZU]ZO",
    "ZO皇[ZU]XO",
    "ZO皇[ZU]TINE",
    "ZO皇[ZU]TIZE",
    "ZO皇[ZU]TINU",
    "ZO皇[ZU]TITU",
    "ZO皇[ZU]TIZU",
    "ZO皇[ZU]ZIZE",
    "ZO皇[ZU]ZITU",
    "ZO皇[ZU]ZIZU",
    "ZO皇[ZU]ZIXU",
    "ZO皇[ZU]XIZE",
    "ZO皇[ZU]XICE",
    "ZO皇[ZU]XIZU",
    "ZO皇[ZU]XIXU",
    "ZO皇[ZU]XICU",
    "ZO皇[TU]NU",
    "ZO皇[TU]ZU",
    "ZO皇[TU]NO",
    "ZO皇[TU]TO",
    "ZO皇[TU]ZO",
    "ZO皇[TU]NINE",
    "ZO皇[TU]NILU",
    "ZO皇[TU]NINU",
    "ZO皇[TU]NITU",
    "ZO皇[TU]TINE",
    "ZO皇[TU]TIZE",
    "ZO皇[TU]TINU",
    "ZO皇[TU]TITU",
    "ZO皇[TU]TIZU",
    "ZO皇[TU]ZIZE",
    "ZO皇[TU]ZITU",
    "ZO皇[TU]ZIZU",
    "ZO皇[TU]ZIXU",
];
