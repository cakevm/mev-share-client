use alloy_primitives::{keccak256, TxHash};
use alloy_rpc_types_mev::mevshare::Event;
use eventsource_client::{Client, ClientBuilder, ReconnectOptions, SSE};
use futures_util::{Stream, StreamExt};
use std::pin::Pin;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, trace};

pub static MEV_SHARE_SSE_URL: &str = "https://mev-share.flashbots.net";

#[derive(Debug, Error)]
pub enum MevShareClientError {
    #[error("Error connecting to mev share: {0}")]
    ClientError(#[from] eventsource_client::Error),
    #[error("Error deserializing event: {0}")]
    DeserializationError(#[from] serde_json::Error),
}

pub struct MevShareClient {
    client: Box<dyn Client>,
}

impl MevShareClient {
    pub fn new() -> Result<Self, eventsource_client::Error> {
        let client = ClientBuilder::for_url(MEV_SHARE_SSE_URL)?
            .reconnect(
                ReconnectOptions::reconnect(true)
                    .retry_initial(false)
                    .delay(Duration::from_secs(1))
                    .backoff_factor(2)
                    .delay_max(Duration::from_secs(60))
                    .build(),
            )
            .build();

        Ok(Self { client: Box::new(client) })
    }

    pub fn new_with_reconnect_options(reconnect_options: ReconnectOptions) -> Result<Self, eventsource_client::Error> {
        let client = ClientBuilder::for_url(MEV_SHARE_SSE_URL)?.reconnect(reconnect_options).build();

        Ok(Self { client: Box::new(client) })
    }

    pub fn subscribe(&self) -> Pin<Box<dyn Stream<Item = Result<Event, MevShareClientError>> + Send>> {
        Box::pin(self.client.stream().filter_map(|event| async move {
            match event {
                Ok(SSE::Connected(connection)) => {
                    debug!(status=%connection.response().status(), "MEV-Share connected");
                    None
                }
                Ok(SSE::Event(ev)) => {
                    trace!(event_type=%ev.event_type, data=%ev.data, "Received event");
                    match serde_json::from_str::<Event>(&ev.data) {
                        Ok(event) => Some(Ok(event)),
                        Err(err) => {
                            error!(?err, "Error deserializing event");
                            Some(Err(MevShareClientError::DeserializationError(err)))
                        }
                    }
                }
                Ok(SSE::Comment(comment)) => {
                    debug!(%comment, "Received comment");
                    None
                }
                Err(err) => {
                    error!(?err, "Error in stream");
                    Some(Err(MevShareClientError::ClientError(err)))
                }
            }
        }))
    }
}

pub fn tx_hash_to_event_hash(tx_hash: TxHash) -> TxHash {
    keccak256(tx_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tx_hash_to_event_hash() {
        let input = TxHash::from_str("d2d662b8aa0e8d86ea75d363522c9ede42ef538ae353da564d501c044a885293").unwrap();
        let expected_output = TxHash::from_str("90b4f5664cc201c3aa112d6bb2fa414c4aee10f00994692b282c1d14a1db6e4d").unwrap();
        assert_eq!(tx_hash_to_event_hash(input), expected_output);
    }
}
