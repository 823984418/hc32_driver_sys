use lang_c::hack_bindgen::{HackBindgenCallbacks, HackBindgenContext, MacroItem, RustExpression};
use std::collections::HashSet;
use std::path::PathBuf;

fn hc32f4xx_bindgen_base() -> anyhow::Result<bindgen::Builder> {
    let mut bindgen = bindgen::builder();
    bindgen = bindgen.use_core();
    bindgen = bindgen.layout_tests(false);
    bindgen = bindgen.derive_partialeq(true);
    bindgen = bindgen.merge_extern_blocks(true);
    bindgen = bindgen.prepend_enum_name(false);
    bindgen = bindgen.raw_line("#![no_std]");
    bindgen = bindgen.allowlist_file(".*hc32[^\\\\/]*");

    bindgen = bindgen.clang_arg("--target=thumbv7em-none-eabihf");
    bindgen = bindgen.clang_arg("-mcpu=cortex-m4");

    bindgen = bindgen.clang_arg("-DUSE_DDL_DRIVER");

    bindgen = bindgen.clang_arg("-Idrivers");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Include");
    bindgen = bindgen.clang_arg("-Idrivers/hc32_ll_driver/inc");
    bindgen = bindgen.clang_arg("-Idrivers/cmsis/Device/HDSC/hc32f4xx/Include");
    bindgen = bindgen.wrap_static_fns(true);
    bindgen = bindgen.wrap_static_fns_path("drivers/bindgen");

    bindgen = bindgen.header("drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h");
    for i in std::fs::read_dir("drivers/hc32_ll_driver/inc")? {
        bindgen = bindgen.header(i?.path().display().to_string());
    }

    Ok(bindgen)
}

fn hc32f4xx_cc_base() -> anyhow::Result<cc::Build> {
    let mut cc = cc::Build::new();
    cc.host(env!("HOST"));
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.opt_level(3);
    cc.target("thumbv7em-none-eabihf");

    cc.flag("--target=thumbv7m-none-eabihf");
    cc.flag("-mcpu=cortex-m4");

    cc.define("USE_DDL_DRIVER", None);

    cc.include(".");
    cc.include("drivers");
    cc.include("drivers/cmsis/Include");
    cc.include("drivers/hc32_ll_driver/inc");
    cc.include("drivers/cmsis/Device/HDSC/hc32f4xx/Include");

    cc.file("drivers/bindgen.c");
    for i in std::fs::read_dir("drivers/hc32_ll_driver/src")? {
        cc.file(i?.path());
    }

    cc.out_dir("build");
    Ok(cc)
}

fn hc32f448() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f448"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut bindgen = hc32f4xx_bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F448");
    let mut ctx = HackBindgenContext::default();
    let pre_define = {
        [
            ("AOS_DCU1", "DCU_TRGSEL1"),
            ("AOS_DCU2", "DCU_TRGSEL2"),
            ("AOS_DCU3", "DCU_TRGSEL3"),
            ("AOS_DCU4", "DCU_TRGSEL4"),
            ("AOS_DMA1_0", "DMA1_TRGSEL0"),
            ("AOS_DMA1_1", "DMA1_TRGSEL1"),
            ("AOS_DMA1_2", "DMA1_TRGSEL2"),
            ("AOS_DMA1_3", "DMA1_TRGSEL3"),
            ("AOS_DMA1_4", "DMA1_TRGSEL4"),
            ("AOS_DMA1_5", "DMA1_TRGSEL5"),
            ("AOS_DMA2_0", "DMA2_TRGSEL0"),
            ("AOS_DMA2_1", "DMA2_TRGSEL1"),
            ("AOS_DMA2_2", "DMA2_TRGSEL2"),
            ("AOS_DMA2_3", "DMA2_TRGSEL3"),
            ("AOS_DMA2_4", "DMA2_TRGSEL4"),
            ("AOS_DMA2_5", "DMA2_TRGSEL5"),
            ("AOS_DMA_RC", "DMA_RC_TRGSEL"),
            ("AOS_TMR6_0", "TMR6_TRGSEL0"),
            ("AOS_TMR6_1", "TMR6_TRGSEL1"),
            ("AOS_TMR4_0", "TMR4_TRGSEL0"),
            ("AOS_TMR4_1", "TMR4_TRGSEL1"),
            ("AOS_TMR4_2", "TMR4_TRGSEL2"),
            ("AOS_EVTPORT12", "PEVNT_TRGSEL12"),
            ("AOS_EVTPORT34", "PEVNT_TRGSEL34"),
            ("AOS_TMR0", "TMR0_TRGSEL"),
            ("AOS_TMRA_0", "TMRA_TRGSEL0"),
            ("AOS_TMRA_1", "TMRA_TRGSEL1"),
            ("AOS_TMRA_2", "TMRA_TRGSEL2"),
            ("AOS_TMRA_3", "TMRA_TRGSEL3"),
            ("AOS_ADC1_0", "ADC1_TRGSEL0"),
            ("AOS_ADC1_1", "ADC1_TRGSEL1"),
            ("AOS_ADC2_0", "ADC2_TRGSEL0"),
            ("AOS_ADC2_1", "ADC2_TRGSEL1"),
            ("AOS_ADC3_0", "ADC3_TRGSEL0"),
            ("AOS_ADC3_1", "ADC3_TRGSEL1"),
            ("AOS_COMM_1", "COMTRGSEL1"),
            ("AOS_COMM_2", "COMTRGSEL2"),
        ]
    };
    for (name, field) in pre_define {
        ctx.define_macro(
            name,
            MacroItem::Expression(RustExpression::from_str(
                &format!(
                    "CM_AOS_BASE + core::mem::offset_of!(CM_AOS_TypeDef, {field}) as uint32_t"
                ),
                "uint32_t",
            )),
        );
    }
    let callback = HackBindgenCallbacks::new(ctx);
    std::fs::write("src/lib.rs", callback.generate(bindgen).unwrap())?;

    let mut cc = hc32f4xx_cc_base()?;
    cc.define("HC32F448", None);
    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f448.c");
    cc.compile("hc32_driver");

    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    Ok(())
}

