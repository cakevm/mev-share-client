use futures_util::TryStreamExt;
use mev_share_client::MevShareClient;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from("info")).init();

    let mut stream = MevShareClient::new()?.subscribe();

    loop {
        let event = stream.try_next().await?;
        if let Some(event) = event {
            info!("Event: {:?}", event);
        }
    }
}
