use std::{borrow::Cow, error::Error, str::FromStr, thread::sleep, time::Duration};

use waku::{
    waku_default_pubsub_topic, waku_new, waku_set_event_callback, ContentFilter, Multiaddr, ProtocolId, Running, WakuContentTopic, WakuMessage, WakuMessageVersion, WakuNodeConfig, WakuNodeHandle
};

pub const WAKU_DISCOVERY_ENR: &str = "enr:-P-4QJI8tS1WTdIQxq_yIrD05oIIW1Xg-tm_qfP0CHfJGnp9dfr6ttQJmHwTNxGEl4Le8Q7YHcmi-kXTtphxFysS11oBgmlkgnY0gmlwhLymh5GKbXVsdGlhZGRyc7hgAC02KG5vZGUtMDEuZG8tYW1zMy53YWt1djIucHJvZC5zdGF0dXNpbS5uZXQGdl8ALzYobm9kZS0wMS5kby1hbXMzLndha3V2Mi5wcm9kLnN0YXR1c2ltLm5ldAYfQN4DiXNlY3AyNTZrMaEDbl1X_zJIw3EAJGtmHMVn4Z2xhpSoUaP5ElsHKCv7hlWDdGNwgnZfg3VkcIIjKIV3YWt1Mg8";

const NODES: &[&str] = &[
    "/dns4/node-01.ac-cn-hongkong-c.wakuv2.test.statusim.net/tcp/30303/p2p/16Uiu2HAkvWiyFsgRhuJEb9JfjYxEkoHLgnUQmr1N5mKWnYjxYRVm",
    "/dns4/node-01.do-ams3.wakuv2.test.statusim.net/tcp/30303/p2p/16Uiu2HAmPLe7Mzm8TsYUubgCAW1aJoeFScxrLj8ppHFivPo97bUZ",
    "/dns4/node-01.gc-us-central1-a.wakuv2.test.statusim.net/tcp/30303/p2p/16Uiu2HAmJb2e28qLXxT5kZxVUUoJt72EMzNGXB47Rxx5hw3q4YjS"
];

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
        min_peers_to_publish: Some(0),
        log_level: None,
        discv5: Some(false),
        discv5_bootstrap_nodes: vec![WAKU_DISCOVERY_ENR.to_string()],
        discv5_udp_port: None,
        gossipsub_params: None,
        dns4_domain_name: None,
        websocket_params: None,
        dns_discovery_urls: vec![],
        dns_discovery_nameserver: None,
    };

    let node_handle = waku_new(Some(config))?;
    let node_handle = node_handle.start()?;

    for address in NODES.iter().map(|a| Multiaddr::from_str(a).unwrap()) {
        let peerid = node_handle.add_peer(&address, ProtocolId::Relay)?;
        node_handle.connect_peer_with_id(&peerid, None)?;
    }

    let content_filter = ContentFilter::new(Some(waku_default_pubsub_topic()), vec![]);
    node_handle.relay_subscribe(&content_filter)?;
    Ok(node_handle)
}

fn main() {
    let node_handle = setup_node_handle().unwrap();

    waku_set_event_callback(move |signal| match signal.event() {
        waku::Event::WakuMessage(event) => {
            println!("received message! {:?}", event.waku_message())
        }
        _ => {}
    });

    loop {
        let payload = b"Hello, Waku!".to_vec();
        let meta = b"metadata".to_vec();
        let content_topic = WakuContentTopic {
            application_name: "first".to_string().into(),
            version: Cow::from("0".to_string()),
            content_topic_name: "test".to_string().into(),
            encoding: waku::Encoding::Proto,
        };
        let version = WakuMessageVersion::default();
        let timestamp = 1622540000;
        let ephemeral = false;

        let waku_message =
            WakuMessage::new(payload, content_topic, version, timestamp, meta, ephemeral);

        println!("Sending message");

        if let Err(e) = node_handle.relay_publish_message(
            &waku_message,
            Some(waku_default_pubsub_topic()),
            None,
        ) {
            println!("Failed to relay publish the message: {:?}", e);
            panic!();
        }

        sleep(Duration::from_secs(5));
    }
}
