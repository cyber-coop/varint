#[cfg(test)]
mod tests {
    use crate::VarInt;

    #[test]
    fn test_varint_encode() {
        assert_eq!(VarInt::encode(515).unwrap(), vec![0xfd, 3, 2]);
    }

    #[test]
    fn test_varint_decode() {
        assert_eq!(VarInt::decode(&vec![0xfd, 3, 2]).unwrap(), 515);
    }

    #[test]
    fn test_varint_get_size() {
        assert_eq!(VarInt::get_size(515).unwrap(), 3);
    }
}
