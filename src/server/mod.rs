use std::collections::HashMap;
use std::sync::Arc;
use std::{io::Read, slice::Iter};

use log::{debug, info};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::world::player::{ConnectionState, Player};

pub mod network;

#[derive(Default)]
pub struct ServerState {
    pub players: HashMap<u32, Player>,
    #[allow(dead_code)]
    pub max_players: u32,
}

pub fn read_packet<'a>(
    buffer: &mut Iter<'_, u8>,
    player: &mut Player,
) -> Result<(u8, Vec<network::NetworkValue>), Box<dyn std::error::Error>> {
    let _packet_length = buffer.next().expect("");
    let packet_id = buffer.next().expect("");
    let mut payload: Vec<network::NetworkValue> = vec![];

    debug!("Parsing packet");

    if player.state == ConnectionState::Handshake && *packet_id == 0 {
        let protocol_version = network::read_varint(buffer)?;
        let server_address = network::read_string(buffer)?;
        let mut server_port = [0u8; 2];
        server_port[0] = buffer.next().expect("").clone();
        server_port[1] = buffer.next().expect("").clone();
        let server_port = u16::from_be_bytes(server_port);
        let intent = network::read_varint(buffer)?;

        payload.push(network::NetworkValue::u16(protocol_version as u16));
        payload.push(network::NetworkValue::String(server_address));
        payload.push(network::NetworkValue::u16(server_port));
        payload.push(network::NetworkValue::u8(intent as u8));
    }
    if player.state == ConnectionState::Status {
        if *packet_id == 1 {
            let mut timestamp = [0u8; 8];
            for i in 0..7 {
                timestamp[i] = buffer.next().expect("").clone();
            }

            payload.push(network::NetworkValue::Array(Box::from(timestamp)));
        }
    }
    if player.state == ConnectionState::Login {
        if *packet_id == 0 {
            let name = network::read_string(buffer)?;
            let mut uuid = [0u8; 16];
            for i in 0..15 {
                uuid[i] = buffer.next().expect("").clone();
            }

            payload.push(network::NetworkValue::String(name));
            payload.push(network::NetworkValue::Array(Box::from(uuid)));
        }
    }
    if player.state == ConnectionState::Configuration {
        if *packet_id == 0 {
            let locale = network::read_string(buffer)?;
            let view_distance = buffer.next().expect("").to_owned();
            let chat_mode = network::read_varint(buffer)?;
            let chat_colors = buffer.next().expect("").to_owned() == 0x01;
            let displayed_skin_parts = buffer.next().expect("").to_owned();
            let main_hand = network::read_varint(buffer)?;
            let enable_text_filtering = buffer.next().expect("").to_owned() == 0x01;
            let allow_server_listings = buffer.next().expect("").to_owned() == 0x01;
            let particle_status = network::read_varint(buffer)?;

            payload.push(network::NetworkValue::String(locale));
            payload.push(network::NetworkValue::u8(view_distance));
            payload.push(network::NetworkValue::u8(chat_mode as u8));
            payload.push(network::NetworkValue::bool(chat_colors));
            payload.push(network::NetworkValue::u8(displayed_skin_parts));
            payload.push(network::NetworkValue::u8(main_hand as u8));
            payload.push(network::NetworkValue::bool(enable_text_filtering));
            payload.push(network::NetworkValue::bool(allow_server_listings));
            payload.push(network::NetworkValue::u8(particle_status as u8));
        }
        if *packet_id == 2 {
            let identifier = network::read_string(buffer)?;
            payload.push(network::NetworkValue::String(identifier.clone()));

            let identifier = identifier.as_str();
            
            match identifier {
                "minecraft:brand" => {
                    let brand = network::read_string(buffer)?;

                    payload.push(network::NetworkValue::String(brand));
                },
                _ => {},
            }

            if buffer.len() > 0 {
                let (_, mut new_packet) = read_packet(buffer, player)?;
                payload.append(&mut new_packet);
            }
        }
    }
    if player.state == ConnectionState::Play {

    }

    Ok((packet_id.clone(), payload))
}

pub async fn write_packet(
    stream: &TcpStream,
    packet: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    stream.writable().await?;

    stream.try_write(packet)?;
    debug!("Sent packet!");

    Ok(())
}

