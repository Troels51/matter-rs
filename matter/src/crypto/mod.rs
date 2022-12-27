/*
 *
 *    Copyright (c) 2020-2022 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use crate::error::Error;

pub const SYMM_KEY_LEN_BITS: usize = 128;
pub const SYMM_KEY_LEN_BYTES: usize = SYMM_KEY_LEN_BITS / 8;

pub const AEAD_MIC_LEN_BITS: usize = 128;
pub const AEAD_MIC_LEN_BYTES: usize = AEAD_MIC_LEN_BITS / 8;

pub const AEAD_NONCE_LEN_BYTES: usize = 13;
pub const AEAD_AAD_LEN_BYTES: usize = 8;

pub const SHA256_HASH_LEN_BYTES: usize = 256 / 8;

pub const BIGNUM_LEN_BYTES: usize = 32;
pub const EC_POINT_LEN_BYTES: usize = 65;

pub const ECDH_SHARED_SECRET_LEN_BYTES: usize = 32;

pub const EC_SIGNATURE_LEN_BYTES: usize = 64;

// APIs particular to a KeyPair so a KeyPair object can be defined
pub trait CryptoKeyPair {
    fn get_csr<'a>(&self, csr: &'a mut [u8]) -> Result<&'a [u8], Error>;
    fn get_public_key(&self, pub_key: &mut [u8]) -> Result<usize, Error>;
    fn get_private_key(&self, priv_key: &mut [u8]) -> Result<usize, Error>;
    fn derive_secret(self, peer_pub_key: &[u8], secret: &mut [u8]) -> Result<usize, Error>;
    fn sign_msg(&self, msg: &[u8], signature: &mut [u8]) -> Result<usize, Error>;
    fn verify_msg(&self, msg: &[u8], signature: &[u8]) -> Result<(), Error>;
}

#[cfg(feature = "crypto_esp_mbedtls")]
mod crypto_esp_mbedtls;
#[cfg(feature = "crypto_esp_mbedtls")]
pub use self::crypto_esp_mbedtls::*;

#[cfg(feature = "crypto_mbedtls")]
mod crypto_mbedtls;
#[cfg(feature = "crypto_mbedtls")]
pub use self::crypto_mbedtls::*;

#[cfg(feature = "crypto_openssl")]
mod crypto_openssl;
#[cfg(feature = "crypto_openssl")]
pub use self::crypto_openssl::*;

pub mod crypto_dummy;

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::{CryptoKeyPair, KeyPair};

    #[test]
    fn test_verify_msg_success() {
        let key = KeyPair::new_from_public(&test_vectors::PUB_KEY1).unwrap();
        key.verify_msg(&test_vectors::MSG1_SUCCESS, &test_vectors::SIGNATURE1)
            .unwrap();
    }
    #[test]
    fn test_verify_msg_fail() {
        let key = KeyPair::new_from_public(&test_vectors::PUB_KEY1).unwrap();
        assert_eq!(
            key.verify_msg(&test_vectors::MSG1_FAIL, &test_vectors::SIGNATURE1),
            Err(Error::InvalidSignature)
        );
    }

    mod test_vectors {
        pub const PUB_KEY1: [u8; 65] = [
            0x4, 0x56, 0x19, 0x77, 0x18, 0x3f, 0xd4, 0xff, 0x2b, 0x58, 0x3d, 0xe9, 0x79, 0x34,
            0x66, 0xdf, 0xe9, 0x0, 0xfb, 0x6d, 0xa1, 0xef, 0xe0, 0xcc, 0xdc, 0x77, 0x30, 0xc0,
            0x6f, 0xb6, 0x2d, 0xff, 0xbe, 0x54, 0xa0, 0x95, 0x75, 0xb, 0x8b, 0x7, 0xbc, 0x55, 0xdb,
            0x9c, 0xb6, 0x55, 0x13, 0x8, 0xb8, 0xdf, 0x2, 0xe3, 0x40, 0x6b, 0xae, 0x34, 0xf5, 0xc,
            0xba, 0xc9, 0xf2, 0xbf, 0xf1, 0xe7, 0x50,
        ];
        pub const MSG1_SUCCESS: [u8; 421] = [
            0x30, 0x82, 0x1, 0xa1, 0xa0, 0x3, 0x2, 0x1, 0x2, 0x2, 0x1, 0x1, 0x30, 0xa, 0x6, 0x8,
            0x2a, 0x86, 0x48, 0xce, 0x3d, 0x4, 0x3, 0x2, 0x30, 0x44, 0x31, 0x20, 0x30, 0x1e, 0x6,
            0xa, 0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x3, 0xc, 0x10, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x31, 0x31,
            0x20, 0x30, 0x1e, 0x6, 0xa, 0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x5, 0xc,
            0x10, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x31, 0x30, 0x1e, 0x17, 0xd, 0x32, 0x31, 0x30, 0x31, 0x30, 0x31, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x17, 0xd, 0x33, 0x30, 0x31, 0x32, 0x33, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x44, 0x31, 0x20, 0x30, 0x1e, 0x6, 0xa,
            0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x1, 0xc, 0x10, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x42, 0x43, 0x35, 0x43, 0x30, 0x32, 0x31,
            0x20, 0x30, 0x1e, 0x6, 0xa, 0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x5, 0xc,
            0x10, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x31, 0x30, 0x59, 0x30, 0x13, 0x6, 0x7, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x2,
            0x1, 0x6, 0x8, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x3, 0x1, 0x7, 0x3, 0x42, 0x0, 0x4, 0x6,
            0x47, 0xf2, 0x86, 0x4d, 0x27, 0x25, 0xdc, 0x1, 0xa, 0x87, 0xde, 0x8d, 0xca, 0x88, 0x37,
            0xcb, 0x3b, 0xd0, 0xea, 0x93, 0xa6, 0x24, 0x65, 0x8, 0x8f, 0xa1, 0x75, 0xc2, 0xd4,
            0x41, 0xfa, 0xca, 0x96, 0x54, 0xa3, 0xd8, 0x10, 0x85, 0x73, 0xce, 0x15, 0xa5, 0x38,
            0xc1, 0xe3, 0xb5, 0x6b, 0x61, 0x1, 0xd3, 0xc4, 0xb7, 0x6b, 0x61, 0x16, 0xc3, 0x77,
            0x8d, 0xe9, 0xb5, 0x44, 0xac, 0x14, 0xa3, 0x81, 0x83, 0x30, 0x81, 0x80, 0x30, 0xc, 0x6,
            0x3, 0x55, 0x1d, 0x13, 0x1, 0x1, 0xff, 0x4, 0x2, 0x30, 0x0, 0x30, 0xe, 0x6, 0x3, 0x55,
            0x1d, 0xf, 0x1, 0x1, 0xff, 0x4, 0x4, 0x3, 0x2, 0x7, 0x80, 0x30, 0x20, 0x6, 0x3, 0x55,
            0x1d, 0x25, 0x1, 0x1, 0xff, 0x4, 0x16, 0x30, 0x14, 0x6, 0x8, 0x2b, 0x6, 0x1, 0x5, 0x5,
            0x7, 0x3, 0x2, 0x6, 0x8, 0x2b, 0x6, 0x1, 0x5, 0x5, 0x7, 0x3, 0x1, 0x30, 0x1d, 0x6, 0x3,
            0x55, 0x1d, 0xe, 0x4, 0x16, 0x4, 0x14, 0xbd, 0xfd, 0x11, 0xac, 0x89, 0xb6, 0xe0, 0x90,
            0x7a, 0xf6, 0x12, 0x61, 0x78, 0x4d, 0x3d, 0x79, 0x56, 0xeb, 0xc2, 0xdc, 0x30, 0x1f,
            0x6, 0x3, 0x55, 0x1d, 0x23, 0x4, 0x18, 0x30, 0x16, 0x80, 0x14, 0xce, 0x60, 0xb4, 0x28,
            0x96, 0x72, 0x27, 0x64, 0x81, 0xbc, 0x4f, 0x0, 0x78, 0xa3, 0x30, 0x48, 0xfe, 0x6e,
            0x65, 0x86,
        ];
        pub const MSG1_FAIL: [u8; 421] = [
            0x30, 0x82, 0x1, 0xa1, 0xa0, 0x3, 0x2, 0x1, 0x2, 0x2, 0x1, 0x1, 0x30, 0xa, 0x6, 0x8,
            0x2a, 0x86, 0x48, 0xce, 0x3d, 0x4, 0x3, 0x2, 0x30, 0x44, 0x31, 0x20, 0x30, 0x1e, 0x6,
            0xa, 0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x3, 0xc, 0x10, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x31, 0x31,
            0x20, 0x30, 0x1e, 0x6, 0xa, 0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x5, 0xc,
            0x10, 0x30, 0x30, 0x30, 0x31, 0x32, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x31, 0x30, 0x1e, 0x17, 0xd, 0x32, 0x31, 0x30, 0x31, 0x30, 0x31, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x17, 0xd, 0x33, 0x30, 0x31, 0x32, 0x33, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x44, 0x31, 0x20, 0x30, 0x1e, 0x6, 0xa,
            0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x1, 0xc, 0x10, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x42, 0x43, 0x35, 0x43, 0x30, 0x32, 0x31,
            0x20, 0x30, 0x1e, 0x6, 0xa, 0x2b, 0x6, 0x1, 0x4, 0x1, 0x82, 0xa2, 0x7c, 0x1, 0x5, 0xc,
            0x10, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x31, 0x30, 0x59, 0x30, 0x13, 0x6, 0x7, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x2,
            0x1, 0x6, 0x8, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x3, 0x1, 0x7, 0x3, 0x42, 0x0, 0x4, 0x6,
            0x47, 0xf2, 0x86, 0x4d, 0x27, 0x25, 0xdc, 0x1, 0xa, 0x87, 0xde, 0x8d, 0xca, 0x88, 0x37,
            0xcb, 0x3b, 0xd0, 0xea, 0x93, 0xa6, 0x24, 0x65, 0x8, 0x8f, 0xa1, 0x75, 0xc2, 0xd4,
            0x41, 0xfa, 0xca, 0x96, 0x54, 0xa3, 0xd8, 0x10, 0x85, 0x73, 0xce, 0x15, 0xa5, 0x38,
            0xc1, 0xe3, 0xb5, 0x6b, 0x61, 0x1, 0xd3, 0xc4, 0xb7, 0x6b, 0x61, 0x16, 0xc3, 0x77,
            0x8d, 0xe9, 0xb5, 0x44, 0xac, 0x14, 0xa3, 0x81, 0x83, 0x30, 0x81, 0x80, 0x30, 0xc, 0x6,
            0x3, 0x55, 0x1d, 0x13, 0x1, 0x1, 0xff, 0x4, 0x2, 0x30, 0x0, 0x30, 0xe, 0x6, 0x3, 0x55,
            0x1d, 0xf, 0x1, 0x1, 0xff, 0x4, 0x4, 0x3, 0x2, 0x7, 0x80, 0x30, 0x20, 0x6, 0x3, 0x55,
            0x1d, 0x25, 0x1, 0x1, 0xff, 0x4, 0x16, 0x30, 0x14, 0x6, 0x8, 0x2b, 0x6, 0x1, 0x5, 0x5,
            0x7, 0x3, 0x2, 0x6, 0x8, 0x2b, 0x6, 0x1, 0x5, 0x5, 0x7, 0x3, 0x1, 0x30, 0x1d, 0x6, 0x3,
            0x55, 0x1d, 0xe, 0x4, 0x16, 0x4, 0x14, 0xbd, 0xfd, 0x11, 0xac, 0x89, 0xb6, 0xe0, 0x90,
            0x7a, 0xf6, 0x12, 0x61, 0x78, 0x4d, 0x3d, 0x79, 0x56, 0xeb, 0xc2, 0xdc, 0x30, 0x1f,
            0x6, 0x3, 0x55, 0x1d, 0x23, 0x4, 0x18, 0x30, 0x16, 0x80, 0x14, 0xce, 0x60, 0xb4, 0x28,
            0x96, 0x72, 0x27, 0x64, 0x81, 0xbc, 0x4f, 0x0, 0x78, 0xa3, 0x30, 0x48, 0xfe, 0x6e,
            0x65, 0x86,
        ];
        pub const SIGNATURE1: [u8; 64] = [
            0x20, 0x16, 0xd0, 0x13, 0x1e, 0xd0, 0xb3, 0x9d, 0x44, 0x25, 0x16, 0xea, 0x9c, 0xf2,
            0x72, 0x44, 0xd7, 0xb0, 0xf4, 0xae, 0x4a, 0xa4, 0x37, 0x32, 0xcd, 0x6a, 0x79, 0x7a,
            0x4c, 0x48, 0x3, 0x6d, 0xef, 0xe6, 0x26, 0x82, 0x39, 0x28, 0x9, 0x22, 0xc8, 0x9a, 0xde,
            0xd5, 0x13, 0x9f, 0xc5, 0x40, 0x25, 0x85, 0x2c, 0x69, 0xe0, 0xdb, 0x6a, 0x79, 0x5b,
            0x21, 0x82, 0x13, 0xb0, 0x20, 0xb9, 0x69,
        ];
    }
}