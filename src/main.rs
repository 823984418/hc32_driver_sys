use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;

fn codegen(name: &str) -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(name))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");

    let mut cc = cc::Build::new();
    cc.host(env!("HOST"));
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.opt_level(3);
    cc.target("thumbv7m-none-eabihf");
    cc.flag("-mcpu=cortex-m4");

    cc.define(&name.to_uppercase(), None);
    cc.define("USE_DDL_DRIVER", None);

    cc.include("drivers");
    cc.include("drivers/cmsis/Include");
    cc.include("drivers/cmsis/Device/HDSC/hc32f4xx/Include");
    cc.include("drivers/hc32_ll_driver/inc");

    for i in std::fs::read_dir("drivers/hc32_ll_driver/src")? {
        cc.file(i?.path());
    }

    cc.out_dir("build");
    cc.compile("hc32_driver");

    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    let mut bindgen = bindgen::builder();
    bindgen = bindgen.use_core();
    bindgen = bindgen.layout_tests(false);
    bindgen = bindgen.derive_partialeq(true);
    bindgen = bindgen.merge_extern_blocks(true);
    // bindgen = bindgen.raw_line("#![no_std]");
    bindgen = bindgen.clang_arg("--target=thumbv7m-none-eabihf");
    bindgen = bindgen.clang_arg("-mcpu=cortex-m4");
    bindgen = bindgen.clang_arg(format!("-D{}", name.to_uppercase()));
    bindgen = bindgen.clang_arg("-Idrivers");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Include");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Device/HDSC/hc32f4xx/Include");
    bindgen = bindgen.clang_arg("-Idrivers/hc32_ll_driver/inc");

    std::fs::create_dir_all("src")?;

    let mut lib_rs = std::fs::File::create("src/lib.rs")?;
    writeln!(lib_rs, "#![no_std]")?;
    writeln!(lib_rs)?;

    {
        writeln!(lib_rs, "pub mod hc32f4xx;")?;
        let mut bindgen = bindgen.clone();
        bindgen = bindgen.header("drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h");
        bindgen = bindgen.allowlist_file(".*hc32f4xx.*");
        bindgen
            .generate()?
            .write_to_file(PathBuf::from("src").join("hc32f4xx.rs"))?;
    }

    for i in std::fs::read_dir("drivers/hc32_ll_driver/inc")? {
        let i = i?;
        let mut bindgen = bindgen.clone();
        bindgen = bindgen.header(i.path().display().to_string());
        let module = i.path().file_stem().unwrap().display().to_string();
        writeln!(lib_rs, "pub mod {};", module)?;
        bindgen = bindgen.allowlist_file(format!(".*{}.*", module));
        bindgen
            .generate()?
            .write_to_file(PathBuf::from("src").join(module).with_extension("rs"))?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mcu = std::env::args().skip(1).collect::<HashSet<_>>();
    if mcu.is_empty() {
        codegen("hc32f460")?;
    } else {
        for i in &mcu {
            codegen(i)?;
        }
    }
    Ok(())
}