pub async fn handle_client(
    stream: TcpStream,
    player: &mut Player,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        stream.readable().await?;
        let result = &mut vec![];
        {
            let mut buffer = [0; 2048];
            match stream.try_read(&mut buffer) {
                Ok(0) => break Ok(()),
                Ok(n) => {
                    buffer[0..n].clone_into(result);
                }
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => continue,
                Err(_) => break Err("unknown err".into()),
            }
        }

        #[cfg(debug_assertions)]
        debug!(
            "New packet recieved: {:?} ({:?})",
            &result,
            String::from_utf8_lossy(result)
        );
        debug!("State: {:?}", player.state);

        let mut buffer: Iter<'_, u8> = result.iter();

        let (packet_id, packet) = read_packet(&mut buffer, player)?;
        debug!("Successfully read packet");

        if player.state == ConnectionState::Handshake && packet_id == 0 {
            if let network::NetworkValue::u8(intent) = packet[3] {
                if intent == 2 {
                    player.state = ConnectionState::Login;
                } else {
                    player.state = ConnectionState::Status;
                }
                continue;
            }
        }
        if player.state == ConnectionState::Status {
            if packet_id == 0 {
                // MAJOR TODO: fill in status response packet
                let response = serde_json::json!({
                    "version": {
                        "name": "1.21.10",
                        "protocol": 773,
                    },
                    "players": {
                        "max": 20,
                        "online": 0,
                        // TODO: impl sample players
                        // "sample": [],
                    },
                    "description": {
                        "text": "Hello, World!",
                    },
                    "favicon": "data:image/png;base64,<data>",
                    "enforcesSecureChat": false,
                });
                let response = serde_json::to_string(&response)?;

                let mut response_buffer = vec![];
                network::write_varint(&mut response_buffer, 0 as i32);
                network::write_string(&mut &mut response_buffer, response);

                let mut client_packet = vec![];
                network::write_varint(&mut client_packet, response_buffer.len() as i32);
                client_packet.append(&mut response_buffer);

                let _ = write_packet(&stream, client_packet.as_mut_slice()).await;
            }
            if packet_id == 1 {
                if let network::NetworkValue::Array(timestamp) = &packet[0] {
                    let mut timestamp = timestamp.clone().to_vec();

                    let mut client_packet: Vec<u8> = vec![0x09, 0x01];
                    client_packet.append(&mut timestamp);

                    let _ = write_packet(&stream, client_packet.as_mut_slice()).await;
                }
            }
        }
        if player.state == ConnectionState::Login {
            if packet_id == 0 {
                if let network::NetworkValue::String(name) = &packet[0]
                    && let network::NetworkValue::Array(uuid) = &packet[1]
                {
                    player.name = name.clone();
                    player.uuid = uuid.take(16).into_inner().to_owned().try_into().expect("");
                    debug!("{:?}", player.uuid);

                    let mut profile: Vec<u8> = vec![0x02];
                    profile.append(player.uuid.clone().as_mut_slice().to_vec().as_mut());
                    network::write_string(&mut profile, name.clone().to_string());
                    profile.push(0x00);

                    let mut client_packet: Vec<u8> = vec![];
                    network::write_varint(&mut client_packet, profile.len() as i32);
                    client_packet.append(&mut profile);

                    debug!("Sending Game Profile {:?}", client_packet);
                    let _ = write_packet(&stream, &client_packet).await;
                }
            }
            if packet_id == 3 {
                player.state = ConnectionState::Configuration;
            }
        }
        if player.state == ConnectionState::Configuration {
            // if packet_id == 0 {
            //     player.state = ConnectionState::Play;
            // }
            if packet_id == 2 {
                if packet.len() > 2 {
                    // assume next packet has id 0x00
                    // TODO: properly process multiple packets
                    
                    // TODO: write dimensions
                    // let mut client_packet: Vec<u8> = vec![7, 0];
                    // network::write_string(&mut client_packet, "minecraft:dimension_type".to_string());
                    
                    // let mut entries: Vec<u8> = vec![];
                    // network::write_string(&mut entries, "minecraft:overworld".to_string());
                    // // see https://minecraft.wiki/Java_Edition_protocol/Registry_data#Dimension_Type
                    // entries.append(fastnbt::to_bytes(&1)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&0)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&0)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&1)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&(1 as f32))?.as_mut());
                    // entries.append(fastnbt::to_bytes(&1)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&0)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&-64)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&512)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&512)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&"#infiniburn_overworld")?.as_mut());
                    // entries.append(fastnbt::to_bytes(&"minecraft:overworld")?.as_mut());
                    // entries.append(fastnbt::to_bytes(&(0 as f32))?.as_mut());
                    // entries.append(fastnbt::to_bytes(&0)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&1)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&0)?.as_mut());
                    // entries.append(fastnbt::to_bytes(&0)?.as_mut());
                    
                    // network::write_varint(&mut client_packet, entries.len() as i32);
                    // client_packet.append(&mut entries);

                    // let _ = write_packet(&stream, &client_packet).await;

                    let client_packet: Vec<u8> = vec![3, 0];
                    let _ = write_packet(&stream, &client_packet).await;

                    player.state = ConnectionState::Play;
                }
            }
        }
    }
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:25565").await?;
    let server = Arc::new(Mutex::new(ServerState::default()));
    info!("Starting server...");
    loop {
        let stream: TcpStream = listener.accept().await?.0;
        let server = Arc::clone(&server);
        // TODO: add disconnect if players >= max_players
        info!("Accepted new connection");

        let mut player: Player = Player::default();
        player.entity_id = rand::random_range(1..u32::MAX);

        let player_id = player.entity_id.clone();
        server.lock().await.players.insert(player_id, player);

        tokio::spawn(async move {
            let _ = handle_client(
                stream,
                &mut server.lock().await.players.get_mut(&player_id).expect(""),
            )
            .await;
        });
    }
}
