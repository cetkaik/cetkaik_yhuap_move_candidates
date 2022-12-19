use cetkaik_core::{absolute::PureMove, perspective::to_absolute_coord};

use crate::{CetkaikCore, CetkaikRepresentation};

use super::PureGameState;

#[must_use]
fn from_hop1zuo1_candidates_old(game_state: &PureGameState) -> Vec<PureMove> {
    let mut ans = vec![];
    for piece in &game_state.f.hop1zuo1of_downward {
        for empty_square in CetkaikCore::empty_squares_relative(&game_state.f.current_board) {
            ans.push(PureMove::NonTamMoveFromHopZuo {
                color: piece.color,
                prof: piece.prof,
                dest: to_absolute_coord(empty_square, game_state.perspective),
            });
        }
    }
    ans
}

#[test]
fn test_initial_board_sample() {
    super::run_test(
        from_hop1zuo1_candidates_old,
        &crate::tests::test_cases::INITIAL_BOARD_SAMPLE,
        crate::pure_move::PureMove::serialize,
        &[],
    );
}

#[test]
fn test_simple_board_sample_4() {
    super::run_test(
        from_hop1zuo1_candidates_old,
        &crate::tests::test_cases::simple_board_sample_4(),
        crate::pure_move::PureMove::serialize,
        &[
            "黒弓KA",
            "黒弓LA",
            "黒弓NA",
            "黒弓TA",
            "黒弓ZA",
            "黒弓XA",
            "黒弓CA",
            "黒弓MA",
            "黒弓PA",
            "黒弓KE",
            "黒弓LE",
            "黒弓NE",
            "黒弓TE",
            "黒弓ZE",
            "黒弓XE",
            "黒弓CE",
            "黒弓ME",
            "黒弓PE",
            "黒弓KI",
            "黒弓LI",
            "黒弓NI",
            "黒弓TI",
            "黒弓ZI",
            "黒弓XI",
            "黒弓CI",
            "黒弓MI",
            "黒弓KU",
            "黒弓LU",
            "黒弓NU",
            "黒弓TU",
            "黒弓ZU",
            "黒弓XU",
            "黒弓CU",
            "黒弓MU",
            "黒弓PU",
            "黒弓KO",
            "黒弓LO",
            "黒弓NO",
            "黒弓TO",
            "黒弓ZO",
            "黒弓XO",
            "黒弓CO",
            "黒弓MO",
            "黒弓PO",
            "黒弓KY",
            "黒弓LY",
            "黒弓NY",
            "黒弓TY",
            "黒弓ZY",
            "黒弓XY",
            "黒弓CY",
            "黒弓MY",
            "黒弓PY",
            "黒弓KAI",
            "黒弓LAI",
            "黒弓NAI",
            "黒弓TAI",
            "黒弓ZAI",
            "黒弓XAI",
            "黒弓CAI",
            "黒弓MAI",
            "黒弓PAI",
            "黒弓KAU",
            "黒弓LAU",
            "黒弓NAU",
            "黒弓TAU",
            "黒弓ZAU",
            "黒弓XAU",
            "黒弓CAU",
            "黒弓MAU",
            "黒弓PAU",
            "黒弓KIA",
            "黒弓LIA",
            "黒弓NIA",
            "黒弓TIA",
            "黒弓ZIA",
            "黒弓XIA",
            "黒弓CIA",
            "黒弓MIA",
        ],
    );
}
