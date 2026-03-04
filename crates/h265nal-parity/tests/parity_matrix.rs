use h265nal_parity::report::run_matrix;

fn docker_tests_enabled() -> bool {
    matches!(
        std::env::var("H265NAL_PARITY_RUN_DOCKER_TESTS").as_deref(),
        Ok("1")
    )
}

#[test]
fn matrix_must_match_contract_holds() {
    if !docker_tests_enabled() {
        return;
    }

    let report = run_matrix(None).expect("matrix run should succeed");
    let unexpected = report.unexpected_mismatches();
    assert!(
        unexpected.is_empty(),
        "must-match regressions found: {}",
        unexpected
            .iter()
            .map(|entry| entry.scenario.name)
            .collect::<Vec<_>>()
            .join(", ")
    );
}

#[test]
fn matrix_mismatches_include_stdout_diff_diagnostics() {
    if !docker_tests_enabled() {
        return;
    }

    let report = run_matrix(None).expect("matrix run should succeed");
    for entry in report.comparisons.iter().filter(|entry| !entry.is_match()) {
        let diagnostic = entry
            .diagnostic
            .as_deref()
            .expect("mismatch should contain diagnostics");
        assert!(diagnostic.contains("baseline command:"));
        assert!(diagnostic.contains("local command:"));
        assert!(
            diagnostic.contains("bytes mismatch") || diagnostic.contains("exit status mismatch"),
            "diagnostic should include mismatch details for scenario {}",
            entry.scenario.name
        );

        if diagnostic.contains("bytes mismatch") {
            assert!(diagnostic.contains("--- baseline"));
            assert!(diagnostic.contains("+++ local"));
        }
    }
}
