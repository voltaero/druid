use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
const VERSION: u8 = 0;

#[inline]
fn bytes_to_hex(bytes: &[u8]) -> String {
    const LUT: &[u8; 16] = b"0123456789abcdef";
    let mut out = vec![0u8; bytes.len() * 2];
    for (i, &b) in bytes.iter().enumerate() {
        out[i * 2] = LUT[(b >> 4) as usize];
        out[i * 2 + 1] = LUT[(b & 0x0f) as usize];
    }
    // Safety: `out` is guaranteed ASCII (valid UTF-8).
    unsafe { String::from_utf8_unchecked(out) }
}

pub struct Druid {
    pub id: [u8; 40],
}
impl Default for Druid {
    #[inline]
    fn default() -> Self {
        let timestamp: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut id = [0u8; 40];
        id[0..16].copy_from_slice(&timestamp.to_be_bytes());
        let mut rng = rand::rng();
        rng.fill_bytes(&mut id[16..39]);
        id[39] = VERSION; //version byte
        Self { id }
    }
}
impl Druid {
    #[inline]
    pub fn to_hex(&self) -> String {
        bytes_to_hex(&self.id)
    }
}

pub struct DruidV7 {
    pub id: [u8; 16],
}
impl Default for DruidV7 {
    #[inline]
    fn default() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp_ms: u64 =
            now.as_secs().saturating_mul(1_000) + u64::from(now.subsec_millis());
        let timestamp = timestamp_ms.to_be_bytes();

        let mut bytes: [u8; 16] = [0u8; 16];
        bytes[0..6].copy_from_slice(&timestamp[2..8]); // 48-bit ms timestamp

        let mut rng = rand::rng();
        rng.fill_bytes(&mut bytes[6..16]);
        bytes[6] = (bytes[6] & 0b00001111) | 0b01110000;
        bytes[8] = (bytes[8] & 0b00111111) | 0b10000000;
        Self { id: bytes }
    }
}
impl DruidV7 {
    #[inline]
    pub fn to_hex(&self) -> String {
        bytes_to_hex(&self.id)
    }
}
