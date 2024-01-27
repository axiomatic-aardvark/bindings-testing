use std::error::Error;

use waku::{
    waku_new, ContentFilter, Running, WakuNodeConfig, WakuNodeHandle,
};

// mainnet ENR enrtree://AOADZWXPAJ56TIXA74PV7VJP356QNBIKUPRKR676BBOOELU5XDDKM@testnet.bootnodes.graphcast.xyz

pub const WAKU_DISCOVERY_ENR: &str = "enr:-P-4QJI8tS1WTdIQxq_yIrD05oIIW1Xg-tm_qfP0CHfJGnp9dfr6ttQJmHwTNxGEl4Le8Q7YHcmi-kXTtphxFysS11oBgmlkgnY0gmlwhLymh5GKbXVsdGlhZGRyc7hgAC02KG5vZGUtMDEuZG8tYW1zMy53YWt1djIucHJvZC5zdGF0dXNpbS5uZXQGdl8ALzYobm9kZS0wMS5kby1hbXMzLndha3V2Mi5wcm9kLnN0YXR1c2ltLm5ldAYfQN4DiXNlY3AyNTZrMaEDbl1X_zJIw3EAJGtmHMVn4Z2xhpSoUaP5ElsHKCv7hlWDdGNwgnZfg3VkcIIjKIV3YWt1Mg8";

fn setup_node_handle() -> std::result::Result<WakuNodeHandle<Running>, Box<dyn Error>> {
    let config = WakuNodeConfig {
        host: None,
        port: None,
        advertise_addr: None,
        node_key: None,
        keep_alive_interval: None,
        relay: None,
        store: None,
        database_url: None, 
        store_retention_max_messages: None,
        store_retention_max_seconds: None,
        relay_topics: vec![],
        min_peers_to_publish:  Some(0),
        filter: None,
        log_level: None,
        discv5: Some(false),
        discv5_bootstrap_nodes: vec![WAKU_DISCOVERY_ENR.to_string()],
        discv5_udp_port: None,
        gossipsub_params: None,
        dns4_domain_name: None,
        websocket_params: None,
    };

    let node_handle = waku_new(Some(config))?;
    let node_handle = node_handle.start()?;

    let content_filter = ContentFilter::new(Some("/waku/2/graphcast-v0-mainnet/proto".to_string()), vec![]);
    node_handle.relay_subscribe(&content_filter)?;
    Ok(node_handle)
}

fn main() {
    let _ = setup_node_handle();

    loop {

    }
}
