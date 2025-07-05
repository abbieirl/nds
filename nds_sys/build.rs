use anyhow::Result;
use bindgen::{Builder, RustTarget};
use std::env::var;
use std::path::PathBuf;

fn main() -> Result<()> {
    let wonderful = var("WONDERFUL")?;
    let blocksds = var("BLOCKSDS")?;
    let profile = var("PROFILE")?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native={blocksds}/libs/libnds/lib");
    println!(
        "cargo:rustc-link-search=native={wonderful}/toolchain/gcc-arm-none-eabi/arm-none-eabi/lib"
    );
    println!("cargo:rustc-link-lib=c");

    match profile.as_str() {
        "dev" => println!("cargo:rustc-link-lib=static=nds9d"),
        _ => println!("cargo:rustc-link-lib=static=nds9"),
    };

    let bindings = Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rust_target(RustTarget::nightly())
        .merge_extern_blocks(true)
        .sort_semantically(true)
        .derive_default(true)
        .rustified_enum("ConsoleColor")
        .bitfield_enum("KEYPAD_BITS")
        .clang_arg(format!(
            "-isystem{wonderful}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include"
        ))
        .clang_arg(format!("-I{blocksds}/libs/libnds/include"))
        .clang_args(["-DARM9"])
        .use_core()
        .generate()?;

    let out = PathBuf::from(var("OUT_DIR")?);
    bindings.write_to_file(out.join("bindings.rs"))?;

    Ok(())
}
