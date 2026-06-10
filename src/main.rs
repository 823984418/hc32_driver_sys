use std::collections::HashSet;
use std::path::PathBuf;

fn hc32f448() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f448"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut cc = cc::Build::new();
    cc.host(env!("HOST"));
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.opt_level(3);
    cc.target("thumbv7m-none-eabihf");
    cc.flag("-mcpu=cortex-m4");

    cc.define("HC32F448", None);
    cc.define("USE_DDL_DRIVER", None);

    cc.include("drivers");
    cc.include("drivers/cmsis/Include");
    cc.include("drivers/cmsis/Device/HDSC/hc32f4xx/Include");
    cc.include("drivers/hc32_ll_driver/inc");

    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f448.c");
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
    bindgen = bindgen.clang_arg("--target=thumbv7m-none-eabihf");
    bindgen = bindgen.clang_arg("-mcpu=cortex-m4");
    bindgen = bindgen.clang_arg("-DHC32F448");
    bindgen = bindgen.clang_arg("-Idrivers");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Include");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Device/HDSC/hc32f4xx/Include");
    bindgen = bindgen.clang_arg("-Idrivers/hc32_ll_driver/inc");
    bindgen = bindgen.raw_line("#![no_std]");
    bindgen = bindgen.sort_semantically(true);

    bindgen = bindgen.header("drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h");
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("hc32f4xx.h")));
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("hc32f448.h")));
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("system_hcf448.h")));
    for i in std::fs::read_dir("drivers/hc32_ll_driver/inc")? {
        let i = i?;
        bindgen = bindgen.header(i.path().display().to_string());
        let module = i.path().file_name().unwrap().display().to_string();
        bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape(&module)));
    }

    bindgen.generate()?.write_to_file("src/lib.rs")?;

    Ok(())
}

fn hc32f460() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f460"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut cc = cc::Build::new();
    cc.host(env!("HOST"));
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.opt_level(3);
    cc.target("thumbv7m-none-eabihf");
    cc.flag("-mcpu=cortex-m4");

    cc.define("HC32F460", None);
    cc.define("USE_DDL_DRIVER", None);

    cc.include("drivers");
    cc.include("drivers/cmsis/Include");
    cc.include("drivers/cmsis/Device/HDSC/hc32f4xx/Include");
    cc.include("drivers/hc32_ll_driver/inc");

    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f460.c");
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
    bindgen = bindgen.clang_arg("--target=thumbv7m-none-eabihf");
    bindgen = bindgen.clang_arg("-mcpu=cortex-m4");
    bindgen = bindgen.clang_arg("-DHC32F460");
    bindgen = bindgen.clang_arg("-Idrivers");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Include");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Device/HDSC/hc32f4xx/Include");
    bindgen = bindgen.clang_arg("-Idrivers/hc32_ll_driver/inc");
    bindgen = bindgen.raw_line("#![no_std]");
    bindgen = bindgen.sort_semantically(true);

    bindgen = bindgen.header("drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h");
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("hc32f4xx.h")));
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("hc32f460.h")));
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("system_hcf460.h")));
    bindgen = bindgen.header("drivers/usb_lib.h");
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("usb_lib.h")));
    bindgen = bindgen.header("drivers/usb_bsp.h");
    bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape("usb_bsp.h")));
    for i in std::fs::read_dir("drivers/hc32_ll_driver/inc")? {
        let i = i?;
        bindgen = bindgen.header(i.path().display().to_string());
        let module = i.path().file_name().unwrap().display().to_string();
        bindgen = bindgen.allowlist_file(format!(".*{}", regex::escape(&module)));
    }
    bindgen.generate()?.write_to_file("src/lib.rs")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mcu = std::env::args().skip(1).collect::<HashSet<_>>();
    if mcu.is_empty() {
        hc32f448()?;
        hc32f460()?;
    } else {
        for i in &mcu {
            match i.as_str() {
                "hc32f448" => hc32f448()?,
                _ => println!("Unknown MCU {}", i),
            }
        }
    }
    Ok(())
}
