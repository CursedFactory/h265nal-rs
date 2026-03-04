#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CliFeature {
    Infile,
    Outfile,
    HvccFile,
    DumpAll,
    DumpLength,
    NaluLengthBytes,
    FramesPerSecond,
    AsOneLine,
    NoAsOneLine,
    AddOffset,
    NoAddOffset,
    AddLength,
    NoAddLength,
    AddParsedLength,
    NoAddParsedLength,
    AddChecksum,
    NoAddChecksum,
    AddResolution,
    NoAddResolution,
    AddContents,
    NoAddContents,
    Debug,
    Quiet,
    Version,
    Help,
    MissingInput,
}

impl CliFeature {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Infile => "--infile/-i",
            Self::Outfile => "--outfile/-o",
            Self::HvccFile => "--hvcc-file",
            Self::DumpAll => "--dump-all",
            Self::DumpLength => "--dump-length",
            Self::NaluLengthBytes => "--nalu-length-bytes",
            Self::FramesPerSecond => "--frames-per-second",
            Self::AsOneLine => "--as-one-line",
            Self::NoAsOneLine => "--no-as-one-line",
            Self::AddOffset => "--add-offset",
            Self::NoAddOffset => "--no-add-offset",
            Self::AddLength => "--add-length",
            Self::NoAddLength => "--no-add-length",
            Self::AddParsedLength => "--add-parsed-length",
            Self::NoAddParsedLength => "--no-add-parsed-length",
            Self::AddChecksum => "--add-checksum",
            Self::NoAddChecksum => "--no-add-checksum",
            Self::AddResolution => "--add-resolution",
            Self::NoAddResolution => "--no-add-resolution",
            Self::AddContents => "--add-contents",
            Self::NoAddContents => "--no-add-contents",
            Self::Debug => "--debug/-d",
            Self::Quiet => "--quiet",
            Self::Version => "--version",
            Self::Help => "--help/-h",
            Self::MissingInput => "missing-input behavior",
        }
    }
}

