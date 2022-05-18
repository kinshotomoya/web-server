use web_server_domain::setting::LogLevel;
use web_server_domain::setting::Settings;
use tracing::metadata::LevelFilter;
use tracing::{trace, Level};
use tracing_subscriber::filter::Filtered;
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::{Layer, Registry};

// tracingの設定を行う
// 環境毎にログレベルを分けている
// 参考：https://github.com/tokio-rs/tracing
pub fn setting_trace(settings: &Settings) {
    // tracing_subscriber::fmtはデフォルトで、stdout
    let layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true);
    let level = settings.log.find_trace_log_level();
    let subscriber = Registry::default().with(layer.with_filter(LevelFilter::from_level(level)));

    tracing::subscriber::set_global_default(subscriber).expect("fail to set tracing");
}
