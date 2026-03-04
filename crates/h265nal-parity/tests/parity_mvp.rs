use h265nal_parity::run_compare;

fn docker_tests_enabled() -> bool {
    matches!(
        std::env::var("H265NAL_PARITY_RUN_DOCKER_TESTS").as_deref(),
        Ok("1")
    )
}

#[test]
fn mvp_dump_one_line_matches_baseline() {
    if !docker_tests_enabled() {
        return;
    }

    run_compare("dump_one_line", None).expect("parity mismatch for dump_one_line");
}

#[test]
fn mvp_dump_multiline_matches_baseline() {
    if !docker_tests_enabled() {
        return;
    }

    run_compare("dump_multiline", None).expect("parity mismatch for dump_multiline");
}

#[test]
fn mvp_dump_crash_fixture_matches_baseline() {
    if !docker_tests_enabled() {
        return;
    }

    run_compare("dump_crash_fixture", None).expect("parity mismatch for dump_crash_fixture");
}
