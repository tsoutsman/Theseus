//! Forgive me father for I have sinned.

#![feature(exit_status_error)]
#![allow(warnings)]

use std::{env, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut libtheseus_dir = manifest_dir.clone();
    libtheseus_dir.pop();
    libtheseus_dir.push("libtheseus");

    // TODO: The following commented-out code doesn't trigger a rerun if cargo clean
    // is called in libtheseus.
    
    // let mut libtheseus_cargo_toml = libtheseus_dir.clone();
    // libtheseus_cargo_toml.push("Cargo.toml");
    // println!("cargo:rerun-if-changed={}", libtheseus_cargo_toml.display());

    // let mut libtheseus_cargo_lock = libtheseus_dir.clone();
    // libtheseus_cargo_lock.push("Cargo.lock");
    // println!("cargo:rerun-if-changed={}", libtheseus_cargo_lock.display());

    // let mut libtheseus_src = libtheseus_dir.clone();
    // libtheseus_src.push("src");
    // println!("cargo:rerun-if-changed={}", libtheseus_src.display());

    let cargo_bin = env::var("CARGO").unwrap();

    // let mut target_spec = manifest_dir.clone();
    // target_spec.pop();
    // target_spec.push("cfg");
    // target_spec.push("x86_64-theseus.json");

    // std::thread::sleep(std::time::Duration::from_secs(3));

    // panic!("{:#?}", env::vars().collect::<Vec<_>>());

    // let cargo_encoded_rustflags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
    // let (cfg_bootstrap, new_cargo_encoded_rustflags) =
    //     cargo_encoded_rustflags.split_once("\u{1f}").unwrap();
    // println!("new flags: {new_cargo_encoded_rustflags}");
    // assert_eq!(cfg_bootstrap, "--cfg=bootstrap");

    // println!("libtheseus: {}", libtheseus_dir.display());

    Command::new(cargo_bin)
        .arg("build")
        .arg("--release")
        .arg("--target")
        // .arg(target_spec)
        .arg("x86_64-unknown-theseus")
        .arg("-Z")
        .arg("build-std=core,alloc")
        // .env("RUSTFLAGS", "--cfg=bootstrap")
        // .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTC_LINT_FLAGS")
        .env_remove("CARGO_TARGET_DIR")
        // .env("CARGO_ENCODED_RUSTFLAGS", new_cargo_encoded_rustflags)
        .env("__CARGO_TESTS_ONLY_SRC_ROOT", "/home/klim/Projects/rust")
        .env("RUSTC_BOOTSTRAP", "1")
        .current_dir(&libtheseus_dir)
        .status()
        .expect("couldn't get cargo build exit status")
        .exit_ok()
        // TODO: Also capture stderr
        .expect("cargo build failed");

    let mut target_library_dir = libtheseus_dir.clone();
    target_library_dir.push("target");
    target_library_dir.push("x86_64-unknown-theseus");
    target_library_dir.push("release");
    target_library_dir.push("deps");
    println!("cargo:rustc-link-search={}", target_library_dir.display());

    // Proc macro crates are compiled for the host not the target.
    let mut host_library_dir = libtheseus_dir;
    host_library_dir.push("target");
    host_library_dir.push("release");
    host_library_dir.push("deps");
    println!("cargo:rustc-link-search={}", host_library_dir.display());

    // println!("target libraries: {}", target_library_dir.display());
    // println!("host libraries: {}", host_library_dir.display());
}
