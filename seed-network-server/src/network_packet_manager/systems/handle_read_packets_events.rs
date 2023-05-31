// fn handle_client_handshaking_packets(
//     commands: &mut Commands,
//     entity: Entity,
//     packet: &Packet,
// ) -> Result<(), anyhow::Error> {
//     match FromPrimitive::from_i32(packet.id.value) {
//         Some(ClientHandshakingPackets::Handshake) => {
//             let request = ClientHandshakePacket::try_from(packet)?;
//             info!(target: "systems", "{:?}", request);

//             commands
//                 .entity(entity)
//                 .remove::<ConnectionHandshakingState>();

//             match request.next_state {
//                 NextState::Status(next_packet) => {
//                     commands.entity(entity).insert(ConnectionStatusState);

//                     if let Some(next_packet) = next_packet {
//                         let request = ClientStatusRequestPacket::try_from(&next_packet)?;
//                         info!(target: "systems", "{:?}", request);
//                     }
//                 }
//                 NextState::Login(next_packet) => {
//                     commands.entity(entity).insert(ConnectionLoginState);

//                     let request = ClientLoginStartPacket::try_from(&next_packet)?;
//                     info!(target: "systems", "{:?}", request);
//                 }
//             }
//         }
//         _ => {}
//     }

//     Ok(())
// }

// fn handle_client_status_packets(
//     commands: &mut Commands,
//     entity: Entity,
//     packet: &Packet,
// ) -> Result<(), anyhow::Error> {
//     match FromPrimitive::from_i32(packet.id.value) {
//         Some(ClientStatusPackets::PingRequest) => {
//             let request = ClientPingRequestPacket::try_from(packet)?;
//             info!(target: "systems", "{:?}", request);
//         }
//         _ => println!("handle_client_status_packets: pacote estranho."),
//     }
//     Ok(())
// }

// fn handle_client_login_packets(
//     commands: &mut Commands,
//     entity: Entity,
//     packet: &Packet,
// ) -> Result<(), anyhow::Error> {
//     match FromPrimitive::from_i32(packet.id.value) {
//         Some(ClientLoginPackets::EncryptionResponse) => {
//             println!("EncryptionResponse.");
//         }
//         _ => println!("handle_client_login_packets: pacote estranho."),
//     }

//     Ok(())
// }

// fn handle_client_play_packets(packet: &Packet) -> Result<(), anyhow::Error> {
//     Ok(())
// }
