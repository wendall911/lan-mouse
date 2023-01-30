use std::{sync::mpsc, thread, net::SocketAddr};

use lan_mouse::{
    event::{self, producer, consumer},
    config,
    request,
    client::{ClientManager, Position}, dns,
};

fn add_client(client_manager: &mut ClientManager, client: &config::Client, pos: Position) {
    let ip = match client.ip {
        Some(ip) => ip,
        None => match &client.host_name {
            Some(host_name) => match dns::resolve(host_name) {
                Ok(ip) => ip,
                Err(e) => panic!("{}", e),
            },
            None => panic!("neither ip nor hostname specified"),
        }
    };
    let addr = SocketAddr::new(ip, client.port.unwrap_or(42069));
    client_manager.add_client(addr, pos);
}

pub fn main() {
    // parse config file
    let config = config::Config::new("config.toml").unwrap();

    // port or default
    let port = config.port.unwrap_or(42069);

    // event channel for producing events
    let (produce_tx, produce_rx) = mpsc::sync_channel(128);

    // event channel for consuming events
    let (consume_tx, consume_rx) = mpsc::sync_channel(128);

    let mut client_manager = ClientManager::new();

    // add clients from config
    for client in vec![
        &config.client.left,
        &config.client.right,
        &config.client.top,
        &config.client.bottom,
    ] {
        if let Some(client) = client {
            let pos = match client {
                client if Some(client) == config.client.left.as_ref() => Position::Left,
                client if Some(client) == config.client.right.as_ref() => Position::Right,
                client if Some(client) == config.client.top.as_ref() => Position::Top,
                client if Some(client) == config.client.bottom.as_ref() => Position::Bottom,
                _ => panic!(),
            };
            add_client(&mut client_manager, client, pos);
        }
    }

    // start receiving client connection requests
    let (request_server, request_thread) = request::Server::listen(port).unwrap();

    let clients = client_manager.get_clients();
    // start producing and consuming events
    let event_producer = thread::Builder::new()
        .name("event producer".into())
        .spawn(|| {
        producer::run(produce_tx, request_server, clients);
    }).unwrap();

    let clients = client_manager.get_clients();
    let event_consumer = thread::Builder::new()
        .name("event consumer".into())
        .spawn(|| {
        consumer::run(consume_rx, clients);
    }).unwrap();

    // start sending and receiving events
    let event_server = event::server::Server::new(port);
    let (receiver, sender) = event_server.run(&mut client_manager, produce_rx, consume_tx).unwrap();

    request_thread.join().unwrap();

    receiver.join().unwrap();
    sender.join().unwrap();

    event_producer.join().unwrap();
    event_consumer.join().unwrap();
}
