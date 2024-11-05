pub(crate) fn checksum(values: &[u8]) -> u64 {
    crc::Crc::<u64>::new(&crc::CRC_64_REDIS).checksum(values)
}
