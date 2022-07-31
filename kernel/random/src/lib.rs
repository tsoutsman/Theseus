//! A crate for generating random numbers.

#![no_std]

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use spin::Mutex;

// TODO: Ideally we'd use core::cell::LazyCell (or core::lazy::Lazy, or whatever
// they choose to call it), but for some reason it doesn't yet allow mutable
// access despite being !Sync.
lazy_static::lazy_static! {
    static ref CSPRNG: Mutex<ChaCha20Rng> = {
        let seed = rdseed_seed()
            .or_else(|_| rdrand_seed())
            .unwrap_or_else(|_| {
                log::error!("Could not generate hardware seed - using a predetermined seed. THIS IS NOT OK.");
                [1; 32]
            });
        Mutex::new(ChaCha20Rng::from_seed(seed))
    };
}

/// Tries to generate a 32 byte seed using the RDSEED x86 instruction.
fn rdseed_seed() -> Result<[u8; 32], ()> {
    match rdrand::RdSeed::new() {
        Ok(mut generator) => {
            let mut seed = [0; 32];
            match generator.try_fill_bytes(&mut seed) {
                Ok(_) => {
                    log::info!("using RDSEED for CSPRNG seed");
                    Ok(seed)
                }
                Err(_) => {
                    log::warn!("failed to generate value from RDSEED");
                    Err(())
                }
            }
        }
        Err(_) => {
            log::warn!("failed to initialise RDSEED");
            Err(())
        }
    }
}

/// Tries to generate a 32 byte seed using the RDRAND x86 instruction.
fn rdrand_seed() -> Result<[u8; 32], ()> {
    match rdrand::RdRand::new() {
        Ok(mut generator) => {
            let mut seed = [0; 32];
            match generator.try_fill_bytes(&mut seed) {
                Ok(_) => {
                    log::info!("using RDRAND for CSPRNG seed");
                    Ok(seed)
                }
                Err(_) => {
                    log::warn!("failed to generate value from RDRAND");
                    Err(())
                }
            }
        }
        Err(_) => {
            log::warn!("failed to initialise RDRAND");
            Err(())
        }
    }
}

/// Returns a random [`u32`].
pub fn next_u32() -> u32 {
    let mut csprng = CSPRNG.lock();
    csprng.next_u32()
}

/// Returns a random [`u64`].
pub fn next_u64() -> u64 {
    let mut csprng = CSPRNG.lock();
    csprng.next_u64()
}

/// Fills `dest` with random data.
pub fn fill_bytes(dest: &mut [u8]) {
    let mut csprng = CSPRNG.lock();
    csprng.fill_bytes(dest);
}
