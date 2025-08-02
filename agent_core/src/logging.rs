use clap::Parser;
use tracing_subscriber::fmt::Subscriber;

// CLI INTERFACE FOR DEVELOPER TRIGGERS

// ENTRYPOINT
#[derive(Parser)]
pub struct CLI {
    #[arg(long)]
    pub config: String,
}

// LOGGING AND AUDITE TRAIL
pub fn init_logging() {
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::fs::File::create("agent.log").unwrap())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}