fn hc32f460() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f460"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut bindgen = hc32f4xx_bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F460");
    let mut ctx = HackBindgenContext::default();
    let pre_define = {
        [
            ("AOS_DCU1", "DCU_TRGSEL1"),
            ("AOS_DCU2", "DCU_TRGSEL2"),
            ("AOS_DCU3", "DCU_TRGSEL3"),
            ("AOS_DCU4", "DCU_TRGSEL4"),
            ("AOS_DMA1_0", "DMA1_TRGSEL0"),
            ("AOS_DMA1_1", "DMA1_TRGSEL1"),
            ("AOS_DMA1_2", "DMA1_TRGSEL2"),
            ("AOS_DMA1_3", "DMA1_TRGSEL3"),
            ("AOS_DMA2_0", "DMA2_TRGSEL0"),
            ("AOS_DMA2_1", "DMA2_TRGSEL1"),
            ("AOS_DMA2_2", "DMA2_TRGSEL2"),
            ("AOS_DMA2_3", "DMA2_TRGSEL3"),
            ("AOS_DMA_RC", "DMA_RC_TRGSEL"),
            ("AOS_TMR6_0", "TMR6_TRGSEL0"),
            ("AOS_TMR6_1", "TMR6_TRGSEL1"),
            ("AOS_TMR0", "TMR0_TRGSEL"),
            ("AOS_EVTPORT12", "PEVNT_TRGSEL12"),
            ("AOS_EVTPORT34", "PEVNT_TRGSEL34"),
            ("AOS_TMRA_0", "TMRA_TRGSEL0"),
            ("AOS_TMRA_1", "TMRA_TRGSEL1"),
            ("AOS_OTS", "OTS_TRGSEL"),
            ("AOS_ADC1_0", "ADC1_TRGSEL0"),
            ("AOS_ADC1_1", "ADC1_TRGSEL1"),
            ("AOS_ADC2_0", "ADC2_TRGSEL0"),
            ("AOS_ADC2_1", "ADC2_TRGSEL1"),
            ("AOS_COMM_1", "COMTRG1"),
            ("AOS_COMM_2", "COMTRG2"),
        ]
    };
    for (name, field) in pre_define {
        ctx.define_macro(
            name,
            MacroItem::Expression(RustExpression::from_str(
                &format!(
                    "CM_AOS_BASE + core::mem::offset_of!(CM_AOS_TypeDef, {field}) as uint32_t"
                ),
                "uint32_t",
            )),
        );
    }
    let callback = HackBindgenCallbacks::new(ctx);
    std::fs::write("src/lib.rs", callback.generate(bindgen).unwrap())?;

    let mut cc = hc32f4xx_cc_base()?;
    cc.define("HC32F460", None);
    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f460.c");
    cc.compile("hc32_driver");

    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    Ok(())
}

