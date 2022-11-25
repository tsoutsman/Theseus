use std::{env, io::Write, path::PathBuf, process::Command};

/// The prefix that all custom rustc-known cfg keys are given by cargo
/// when it transforms them into environment variables.
const CARGO_CFG_PREFIX: &'static str = "CARGO_CFG_";

/// The set of built-in environment variables defined by cargo.
static NON_CUSTOM_CFGS: [&str; 12] = [
    "CARGO_CFG_PANIC",
    "CARGO_CFG_TARGET_ABI",
    "CARGO_CFG_TARGET_ARCH",
    "CARGO_CFG_TARGET_ENDIAN",
    "CARGO_CFG_TARGET_ENV",
    "CARGO_CFG_TARGET_FEATURE",
    "CARGO_CFG_TARGET_HAS_ATOMIC",
    "CARGO_CFG_TARGET_HAS_ATOMIC_EQUAL_ALIGNMENT",
    "CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE",
    "CARGO_CFG_TARGET_OS",
    "CARGO_CFG_TARGET_POINTER_WIDTH",
    "CARGO_CFG_TARGET_VENDOR",
];

fn main() {
    compile_asm();
    enable_custom_linker_script();
    emit_build_info();
}

fn compile_asm() {
    let include_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("asm");

    let asm_path = include_path.join("bios");

    for file in include_path
        .read_dir()
        .expect("failed to open include directory")
        .chain(asm_path.read_dir().expect("failed to open asm directory"))
    {
        let file = file.expect("failed to read asm file");
        if file.file_type().expect("couldn't get file type").is_file() {
            let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
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
                .status()
                .expect("failed to acquire nasm output status")
                .success());

            println!("cargo:rustc-link-arg={}", output_path.display());
        }
    }
}

fn enable_custom_linker_script() {
    let linker_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("linker.ld");
    println!("cargo:rustc-link-arg=-T{}", linker_file.display());
}

fn emit_build_info() {
    let build_info_path =
        std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("build_info.rs");
    let mut build_info_file = std::fs::File::options()
        .write(true)
        .create(true)
        .append(false)
        .truncate(true)
        .open(build_info_path)
        .expect("Failed to open the build-time information file");

    build_info_file.write_all(
        b"// BELOW: THESEUS-SPECIFIC BUILD INFORMATION THAT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.\n"
    ).expect("Failed to write to the build-time information file.");

    let mut num_custom_cfgs = 0usize;
    let mut custom_cfgs = String::new();
    let mut custom_cfgs_str = String::new();

    for (k, v) in std::env::vars() {
        if k.starts_with("CARGO_CFG_") && !NON_CUSTOM_CFGS.contains(&k.as_str()) {
            let key = k[CARGO_CFG_PREFIX.len()..].to_lowercase();
            custom_cfgs = format!("{}(\"{}\", \"{}\"), ", custom_cfgs, key, v);

            custom_cfgs_str.push_str(&key);
            if !v.is_empty() {
                custom_cfgs_str.push_str(&format!("=\"{}\"", v));
            }
            custom_cfgs_str.push(' ');

            num_custom_cfgs += 1;
        }
    }

    // Append all of the custom cfg values to the built.rs file as an array.
    write!(
        &mut build_info_file,
        "#[allow(dead_code)]\npub const CUSTOM_CFG: [(&str, &str); {}] = [{}];\n",
        num_custom_cfgs, custom_cfgs,
    )
    .unwrap();

    // Append all of the custom cfg values to the built.rs file as a single string.
    write!(
        &mut build_info_file,
        "#[allow(dead_code)]\npub const CUSTOM_CFG_STR: &str = r#\"{}\"#;\n",
        custom_cfgs_str,
    )
    .unwrap();
}
