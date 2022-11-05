pub mod test;

use std::io::Error;

/// CompactSize Unsigned Integers  
///
/// The raw transaction format and several peer-to-peer network messages use a type of variable-length integer to indicate the number of bytes in a following piece of data.
///
/// Bitcoin Core code and this document refers to these variable length integers as compactSize.
/// Many other documents refer to them as var_int or varInt, but this risks conflation with other variable-length integer encodings—such as the CVarInt class used in Bitcoin Core for serializing data to disk.
/// Because it’s used in the transaction format, the format of compactSize unsigned integers is part of the consensus rules.
///
/// https://developer.bitcoin.org/reference/transactions.html#compactsize-unsigned-integers
///
/// https://learnmeabitcoin.com/technical/varint
pub struct VarInt;

impl VarInt {
    /// For numbers from 0 to 252, compactSize unsigned integers look like regular unsigned integers.
    /// For other numbers up to 0xffffffffffffffff, a byte is prefixed to the number to indicate its length—but otherwise the numbers look like regular unsigned integers in little-endian order.
    pub fn encode(size: u64) -> Result<Vec<u8>, Error> {
        let size_bytes = size.to_le_bytes();
        let result = match size {
            x if x <= 252 => vec![size_bytes[0]],
            x if (253..0xffff).contains(&x) => {
                vec![0xfd, size_bytes[0], size_bytes[1]]
            }
            x if (0x10000..0xffffffff).contains(&x) => vec![
                0xfe,
                size_bytes[0],
                size_bytes[1],
                size_bytes[2],
                size_bytes[3],
            ],
            x if (0x100000000..u64::MAX).contains(&x) => {
                let mut x = size_bytes.to_vec();
                x.insert(0, 0xff);
                x
            }
            _ => panic!("VarInt: unexpected input"),
        };
        Ok(result)
    }

    /// For numbers from 0 to 252, compactSize unsigned integers look like regular unsigned integers.
    /// For other numbers up to 0xffffffffffffffff, a byte is prefixed to the number to indicate its length—but otherwise the numbers look like regular unsigned integers in little-endian order.
    pub fn decode(bytes: &[u8]) -> Result<u64, Error> {
        let result = match bytes[0] {
            x if x < 0xfd => u64::from_le_bytes([bytes[0], 0, 0, 0, 0, 0, 0, 0]),
            x if x == 0xfd => u64::from_le_bytes([bytes[1], bytes[2], 0, 0, 0, 0, 0, 0]),
            x if x == 0xfe => {
                u64::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0, 0])
            }
            x if x == 0xff => u64::from_le_bytes([
                bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
            ]),
            _ => panic!("VarInt: unexpected input"),
        };
        Ok(result)
    }

    /// Returns the bytes needed to encode this varint
    pub fn get_size(varint: u64) -> Result<u8, Error> {
        match varint {
            x if x <= 252 => Ok(1),
            x if (253..0xffff).contains(&x) => Ok(3),
            x if (0x10000..0xffffffff).contains(&x) => Ok(5),
            x if (0x10000000..u64::MAX).contains(&x) => Ok(9),
            _ => panic!("VarInt: unexpected input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_encode() {
        assert_eq!(VarInt::encode(515).unwrap(), vec![0xfd, 3, 2]);
    }

    #[test]
    fn test_varint_decode() {
        assert_eq!(VarInt::decode(vec![0xfd, 3, 2]).unwrap(), 515);
    }

    #[test]
    fn test_varint_get_size() {
        assert_eq!(VarInt::get_size(515).unwrap(), 3);
    }
}
