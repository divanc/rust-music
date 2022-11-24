// WAV file consists of 4 chunks: RIFF, fmt, data, and fact.
pub trait Chunk {
    // Get the chunk ID as a big-endian array of 4 bytes
    fn get_be_id(&self) -> &[u8; 4];
    // Get the chunk size as a little-endian u32
    fn get_le_size(&self) -> u32;
}
