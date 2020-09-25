use crate::serialize::*;

#[test]
fn test_tam_corner_sample() {
    super::run_test(
        super::not_from_hand_candidates_with_kut2tam2,
        &crate::tests::test_cases::tam_corner_sample(),
        serialize_pure_opponent_move,
        &[
            "CAI片XAI",
            "CAI片CY",
            "MAI片MAUMAI",
            "PAI片PAUPAI",
            "MAU片MIAMAU",
            "MAU片PAUMAU",
            "MAU片MAIMAU",
            "PAU片MAUPAU",
            "PAU片PAIPAU",
            "MIA片MAUMIA",
            /* 撃皇 */
            "PAU片PIAPAU",
            "MIA片PIAMIA",
        ],
    )
}

#[test]
fn test_tam_itself_is_not_tam_hue_sample() {
    super::run_test(
        super::not_from_hand_candidates_with_kut2tam2,
        &crate::TAM_ITSELF_IS_NOT_TAM_HUE_SAMPLE,
        serialize_pure_opponent_move,
        &[
            &crate::INITIAL_MOVES_NO_KUT_TAM[..],
            &vec![
                "ZI片ZOXO",
                "ZI片ZOTO",
                "ZI片ZOCO",
                "ZI片ZONO",
                "ZI片ZO心ZY",
                "ZI片ZO心ZAI",
                "ZI片ZO心ZU",
                "ZI片ZO心ZI",
                "ZI片ZO心ZE",
            ][..],
        ]
        .concat(),
    )
}

#[test]
fn test_initial_board_sample() {
    super::run_test(
        super::not_from_hand_candidates_with_kut2tam2,
        &crate::INITIAL_BOARD_SAMPLE,
        serialize_pure_opponent_move,
        &[
            &crate::INITIAL_MOVES_NO_KUT_TAM[..],
            &vec![
                "ZI片ZOXO",
                "ZI片ZOTO",
                "ZI片ZOCO",
                "ZI片ZONO",
                "ZI片ZO心ZY",
                "ZI片ZO心ZAI",
                "ZI片ZO心ZU",
                "ZI片ZO心ZI",
                "ZI片ZO心ZE",
            ],
        ]
        .concat(),
    )
}