pub const ALL_CLI_FEATURES: [CliFeature; 26] = [
    CliFeature::Infile,
    CliFeature::Outfile,
    CliFeature::HvccFile,
    CliFeature::DumpAll,
    CliFeature::DumpLength,
    CliFeature::NaluLengthBytes,
    CliFeature::FramesPerSecond,
    CliFeature::AsOneLine,
    CliFeature::NoAsOneLine,
    CliFeature::AddOffset,
    CliFeature::NoAddOffset,
    CliFeature::AddLength,
    CliFeature::NoAddLength,
    CliFeature::AddParsedLength,
    CliFeature::NoAddParsedLength,
    CliFeature::AddChecksum,
    CliFeature::NoAddChecksum,
    CliFeature::AddResolution,
    CliFeature::NoAddResolution,
    CliFeature::AddContents,
    CliFeature::NoAddContents,
    CliFeature::Debug,
    CliFeature::Quiet,
    CliFeature::Version,
    CliFeature::Help,
    CliFeature::MissingInput,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScenarioExpectation {
    MustMatch,
    KnownGap(&'static str),
}

impl ScenarioExpectation {
    pub fn label(self) -> &'static str {
        match self {
            Self::MustMatch => "must_match",
            Self::KnownGap(_) => "known_gap",
        }
    }

    pub fn gap_note(self) -> Option<&'static str> {
        match self {
            Self::KnownGap(note) => Some(note),
            Self::MustMatch => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scenario {
    pub name: &'static str,
    pub description: &'static str,
    pub default_fixture: Option<&'static str>,
    pub baseline_template: &'static [&'static str],
    pub local_template: &'static [&'static str],
    pub expectation: ScenarioExpectation,
    pub features: &'static [CliFeature],
}

impl Scenario {
    pub fn baseline_args(self, fixture_override: Option<&str>) -> Result<Vec<String>, String> {
        render_args(
            self.name,
            "baseline",
            self.baseline_template,
            self.default_fixture,
            fixture_override,
        )
    }

    pub fn local_args(self, fixture_override: Option<&str>) -> Result<Vec<String>, String> {
        render_args(
            self.name,
            "local",
            self.local_template,
            self.default_fixture,
            fixture_override,
        )
    }

    pub fn resolved_fixture(
        self,
        fixture_override: Option<&str>,
    ) -> Result<Option<String>, String> {
        let has_placeholder = self
            .baseline_template
            .iter()
            .chain(self.local_template.iter())
            .any(|value| *value == "{fixture}");

        if !has_placeholder {
            return Ok(None);
        }

        let fixture = fixture_override
            .or(self.default_fixture)
            .ok_or_else(|| format!("scenario `{}` requires a fixture path", self.name))?;
        Ok(Some(fixture.to_string()))
    }
}

fn render_args(
    scenario_name: &str,
    command_side: &str,
    template: &[&str],
    default_fixture: Option<&str>,
    fixture_override: Option<&str>,
) -> Result<Vec<String>, String> {
    let mut args = Vec::with_capacity(template.len());
    for token in template {
        if *token == "{fixture}" {
            let fixture = fixture_override.or(default_fixture).ok_or_else(|| {
                format!("scenario `{scenario_name}` requires a fixture for {command_side} args")
            })?;
            args.push(fixture.to_string());
        } else {
            args.push((*token).to_string());
        }
    }
    Ok(args)
}

const DEFAULT_FIXTURE: &str = "/work/media/nvenc.265";
const CRASH_FIXTURE: &str = "/work/media/pps_fdump_crash.202203.265";

pub const SCENARIOS: [Scenario; 31] = [
    Scenario {
        name: "dump_one_line",
        description: "default dump-all one-line output parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--as-one-line"],
        local_template: &["--dump-all", "--as-one-line", "{fixture}"],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::Infile, CliFeature::DumpAll, CliFeature::AsOneLine],
    },
    Scenario {
        name: "dump_multiline",
        description: "dump-all multiline layout parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--no-as-one-line"],
        local_template: &["--dump-all", "--no-as-one-line", "{fixture}"],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::Infile, CliFeature::DumpAll, CliFeature::NoAsOneLine],
    },
    Scenario {
        name: "dump_crash_fixture",
        description: "regression fixture that previously triggered fdump crash",
        default_fixture: Some(CRASH_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--no-as-one-line"],
        local_template: &["--dump-all", "--no-as-one-line", "{fixture}"],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::Infile, CliFeature::DumpAll, CliFeature::NoAsOneLine],
    },
    Scenario {
        name: "dump_with_no_add_offset",
        description: "explicit negative toggle for add-offset",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--no-add-offset",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--no-add-offset",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::NoAddOffset],
    },
    Scenario {
        name: "dump_with_no_add_length",
        description: "explicit negative toggle for add-length",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--no-add-length",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--no-add-length",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::NoAddLength],
    },
    Scenario {
        name: "dump_with_no_add_parsed_length",
        description: "explicit negative toggle for add-parsed-length",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--no-add-parsed-length",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--no-add-parsed-length",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::NoAddParsedLength],
    },
    Scenario {
        name: "dump_with_no_add_checksum",
        description: "explicit negative toggle for add-checksum",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--no-add-checksum",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--no-add-checksum",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::NoAddChecksum],
    },
    Scenario {
        name: "dump_with_no_add_resolution",
        description: "explicit negative toggle for add-resolution",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--no-add-resolution",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--no-add-resolution",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::NoAddResolution],
    },
    Scenario {
        name: "dump_with_no_add_contents",
        description: "explicit negative toggle for add-contents",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--no-add-contents",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--no-add-contents",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::NoAddContents],
    },
    Scenario {
        name: "dump_with_stdout_sink",
        description: "dash outfile routes output to stdout",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "-o",
            "-",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "-o",
            "-",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::Outfile],
    },
    Scenario {
        name: "dump_with_quiet",
        description: "quiet flag parity while dumping",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "--quiet",
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
        ],
        local_template: &["--quiet", "--dump-all", "--as-one-line", "{fixture}"],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::Quiet],
    },
    Scenario {
        name: "dump_with_debug_once",
        description: "single debug flag parity while dumping",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-d",
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
        ],
        local_template: &["-d", "--dump-all", "--as-one-line", "{fixture}"],
        expectation: ScenarioExpectation::MustMatch,
        features: &[CliFeature::DumpAll, CliFeature::Debug],
    },
    Scenario {
        name: "dump_with_add_offset",
        description: "positive add-offset toggle parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--as-one-line", "--add-offset"],
        local_template: &["--dump-all", "--as-one-line", "--add-offset", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI dump path currently ignores add-offset metadata fields.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::AddOffset],
    },
    Scenario {
        name: "dump_with_add_length",
        description: "positive add-length toggle parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--as-one-line", "--add-length"],
        local_template: &["--dump-all", "--as-one-line", "--add-length", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI dump path currently ignores add-length metadata fields.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::AddLength],
    },
    Scenario {
        name: "dump_with_add_parsed_length",
        description: "positive add-parsed-length toggle parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--add-parsed-length",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--add-parsed-length",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI dump path currently ignores add-parsed-length metadata fields.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::AddParsedLength],
    },
    Scenario {
        name: "dump_with_add_checksum",
        description: "positive add-checksum toggle parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--as-one-line", "--add-checksum"],
        local_template: &["--dump-all", "--as-one-line", "--add-checksum", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI dump path currently ignores add-checksum metadata fields.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::AddChecksum],
    },
    Scenario {
        name: "dump_with_add_resolution",
        description: "positive add-resolution toggle parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--add-resolution",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--add-resolution",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI dump path currently ignores add-resolution metadata fields.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::AddResolution],
    },
    Scenario {
        name: "dump_with_add_contents",
        description: "positive add-contents toggle parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-all", "--as-one-line", "--add-contents"],
        local_template: &["--dump-all", "--as-one-line", "--add-contents", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI dump path currently does not emit raw NAL contents in C-mode.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::AddContents],
    },
    Scenario {
        name: "dump_with_add_contents_no_add_offset",
        description: "derived behavior where add-contents forces add-offset",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--add-contents",
            "--no-add-offset",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--add-contents",
            "--no-add-offset",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI currently does not implement add-contents derived output behavior.",
        ),
        features: &[CliFeature::AddContents, CliFeature::NoAddOffset],
    },
    Scenario {
        name: "dump_length_mode",
        description: "csv dump-length mode parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-length"],
        local_template: &["--dump-length", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI returns not-implemented for --dump-length.",
        ),
        features: &[CliFeature::DumpLength],
    },
    Scenario {
        name: "dump_length_with_no_add_length",
        description: "dump-length derived behavior over explicit no-add-length",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-length", "--no-add-length"],
        local_template: &["--dump-length", "--no-add-length", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI returns not-implemented for --dump-length before derived output is evaluated.",
        ),
        features: &[CliFeature::DumpLength, CliFeature::NoAddLength],
    },
    Scenario {
        name: "dump_length_with_fps",
        description: "dump-length bitrate rows with explicit fps",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["-i", "{fixture}", "--dump-length", "--frames-per-second", "60"],
        local_template: &["--dump-length", "--frames-per-second", "60", "{fixture}"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI returns not-implemented for --frames-per-second.",
        ),
        features: &[CliFeature::DumpLength, CliFeature::FramesPerSecond],
    },
    Scenario {
        name: "dump_with_nalu_length_bytes_zero",
        description: "single NAL parse path via explicit nalu-length-bytes=0",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--nalu-length-bytes",
            "0",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--nalu-length-bytes",
            "0",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI returns not-implemented for --nalu-length-bytes != -1.",
        ),
        features: &[CliFeature::NaluLengthBytes],
    },
    Scenario {
        name: "dump_with_nalu_length_bytes_four",
        description: "length-prefixed stream path via nalu-length-bytes=4",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "--nalu-length-bytes",
            "4",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "--nalu-length-bytes",
            "4",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI returns not-implemented for --nalu-length-bytes != -1.",
        ),
        features: &[CliFeature::NaluLengthBytes],
    },
    Scenario {
        name: "dump_with_file_outfile",
        description: "file output sink parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "-o",
            "/tmp/h265nal_parity_output.txt",
        ],
        local_template: &[
            "--dump-all",
            "--as-one-line",
            "-o",
            "/tmp/h265nal_parity_output.txt",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI does not yet support --outfile file sink with --dump-all.",
        ),
        features: &[CliFeature::DumpAll, CliFeature::Outfile],
    },
    Scenario {
        name: "dump_with_hvcc_and_infile",
        description: "combined hvcc + bitstream parse path",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "--hvcc-file",
            "{fixture}",
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
        ],
        local_template: &[
            "--hvcc-file",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI currently ignores hvcc priming state in parse execution.",
        ),
        features: &[CliFeature::HvccFile, CliFeature::Infile, CliFeature::DumpAll],
    },
    Scenario {
        name: "hvcc_only_input",
        description: "hvcc-only path without annex-b input",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &["--hvcc-file", "{fixture}", "--dump-all", "--as-one-line"],
        local_template: &["--hvcc-file", "{fixture}", "--dump-all"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI rejects hvcc-only input path as not implemented.",
        ),
        features: &[CliFeature::HvccFile],
    },
    Scenario {
        name: "version_output",
        description: "version text parity",
        default_fixture: None,
        baseline_template: &["--version"],
        local_template: &["--version"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI version text currently follows clap format, not native `version: ...` output.",
        ),
        features: &[CliFeature::Version],
    },
    Scenario {
        name: "help_output",
        description: "help text parity",
        default_fixture: None,
        baseline_template: &["--help"],
        local_template: &["--help"],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI help text currently follows clap layout, not native help formatting.",
        ),
        features: &[CliFeature::Help],
    },
    Scenario {
        name: "missing_input_error",
        description: "error parity when no input source is provided",
        default_fixture: None,
        baseline_template: &[],
        local_template: &[],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI missing-input diagnostics do not yet match native usage/error output.",
        ),
        features: &[CliFeature::MissingInput],
    },
    Scenario {
        name: "dump_with_debug_twice",
        description: "double debug flag parity",
        default_fixture: Some(DEFAULT_FIXTURE),
        baseline_template: &[
            "-d",
            "-d",
            "-i",
            "{fixture}",
            "--dump-all",
            "--as-one-line",
        ],
        local_template: &[
            "-d",
            "-d",
            "--dump-all",
            "--as-one-line",
            "{fixture}",
        ],
        expectation: ScenarioExpectation::KnownGap(
            "Rust CLI models --debug as boolean instead of native counted verbosity.",
        ),
        features: &[CliFeature::Debug],
    },
];

pub fn all_scenarios() -> &'static [Scenario] {
    &SCENARIOS
}

pub fn find_scenario(name: &str) -> Option<Scenario> {
    SCENARIOS
        .iter()
        .copied()
        .find(|scenario| scenario.name == name)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{all_scenarios, CliFeature, ALL_CLI_FEATURES};

    #[test]
    fn scenario_names_are_unique() {
        let mut names = HashSet::new();
        for scenario in all_scenarios() {
            assert!(
                names.insert(scenario.name),
                "duplicate scenario name: {}",
                scenario.name
            );
        }
    }

    #[test]
    fn matrix_covers_all_cli_features() {
        let mut covered = HashSet::new();
        for scenario in all_scenarios() {
            for feature in scenario.features {
                covered.insert(*feature);
            }
        }

        for feature in ALL_CLI_FEATURES {
            assert!(
                covered.contains(&feature),
                "feature `{}` is not covered by any parity scenario",
                CliFeature::as_str(feature)
            );
        }
    }
}
