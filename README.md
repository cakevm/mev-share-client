# Alloy MEV-Share Client

This crate allows to subscribe to the [MEV-Share event stream](https://docs.flashbots.net/flashbots-mev-share/searchers/event-stream) from [Flashbots](https://www.flashbots.net) using types from `alloy-rpc-types-mev`. But right now does not allow to send bundles to the MEV-Share API.


# Why another crate?
Clients like [mev-share-rs](https://github.com/paradigmxyz/mev-share-rs) still depend on `ethers` and provide more than necessary for most users. This crate is a lightweight alternative to subscribe to the MEV-Share event stream.


# Usage
See `subscribe_mev_share.rs` in [examples](./examples) for a full usage examples. 

Example usage:
```rust
let mut stream = MevShareClient::new()?.subscribe();

loop {
    let event = stream.try_next().await?;
    if let Some(event) = event {
        info!("Event: {:?}", event);
    }
}

```

# Acknowledgements
Many thanks to the [Flashbots](https://www.flashbots.net) team for proving this API. And many thanks to the [alloy-rs](https://github.com/alloy-rs) team.

# License
This project is licensed under the [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).