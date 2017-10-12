use typenum::{U15, U32};
use generic_array::{GenericArray, ArrayLength};
use libc::c_uchar;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct AES_state {
  pub slice: [u16; 8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct AES256_ctx {
    pub rk: [AES_state; 15],
}

#[link_name = "ctaes"]
extern {
    pub fn AES256_init(ctx: *mut AES256_ctx,
                       key32: *const c_uchar);

    pub fn AES256_encrypt(ctx: *const AES256_ctx, blocks: usize,
                          cipher16: *mut c_uchar,
                          plain16: *const c_uchar);

    pub fn AES256_decrypt(ctx: *const AES256_ctx, blocks: usize,
                          plain16: *mut c_uchar,
                          cipher16: *const c_uchar);
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AESGCMState(pub [u16; 8]);

impl AESGCMState {
  fn as_c_repr(&self) -> AES_state {
    unreachable!()
  }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AESGCM256(pub GenericArray<AESGCMState, U15>);

impl AESGCM256 {
  fn as_c_repr(&self) -> AES256_ctx {
    unreachable!()
  }
}

pub trait AESGCMCipher {
    type Ctx;
    type KeySize: ArrayLength<u8>;
    fn new(key: GenericArray<u8, Self::KeySize>) -> Self;
    fn encrypt(&mut self, plain: &[u8]) -> Vec<u8>;
    fn decrypt(&mut self, ciph: &[u8]) -> Vec<u8>;
}

impl AESGCMCipher for AESGCM256 {
    type Ctx = AESGCM256;
    type KeySize = U32;
    fn new(key: GenericArray<u8, Self::KeySize>) -> Self {
        let mut ctx = AES256_ctx::default();
        unsafe {
          //AES256_init(*mut ctx, key.as_slice().as_ptr());
        }
        unreachable!()
    }
    fn encrypt(&mut self, plain: &[u8]) -> Vec<u8> {
        let mut ctx = self.as_c_repr();
        let len = plain.len();
        let blocks = len % 16 + 1;
        let pad_count = 16 - (len / 16);
        let mut plain_padded = Vec::new();
        plain_padded.extend_from_slice(plain);
        for _ in 0..pad_count {
          plain_padded.insert(0, 0);
        }
        let mut ciph = Vec::new();
        for i in 0..blocks {
          ciph.extend_from_slice(&[0u8; 16][..]);
        }
        unsafe {
          AES256_encrypt(&ctx, blocks, ciph.as_mut_ptr(), plain_padded.as_ptr());
        }
        ciph
    }
    fn decrypt(&mut self, ciph: &[u8]) -> Vec<u8> {
        let mut ctx = self.as_c_repr();
        let len = ciph.len();
        if len % 16 != 0 {
          panic!("Invalid length"); // TODO: solve it
        }
        let blocks = len / 16;
        let mut plain = Vec::new();
        for i in 0..blocks {
          plain.extend_from_slice(&[0u8; 16][..]);
        }
        unsafe {
          AES256_decrypt(&ctx, blocks, plain.as_mut_ptr(), ciph.as_ptr());
        }
        let mut plain_unpadded = plain.clone();
        for i in 0..len {
          // TODO: check where big_endian
          if plain_unpadded[i] == 0 {
            plain_unpadded.remove(i);
          }
        }
        plain_unpadded
    }
}
