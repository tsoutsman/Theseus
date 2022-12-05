use std::{env, path::PathBuf, process::Command};

// We put the feature checks here because the build script will give unhelpful
// errors if it's built with the wrong combination of features.
//
// We prefer BIOS over UEFI to avoid mutually exclusive features as they mess up
// building with --all-features.
// https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features

cfg_if::cfg_if! {
    if #[cfg(feature = "bios")] {
        const SPECIFICATION: &str = "bios";
    } else if #[cfg(feature = "uefi")] {
        const SPECIFICATION: &str = "uefi";
    } else {
        compile_error!("either the bios or uefi features must be enabled");
    }
}

fn main() {
    compile_asm();
}

fn compile_asm() {
    let out_dir = match env::var("THESEUS_NANO_CORE_BUILD_DIR") {
        Ok(out_dir) => PathBuf::from(out_dir),
        // nano core is being compiled for docs or clippy
        Err(_) => std::env::temp_dir(),
    }
    .join("compiled_asm")
    .join(SPECIFICATION);
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("failed to create compiled_asm directory: {e}");
        }
    }
    let include_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("asm");

    println!("cargo:rerun-if-changed={}", include_path.display());
    // TODO: This recompiles the assembly files every time.
    println!("cargo:rerun-if-changed={}", out_dir.display());

    let asm_path = include_path.join(SPECIFICATION);

    let cflags = env::var("THESEUS_CFLAGS").unwrap_or_default();

    for file in include_path
        .read_dir()
        .expect("failed to open include directory")
        .chain(asm_path.read_dir().expect("failed to open asm directory"))
    {
        let file = file.expect("failed to read asm file");
        if file.file_type().expect("couldn't get file type").is_file() {
            assert_eq!(file.path().extension(), Some("asm".as_ref()));

            let mut output_path = out_dir.join(file.path().file_name().unwrap());
            assert!(output_path.set_extension("o"));

            assert!(Command::new("nasm")
                .args(["-f", "elf64"])
                .arg("-i")
                .arg(&include_path)
                .arg("-o")
                .arg(&output_path)
                .arg(file.path())
                .args(cflags.split(' '))
                .status()
                .expect("failed to acquire nasm output status")
                .success());
        }
    }
}
