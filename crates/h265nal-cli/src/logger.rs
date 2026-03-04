use std::io::IsTerminal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const RUST_LOG_ENV: &str = "RUST_LOG";
const NO_COLOR_ENV: &str = "NO_COLOR";
const DEFAULT_LOG_LEVEL: &str = "INFO";

#[derive(Debug, Clone, Copy)]
pub struct InitOptions {
    pub default_level: &'static str,
}

impl InitOptions {
    pub fn cli_default() -> Self {
        Self {
            default_level: DEFAULT_LOG_LEVEL,
        }
    }
}

pub fn init(options: InitOptions) -> Result<(), String> {
    let filter_value =
        std::env::var(RUST_LOG_ENV).unwrap_or_else(|_| options.default_level.to_owned());
    let env_filter = EnvFilter::try_new(&filter_value)
        .or_else(|_| EnvFilter::try_new(options.default_level))
        .map_err(|e| format!("failed to build env filter: {}", e))?;

    let use_ansi = std::io::stdout().is_terminal() && std::env::var_os(NO_COLOR_ENV).is_none();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_ansi(use_ansi);

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|e| format!("failed to initialize tracing subscriber: {}", e))?;

    Ok(())
}
