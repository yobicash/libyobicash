use rand::random;

pub struct Random;

impl Random {
    pub fn u32() -> u32 {
        random::<u32>()
    }

    pub fn u64() -> u64 {
        random::<u64>()
    }

    pub fn u32_range(len: u32) -> Vec<u32> {
        let mut v = Vec::new();
        for _ in 0..len {
            v.push(random::<u32>());
        }
        v
    }

    pub fn u64_range(len: u32) -> Vec<u64> {
        let mut v = Vec::new();
        for _ in 0..len {
            v.push(random::<u64>());
        }
        v
    }

    pub fn bytes_mut(sl: &mut [u8]) {
        for i in 0..sl.len() {
            sl[i] = random::<u8>();
        }
    }

    pub fn bytes(len: u32) -> Vec<u8> {
        let mut v = Vec::new();
        for _ in 0..len {
            v.push(random::<u8>());
        }
        v
    }
}
