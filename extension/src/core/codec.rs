use bytes::BufMut;
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use std::io;

pub async fn decode<T: Message + Default, R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<T, io::Error> {
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes).await?;
    let len = u32::from_le_bytes(len_bytes) as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    T::decode(buf.as_slice())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn encode<T: Message>(msg: &T) -> Result<Vec<u8>, io::Error> {
    let mut buf = Vec::new();
    let mut encoded = Vec::new();
    msg.encode(&mut encoded)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    buf.put_u32_le(encoded.len() as u32);
    buf.put_slice(&encoded);
    Ok(buf)
}