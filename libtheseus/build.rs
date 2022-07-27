#![feature(exit_status_error)]

use std::{collections::HashMap, path::PathBuf, process::Command};

enum Item {
    Function {
        signature: String,
        link_name: String,
    },
    Module {
        name: String,
        items: Vec<Item>,
    },
}

impl Item {
    fn into_rust_code(self, symbols: &HashMap<String, String>) -> String {
        match self {
            Self::Function {
                signature,
                link_name,
            } => {
                let hash = symbols.get(&link_name).expect("link name not found");
                let link_name = format!("{link_name}::{hash}");
                format!(
                    r#"

#[export_name = "{link_name}"]
#[no_mangle]
#[inline(never)]
pub fn {signature} {{}} 
                "#
                )
            }
            Self::Module { name, items } => {
                let items: String = items
                    .into_iter()
                    .map(|item| item.into_rust_code(symbols))
                    .collect();
                format!(
                    "
pub mod {name} {{
    {items}
}}
                "
                )
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=../build/deps");

    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("..");

    // Command::new("make")
    //     .arg("build")
    //     .arg("-C")
    //     .arg(&path)
    //     .status()
    //     .expect("failed to execute make command")
    //     .exit_ok()
    //     .expect("make command failed");

    path.push("build");
    path.push("deps");
    
    let mut rlib_paths = Vec::new();

    for item in std::fs::read_dir(path).expect("failed to read deps directory") {
        let item = item.expect("failed to read item in deps directory");

        if PathBuf::from(item.file_name())
            .extension()
            .unwrap_or_default()
            == "rlib"
        {
            rlib_paths.push(item.path());
        }
    }

    let symbols: HashMap<_, _> = rlib_paths
        .into_iter()
        .map(|path| {
            let output = Command::new("nm")
                .arg("-g")
                .arg(path)
                .output()
                .expect("failed to execute nm command");
            output.status.exit_ok().expect("nm command failed");
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| {
                    let index = line.find("_Z")?;
                    let (_, symbol) = line.split_at(index);
                    let demangled = rustc_demangle::demangle(symbol).to_string();
                    let hash_start_index = demangled.rfind("::")?;
                    let (symbol, hash) = demangled.split_at(hash_start_index);

                    let symbol = String::from(symbol);
                    let hash = String::from(&hash[2..]);

                    Some((symbol, hash))
                })
                .filter_map(|x| x)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let root = vec![Item::Module {
        name: "stdio".to_string(),
        items: vec![Item::Function {
            signature: "print_to_stdout_args(fmt_args: core::fmt::Arguments)".to_string(),
            link_name: "terminal_print::print_to_stdout_args".to_string(),
        }],
    }];

    let code: String = root
        .into_iter()
        .map(|item| item.into_rust_code(&symbols))
        .collect();

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(out_path.join("stubs.rs"), code).expect("couldn't write to file");
}
