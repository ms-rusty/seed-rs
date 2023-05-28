use bevy::prelude::{Component, Entity};
use bytes::{Bytes, BytesMut};
use seed_network_server_common::VarInt;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::tcp::OwnedWriteHalf,
};

use super::{packet_reader::PacketReader, packet_writer::PacketWriter};

pub async fn write_packet(
    connection_writer: &mut BufWriter<OwnedWriteHalf>,
    packet: Packet,
) -> Result<(), anyhow::Error> {
    let packet_id = packet.id;
    let data_length = packet.data.len();
    let packet_length = packet_id.position + data_length;

    let mut writer = PacketWriter::new();
    writer.write_var_int(packet_length as i32);
    writer.write_var_int(packet_id.value);
    writer.write_bytes(packet.data.as_ref());

    connection_writer.write_all(&writer.get_data()).await?;

    Ok(())
}
