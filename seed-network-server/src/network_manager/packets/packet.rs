use bevy::prelude::info;
use bytes::{Bytes, BytesMut};
use seed_network_server_common::VarInt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use super::{packet_reader::PacketReader, packet_writer::PacketWriter};

// https://wiki.vg/Protocol#Packet_format
#[derive(Debug)]
pub struct Packet {
    pub id: VarInt,
    pub data: Bytes,
}

pub async fn read_packet(
    connection_reader: &mut BufReader<OwnedReadHalf>,
) -> anyhow::Result<Packet, anyhow::Error> {
    let mut buffer = BytesMut::with_capacity(4 * 1024); // TODO: check later
    connection_reader.read_buf(&mut buffer).await?;

    if buffer.is_empty() {
        return Err(anyhow::anyhow!("buffer empty!"));
    }

    let mut reader = PacketReader::new(&buffer);
    let packet_length = reader.read_var_int()?;
    let packet_id = reader.read_var_int()?;

    let buffer = buffer
        .split_off(packet_length.position + packet_id.position)
        .freeze();

    info!(target: "packets", "RECV [{}] [{:03X}] {:02x?}", packet_length.value, packet_id.value, &buffer[..]);

    Ok(Packet {
        id: packet_id,
        data: buffer,
    })
}

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
