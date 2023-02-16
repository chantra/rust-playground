use log::info;
use tracing::info as tracing_info;
use tracing_log::env_logger;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

fn main() {
    env_logger::init();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let number_of_teams: i32 = 3;
    info!("env_logger: We've got {} teams!", number_of_teams);
    tracing_info!("tracing: We've got {} teams!", number_of_teams);
}
