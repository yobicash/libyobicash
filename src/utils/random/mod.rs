use rand::{random, thread_rng, sample};

pub struct YRandom;

impl YRandom {
    pub fn u32() -> u32 {
        random::<u32>()
    }

    pub fn u64() -> u64 {
        random::<u64>()
    }

    pub fn u32_range(from: u32, to: u32) -> u32 {
        let mut rng = thread_rng();
        sample(&mut rng, from..to, 1)[0]
    }

    pub fn u32_sample(from: u32, to: u32, n: u32) -> Vec<u32> {
        let mut rng = thread_rng();
        sample(&mut rng, from..to, n as usize)
    }

    pub fn u64_range(from: u64, to: u64) -> u64 {
        let mut rng = thread_rng();
        sample(&mut rng, from..to, 1)[0]
    }

    pub fn u64_sample(from: u64, to: u64, n: u64) -> Vec<u64> {
        let mut rng = thread_rng();
        sample(&mut rng, from..to, n as usize)
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
