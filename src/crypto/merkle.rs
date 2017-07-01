use errors::*;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;

pub fn merkle_root(leafs: &Vec<Hash>) -> YResult<Hash> {
    let mut leafs_len = leafs.len();
    if leafs_len == 0 {
        panic!("empty leafs")
    }
    let mut base: Vec<Vec<u8>> = vec![];
    for d in leafs {
        check_hash_size(d)?;
        base.push(d.clone());
    }
    while leafs_len > 1 {
        let mut j = 0;
        let mut i = 0;
        while i < leafs_len {
            j += 1;
            let mut data: Vec<u8> = vec![];
            let h_1 = base[i].clone();
            data.extend(h_1.iter().cloned());
            let h_2 = if i + 1 != leafs_len {
                base[i + 1].clone()
            } else {
                base[i].clone()
            };
            data.extend(h_2.iter().cloned());
            base[j] = hash(data.as_slice())?;
            i += 2;
        }
        leafs_len = j;
    }
    let r = base[0].clone();
    Ok(r)
}

pub fn verify_merkle_root(leafs: &Vec<Hash>, root: &Hash) -> YResult<bool> {
    let mr = merkle_root(leafs)?;
    let ok = mr == *root;
    Ok(ok)
}
