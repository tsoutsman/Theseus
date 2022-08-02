#![feature(exit_status_error)]

use std::{env, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut libtheseus_dir = manifest_dir.clone();
    libtheseus_dir.pop();
    libtheseus_dir.push("libtheseus");

    let mut libtheseus_cargo_toml = libtheseus_dir.clone();
    libtheseus_cargo_toml.push("Cargo.toml");
    println!("cargo:rerun-if-changed={}", libtheseus_cargo_toml.display());

    let mut libtheseus_cargo_lock = libtheseus_dir.clone();
    libtheseus_cargo_lock.push("Cargo.lock");
    println!("cargo:rerun-if-changed={}", libtheseus_cargo_lock.display());

    let mut libtheseus_src = libtheseus_dir.clone();
    libtheseus_src.push("src");
    println!("cargo:rerun-if-changed={}", libtheseus_src.display());

    // Prevents multiple libtheseus rlib files with different hashes.
    Command::new("cargo")
        .arg("clean")
        .current_dir(&libtheseus_dir)
        .status()
        .expect("couldn't get cargo clean exit status")
        .exit_ok()
        // TODO: Also capture stderr
        .expect("cargo clean failed");

    let mut target_spec = manifest_dir.clone();
    target_spec.pop();
    target_spec.push("cfg");
    target_spec.push("x86_64-theseus.json");

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg(target_spec)
        .arg("-Z")
        .arg("build-std=core,alloc")
        .current_dir(&libtheseus_dir)
        .status()
        .expect("couldn't get cargo build exit status")
        .exit_ok()
        // TODO: Also capture stderr
        .expect("cargo build failed");

    let mut target_library_dir = libtheseus_dir.clone();
    target_library_dir.push("target");
    target_library_dir.push("x86_64-theseus");
    target_library_dir.push("release");
    target_library_dir.push("deps");
    println!("cargo:rustc-link-search={}", target_library_dir.display());

    // Proc macro crates are compiled for the host not the target.
    let mut host_library_dir = libtheseus_dir;
    host_library_dir.push("target");
    host_library_dir.push("release");
    host_library_dir.push("deps");
    println!("cargo:rustc-link-search={}", host_library_dir.display());

    println!("target libraries: {}", target_library_dir.display());
    println!("host libraries: {}", host_library_dir.display());
}
