use lang_c::hack_bindgen::HackBindgenCallbacks;
use std::collections::HashSet;
use std::path::PathBuf;

fn cc_base() -> anyhow::Result<cc::Build> {
    let mut cc = cc::Build::new();
    cc.host(env!("HOST"));
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.opt_level(3);
    cc.target("thumbv7em-none-eabihf");

    cc.flag("--target=thumbv7m-none-eabihf");
    cc.flag("-mcpu=cortex-m4");

    cc.define("USE_DDL_DRIVER", None);

    cc.include("drivers");
    cc.include("drivers/cmsis/Include");
    cc.include("drivers/hc32_ll_driver/inc");
    cc.include("drivers/cmsis/Device/HDSC/hc32f4xx/Include");

    for i in std::fs::read_dir("drivers/hc32_ll_driver/src")? {
        cc.file(i?.path());
    }

    cc.out_dir("build");
    Ok(cc)
}

fn bindgen_base() -> anyhow::Result<bindgen::Builder> {
    let mut bindgen = bindgen::builder();
    bindgen = bindgen.use_core();
    bindgen = bindgen.layout_tests(false);
    bindgen = bindgen.derive_partialeq(true);
    bindgen = bindgen.merge_extern_blocks(true);
    bindgen = bindgen.raw_line("#![no_std]");
    bindgen = bindgen.allowlist_file(".*hc32[^\\\\/]*");

    bindgen = bindgen.clang_arg("--target=thumbv7em-none-eabihf");
    bindgen = bindgen.clang_arg("-mcpu=cortex-m4");

    bindgen = bindgen.clang_arg("-DUSE_DDL_DRIVER");

    bindgen = bindgen.clang_arg("-Idrivers");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Include");
    bindgen = bindgen.clang_arg("-Idrivers/hc32_ll_driver/inc");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Device/HDSC/hc32f4xx/Include");

    bindgen = bindgen.header("drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h");
    for i in std::fs::read_dir("drivers/hc32_ll_driver/inc")? {
        bindgen = bindgen.header(i?.path().display().to_string());
    }

    Ok(bindgen)
}

fn hc32f448() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f448"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut cc = cc_base()?;
    cc.define("HC32F448", None);
    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f448.c");
    cc.compile("hc32_driver");

    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    let mut bindgen = bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F448");
    bindgen = bindgen.blocklist_item("AOS_DCU[0-9]");
    bindgen = bindgen.blocklist_item("AOS_DMA[0-9]_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_DMA_RC");
    bindgen = bindgen.blocklist_item("AOS_TMR[0-9A-Z]_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_EVTPORT12");
    bindgen = bindgen.blocklist_item("AOS_EVTPORT34");
    bindgen = bindgen.blocklist_item("AOS_TMR0");
    bindgen = bindgen.blocklist_item("AOS_ADC[0-9]_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_COMM_[0-9]");
    std::fs::write(
        "src/lib.rs",
        HackBindgenCallbacks::generate(bindgen).unwrap(),
    )?;

    Ok(())
}

fn hc32f460() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f460"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut cc = cc_base()?;
    cc.define("HC32F460", None);
    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f460.c");
    cc.compile("hc32_driver");

    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    let mut bindgen = bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F460");
    bindgen = bindgen.allowlist_file(".*usb[^\\\\/]*");
    bindgen = bindgen.blocklist_item("AOS_DCU[0-9]");
    bindgen = bindgen.blocklist_item("AOS_DMA[0-9]_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_DMA_RC");
    bindgen = bindgen.blocklist_item("AOS_TMR[0-9A-Z]_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_EVTPORT12");
    bindgen = bindgen.blocklist_item("AOS_EVTPORT34");
    bindgen = bindgen.blocklist_item("AOS_TMR0");
    bindgen = bindgen.blocklist_item("AOS_ADC[0-9]_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_COMM_[0-9]");
    bindgen = bindgen.blocklist_item("AOS_OTS");
    std::fs::write(
        "src/lib.rs",
        HackBindgenCallbacks::generate(bindgen).unwrap(),
    )?;

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
                "hc32f460" => hc32f460()?,
                _ => println!("Unknown MCU {}", i),
            }
        }
    }
    Ok(())
}
