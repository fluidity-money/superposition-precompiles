#![cfg_attr(not(feature = "std"), no_std, no_main)]

use bobcat_sdk::prelude::*;

use ed25519_dalek::{Signature, VerifyingKey};

use sha2::digest::{
    generic_array::{typenum::U64, GenericArray},
    Digest, FixedOutput, FixedOutputReset, OutputSizeUser, Reset, Update,
};

#[derive(Debug, Clone, Copy)]
struct PrecomputedSha512([u8; 64]);

impl OutputSizeUser for PrecomputedSha512 {
    type OutputSize = U64;
}

impl Update for PrecomputedSha512 {
    fn update(&mut self, data: &[u8]) {
        let len = data.len().min(64);
        self.0[..len].copy_from_slice(&data[..len]);
    }
}

impl FixedOutput for PrecomputedSha512 {
    fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
        *out = GenericArray::from(self.0);
    }
}

impl Reset for PrecomputedSha512 {
    fn reset(&mut self) {
        self.0 = [0u8; 64];
    }
}

impl FixedOutputReset for PrecomputedSha512 {
    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        *out = GenericArray::from(self.0);
        Reset::reset(self);
    }
}

impl Digest for PrecomputedSha512 {
    fn new() -> Self {
        PrecomputedSha512([0u8; 64])
    }

    fn new_with_prefix(_data: impl AsRef<[u8]>) -> Self {
        unimplemented!()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        Update::update(self, data.as_ref());
    }

    fn chain_update(self, _data: impl AsRef<[u8]>) -> Self {
        unimplemented!()
    }

    fn finalize(self) -> GenericArray<u8, Self::OutputSize> {
        GenericArray::from(self.0)
    }

    fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
        FixedOutput::finalize_into(self, out);
    }

    fn finalize_reset(&mut self) -> GenericArray<u8, Self::OutputSize>
    where
        Self: FixedOutputReset,
    {
        let result = GenericArray::from(self.0);
        Reset::reset(self);
        result
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>)
    where
        Self: FixedOutputReset,
    {
        FixedOutputReset::finalize_into_reset(self, out);
    }

    fn reset(&mut self)
    where
        Self: Reset,
    {
        Reset::reset(self);
    }

    fn output_size() -> usize {
        64
    }

    fn digest(data: impl AsRef<[u8]>) -> GenericArray<u8, Self::OutputSize> {
        let bytes = data.as_ref();
        let mut result = [0u8; 64];
        let len = bytes.len().min(64);
        result[..len].copy_from_slice(&bytes[..len]);
        GenericArray::from(result)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = read_args_safe!(args_len, { 32 * 5 });
    let digest = PrecomputedSha512(args[..32 * 2].try_into().unwrap());
    let key: [u8; 32] = args[32 * 2..32 * 3].try_into().unwrap();
    let key = VerifyingKey::from_bytes(&key).unwrap();
    let sig = Signature::from_bytes(&args[32 * 3..32 * 5].try_into().unwrap());
    key.verify_prehashed_strict(digest, None, &sig).unwrap();
    0
}

#[allow(unused)]
fn main() {}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "std"))]
mod test {
    use proptest::prelude::*;

    use super::*;

    use ed25519_dalek::{Sha512, SigningKey};

    use array_concat::concat_arrays;

    proptest! {
        #[test]
        fn test_ed25519(
            key in any::<[u8; 32]>(),
            data in proptest::collection::vec(any::<u8>(), 0..10000)
        ) {
            entry_host::set_args(vec![]);
            let key = SigningKey::from_bytes(&key);
            let mut d = Sha512::new();
            sha2::digest::Update::update(&mut d, &data);
            let sig = key.sign_prehashed(d.clone(), None).unwrap();
            let d: [u8; 64] = d.finalize().into();
            let key: [u8; 32] = key.verifying_key().to_bytes();
            let sig: [u8; 64] = sig.to_bytes();
            let x: [u8; 32 * 5] = concat_arrays!(d, key, sig);
            entry_host::set_args(x.to_vec());
            unsafe { user_entrypoint(x.len()) };
        }
    }
}
