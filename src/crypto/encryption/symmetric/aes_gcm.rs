use typenum::{U15, U32};
use generic_array::{GenericArray, ArrayLength};
use libc::c_uchar;
use errors::*;

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
extern "C" {
    pub fn AES256_init(ctx: *mut AES256_ctx, key32: *const c_uchar);

    pub fn AES256_encrypt(
        ctx: *const AES256_ctx,
        blocks: usize,
        cipher16: *mut c_uchar,
        plain16: *const c_uchar,
    );

    pub fn AES256_decrypt(
        ctx: *const AES256_ctx,
        blocks: usize,
        plain16: *mut c_uchar,
        cipher16: *const c_uchar,
    );
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AESGCMState(pub [u16; 8]);

impl AESGCMState {
    fn as_c_repr(&self) -> AES_state {
        AES_state { slice: self.0 }
    }

    fn from_c_repr(repr: AES_state) -> AESGCMState {
        AESGCMState(repr.slice)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AESGCM256(pub GenericArray<AESGCMState, U15>);

impl AESGCM256 {
    fn as_c_repr(&self) -> AES256_ctx {
        let mut arr = [AES_state::default(); 15];
        for i in 0..15 {
            arr[i] = self.0[i].as_c_repr();
        }
        AES256_ctx { rk: arr }
    }

    fn from_c_repr(repr: AES256_ctx) -> AESGCM256 {
        let mut arr = GenericArray::<AESGCMState, U15>::default();
        for i in 0..15 {
            arr[i] = AESGCMState::from_c_repr(repr.rk[i]);
        }
        AESGCM256(arr)
    }
}

pub trait AESGCMCipher {
    type Ctx;
    type KeySize: ArrayLength<u8>;
    fn new(key: GenericArray<u8, Self::KeySize>) -> Self;
    fn encrypt(&mut self, plain: &[u8]) -> YResult<Vec<u8>>;
    fn decrypt(&mut self, ciph: &[u8]) -> YResult<Vec<u8>>;
}

pub type AES256GCMKey = GenericArray<u8, U32>;

impl AESGCMCipher for AESGCM256 {
    type Ctx = AESGCM256;
    type KeySize = U32;

    fn new(key: AES256GCMKey) -> Self {
        let mut ctx = AES256_ctx::default();
        unsafe {
            AES256_init(&mut ctx, key.as_slice().as_ptr());
        }
        AESGCM256::from_c_repr(ctx)
    }

    fn encrypt(&mut self, plain: &[u8]) -> YResult<Vec<u8>> {
        let ctx = self.as_c_repr();
        let len = plain.len();
        if len % 16 != 0 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let blocks = len / 16;
        let mut ciph = Vec::new();
        for _ in 0..blocks {
            ciph.extend_from_slice(&[0u8; 16][..]);
        }
        unsafe {
            AES256_encrypt(&ctx, blocks, ciph.as_mut_ptr(), plain.as_ptr());
        }
        Ok(ciph)
    }

    fn decrypt(&mut self, ciph: &[u8]) -> YResult<Vec<u8>> {
        let ctx = self.as_c_repr();
        let len = ciph.len();
        if len % 16 != 0 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let blocks = len / 16;
        let mut plain = Vec::new();
        for _ in 0..blocks {
            plain.extend_from_slice(&[0u8; 16][..]);
        }
        unsafe {
            AES256_decrypt(&ctx, blocks, plain.as_mut_ptr(), ciph.as_ptr());
        }
        Ok(plain)
    }
}