fn hc32f4a0() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f4a0"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_dir_all("src");
    std::fs::create_dir_all("src")?;

    let mut bindgen = hc32f4xx_bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F4A0");
    let mut ctx = HackBindgenContext::default();
    let pre_define = {
        [
            ("AOS_DCU1", "DCU_TRGSEL1"),
            ("AOS_DCU2", "DCU_TRGSEL2"),
            ("AOS_DCU3", "DCU_TRGSEL3"),
            ("AOS_DCU4", "DCU_TRGSEL4"),
            ("AOS_DMA1_0", "DMA1_TRGSEL0"),
            ("AOS_DMA1_1", "DMA1_TRGSEL1"),
            ("AOS_DMA1_2", "DMA1_TRGSEL2"),
            ("AOS_DMA1_3", "DMA1_TRGSEL3"),
            ("AOS_DMA1_4", "DMA1_TRGSEL4"),
            ("AOS_DMA1_5", "DMA1_TRGSEL5"),
            ("AOS_DMA1_6", "DMA1_TRGSEL6"),
            ("AOS_DMA1_7", "DMA1_TRGSEL7"),
            ("AOS_DMA2_0", "DMA2_TRGSEL0"),
            ("AOS_DMA2_1", "DMA2_TRGSEL1"),
            ("AOS_DMA2_2", "DMA2_TRGSEL2"),
            ("AOS_DMA2_3", "DMA2_TRGSEL3"),
            ("AOS_DMA2_4", "DMA2_TRGSEL4"),
            ("AOS_DMA2_5", "DMA2_TRGSEL5"),
            ("AOS_DMA2_6", "DMA2_TRGSEL6"),
            ("AOS_DMA2_7", "DMA2_TRGSEL7"),
            ("AOS_DMA_RC", "DMA_RC_TRGSEL"),
            ("AOS_TMR6_0", "TMR6_TRGSEL0"),
            ("AOS_TMR6_1", "TMR6_TRGSEL1"),
            ("AOS_TMR6_2", "TMR6_TRGSEL2"),
            ("AOS_TMR6_3", "TMR6_TRGSEL3"),
            ("AOS_EVTPORT12", "PEVNT_TRGSEL12"),
            ("AOS_EVTPORT34", "PEVNT_TRGSEL34"),
            ("AOS_TMR0", "TMR0_TRGSEL"),
            ("AOS_TMR2", "TMR2_TRGSEL"),
            ("AOS_HASH_A", "HASH_TRGSELA"),
            ("AOS_HASH_B", "HASH_TRGSELB"),
            ("AOS_TMRA_0", "TMRA_TRGSEL0"),
            ("AOS_TMRA_1", "TMRA_TRGSEL1"),
            ("AOS_TMRA_2", "TMRA_TRGSEL2"),
            ("AOS_TMRA_3", "TMRA_TRGSEL3"),
            ("AOS_OTS", "OTS_TRGSEL"),
            ("AOS_ADC1_0", "ADC1_TRGSEL0"),
            ("AOS_ADC1_1", "ADC1_TRGSEL1"),
            ("AOS_ADC2_0", "ADC2_TRGSEL0"),
            ("AOS_ADC2_1", "ADC2_TRGSEL1"),
            ("AOS_ADC3_0", "ADC3_TRGSEL0"),
            ("AOS_ADC3_1", "ADC3_TRGSEL1"),
            ("AOS_COMM_1", "COMTRG1"),
            ("AOS_COMM_2", "COMTRG2"),
        ]
    };
    for (name, field) in pre_define {
        ctx.define_macro(
            name,
            MacroItem::Expression(RustExpression::from_str(
                &format!(
                    "CM_AOS_BASE + core::mem::offset_of!(CM_AOS_TypeDef, {field}) as uint32_t"
                ),
                "uint32_t",
            )),
        );
    }
    let callback = HackBindgenCallbacks::new(ctx);
    std::fs::write("src/lib.rs", callback.generate(bindgen).unwrap())?;

    let mut cc = hc32f4xx_cc_base()?;
    cc.define("HC32F4A0", None);
    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f4a0.c");
    cc.compile("hc32_driver");

    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mcu = std::env::args().skip(1).collect::<HashSet<_>>();
    if mcu.is_empty() {
        hc32f448()?;
        hc32f460()?;
        hc32f4a0()?;
    } else {
        for i in &mcu {
            match i.as_str() {
                "hc32f448" => hc32f448()?,
                "hc32f460" => hc32f460()?,
                "hc32f4a0" => hc32f4a0()?,
                _ => println!("Unknown MCU {}", i),
            }
        }
    }
    Ok(())
}
