use std::fs;

use h265nal_parity::run_compare;
use h265nal_parity::runner::repo_root;

fn docker_tests_enabled() -> bool {
    matches!(
        std::env::var("H265NAL_PARITY_RUN_DOCKER_TESTS").as_deref(),
        Ok("1")
    )
}

fn all_video_fixtures_for_docker() -> Result<Vec<String>, String> {
    let video_dir = repo_root().join("video");
    let mut fixtures = Vec::new();

    let entries = fs::read_dir(&video_dir).map_err(|error| {
        format!(
            "failed to read video directory {}: {error}",
            video_dir.display()
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to read video entry: {error}"))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("265") {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        fixtures.push(format!("/work/video/{file_name}"));
    }

    fixtures.sort();
    if fixtures.is_empty() {
        return Err(format!("no .265 fixtures found in {}", video_dir.display()));
    }
    Ok(fixtures)
}

#[test]
fn all_video_fixtures_match_dump_modes() {
    if !docker_tests_enabled() {
        return;
    }

    let fixtures = all_video_fixtures_for_docker().expect("video fixture discovery should work");
    for fixture in fixtures {
        run_compare("dump_one_line", Some(&fixture))
            .unwrap_or_else(|error| panic!("dump_one_line mismatch for {fixture}: {error}"));
        run_compare("dump_multiline", Some(&fixture))
            .unwrap_or_else(|error| panic!("dump_multiline mismatch for {fixture}: {error}"));
    }
}
