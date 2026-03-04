#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScenarioId {
    DumpOneLine,
    DumpMultiline,
    DumpCrashFixture,
}

impl ScenarioId {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "dump_one_line" => Some(Self::DumpOneLine),
            "dump_multiline" => Some(Self::DumpMultiline),
            "dump_crash_fixture" => Some(Self::DumpCrashFixture),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DumpOneLine => "dump_one_line",
            Self::DumpMultiline => "dump_multiline",
            Self::DumpCrashFixture => "dump_crash_fixture",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scenario {
    pub id: ScenarioId,
    pub default_fixture: &'static str,
    pub one_line: bool,
}

impl Scenario {
    pub fn name(self) -> &'static str {
        self.id.as_str()
    }

    pub fn baseline_args(self, fixture: &str) -> Vec<String> {
        let mut args = vec![
            "-i".to_string(),
            fixture.to_string(),
            "--dump-all".to_string(),
        ];
        if self.one_line {
            args.push("--as-one-line".to_string());
        } else {
            args.push("--no-as-one-line".to_string());
        }
        args
    }

    pub fn local_args(self, fixture: &str) -> Vec<String> {
        let mut args = vec!["--dump".to_string()];
        if self.one_line {
            args.push("--one-line".to_string());
        }
        args.push(fixture.to_string());
        args
    }
}

pub const MVP_SCENARIOS: [Scenario; 3] = [
    Scenario {
        id: ScenarioId::DumpOneLine,
        default_fixture: "/work/media/nvenc.265",
        one_line: true,
    },
    Scenario {
        id: ScenarioId::DumpMultiline,
        default_fixture: "/work/media/nvenc.265",
        one_line: false,
    },
    Scenario {
        id: ScenarioId::DumpCrashFixture,
        default_fixture: "/work/media/pps_fdump_crash.202203.265",
        one_line: false,
    },
];

pub fn mvp_scenarios() -> &'static [Scenario] {
    &MVP_SCENARIOS
}

pub fn find_scenario(name: &str) -> Option<Scenario> {
    ScenarioId::parse(name)
        .and_then(|id| MVP_SCENARIOS.iter().find(|entry| entry.id == id).copied())
}
