use std::mem;

pub fn filled_vec<T: Clone>(elem: T, length: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    for _ in 0..length {
        v.push(elem.to_owned());
    }
    v
}

pub fn ints_to_bchannel(t: usize, m: usize, i: usize) -> Vec<u8> {
    filled_vec::<u8>(((m*i) % 255) as u8, t)
}

pub fn to_int(buf: &[u8], max_value: usize) -> usize {
    let arr: [u8; 4] = [buf[0], buf[1], buf[2], buf[3]];
    #[allow(unused_assignments)]
    let mut int: usize = 0;
    unsafe {
        int = mem::transmute::<[u8; 4], u32>(arr) as usize
    };
    let res = int % max_value; // to avoid out of ranges
    res
}
