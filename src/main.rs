use anyhow::anyhow;
use lang_c::hack_bindgen::{HackBindgenCallbacks, HackBindgenContext, MacroItem, RustExpression};
use std::collections::HashSet;
use std::path::PathBuf;

fn hc32f4xx_interrupt(name: &str) -> anyhow::Result<()> {
    let startup_s = std::fs::read_to_string(format!(
        "drivers/cmsis/Device/HDSC/hc32f4xx/Source/GCC/startup_{}.S",
        name
    ))?;
    let start = startup_s.find("Interrupts").ok_or(anyhow!("error"))?;
    let end = start
        + startup_s[start..]
            .find("__Vectors_End")
            .ok_or(anyhow!("error"))?;
    let reg = regex::Regex::new(r##"\.long\s+(\w+)"##)?;
    let interrupts = reg
        .captures_iter(&startup_s[start..end])
        .map(|cap| cap[1].to_string())
        .collect::<Vec<_>>();

    fn is_interrupt_name(x: &str) -> bool {
        x.len() > 1
    }

    let device_x = interrupts
        .iter()
        .filter(|i| is_interrupt_name(i))
        .map(|i| format!("PROVIDE({} = DefaultHandler);", i))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    std::fs::write("device.x", device_x)?;

    let mut rt_mod = String::new();
    rt_mod += r#"unsafe extern "C" {"#;
    rt_mod += r#"fn DefaultHandler();"#;
    rt_mod += &interrupts
        .iter()
        .filter(|i| is_interrupt_name(i))
        .map(|i| format!("fn {}();", i))
        .collect::<Vec<_>>()
        .join("\n");
    rt_mod += r#"}"#;
    rt_mod += r#"#[cfg(feature = "rt")]"#;
    rt_mod += r#"#[doc(hidden)]"#;
    rt_mod += r#"#[unsafe(link_section = ".vector_table.interrupts")]"#;
    rt_mod += r#"#[unsafe(no_mangle)]"#;
    rt_mod += &format!(
        "pub static __INTERRUPTS: [unsafe extern \"C\" fn(); {}] = [",
        interrupts.len()
    );
    rt_mod += &interrupts
        .iter()
        .map(|i| {
            if is_interrupt_name(i) {
                i.to_string()
            } else {
                "DefaultHandler".to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(",\n");
    rt_mod += r#"];"#;
    rt_mod += r#"#[doc = r"Enumeration of all the interrupts."]"#;
    rt_mod += r#"#[derive(Copy, Clone, Debug, PartialEq, Eq)]"#;
    rt_mod += r#"#[repr(u16)]"#;
    rt_mod += r#"pub enum Interrupt {"#;
    rt_mod += &interrupts
        .iter()
        .enumerate()
        .filter(|(_, i)| is_interrupt_name(i))
        .map(|(v, i)| format!("{} = {},", i, v))
        .collect::<Vec<_>>()
        .join("");
    rt_mod += r#"}"#;
    rt_mod += r#"unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {"#;
    rt_mod += r#"#[inline(always)]"#;
    rt_mod += r#"fn number(self) -> u16 {"#;
    rt_mod += r#"self as u16"#;
    rt_mod += r#"}"#;
    rt_mod += r#"}"#;

    std::fs::write("src/rt.rs", rt_mod)?;

    std::process::Command::new("cargo")
        .args(["fmt", "--", "src/rt.rs"])
        .status()?;

    Ok(())
}

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
    _ = std::fs::remove_file("src/lib.rs");
    _ = std::fs::remove_file("src/rt.rs");

    let mut bindgen = hc32f4xx_bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F448");
    bindgen = bindgen.raw_line("mod patch;");
    bindgen = bindgen.raw_line("pub use patch::*;");
    bindgen = bindgen.raw_line("#[cfg(feature = \"rt\")]");
    bindgen = bindgen.raw_line("mod rt;");
    bindgen = bindgen.raw_line("#[cfg(feature = \"rt\")]");
    bindgen = bindgen.raw_line("pub use rt::*;");
    // bindgen = bindgen.blocklist_item("HCLK_VALUE");
    // bindgen = bindgen.blocklist_item("I2C_SRC_CLK");
    let mut ctx = HackBindgenContext::default();
    ctx.env.hack_is_typename = Some(true);
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

    hc32f4xx_interrupt("hc32f448")?;
    Ok(())
}

fn hc32f460() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f460"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_file("src/lib.rs");
    _ = std::fs::remove_file("src/rt.rs");

    let mut bindgen = hc32f4xx_bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F460");
    bindgen = bindgen.raw_line("mod patch;");
    bindgen = bindgen.raw_line("pub use patch::*;");
    bindgen = bindgen.raw_line("#[cfg(feature = \"rt\")]");
    bindgen = bindgen.raw_line("mod rt;");
    bindgen = bindgen.raw_line("#[cfg(feature = \"rt\")]");
    bindgen = bindgen.raw_line("pub use rt::*;");
    // bindgen = bindgen.blocklist_item("HCLK_VALUE");
    // bindgen = bindgen.blocklist_item("I2C_SRC_CLK");
    let mut ctx = HackBindgenContext::default();
    ctx.env.hack_is_typename = Some(true);
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

    hc32f4xx_interrupt("hc32f460")?;
    Ok(())
}

fn hc32f4a0() -> anyhow::Result<()> {
    std::env::set_current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hc32f4a0"))?;

    _ = std::fs::remove_dir_all("build");
    _ = std::fs::remove_file("src/lib.rs");
    _ = std::fs::remove_file("src/rt.rs");

    let mut bindgen = hc32f4xx_bindgen_base()?;
    bindgen = bindgen.clang_arg("-DHC32F4A0");
    bindgen = bindgen.raw_line("mod patch;");
    bindgen = bindgen.raw_line("pub use patch::*;");
    bindgen = bindgen.raw_line("#[cfg(feature = \"rt\")]");
    bindgen = bindgen.raw_line("mod rt;");
    bindgen = bindgen.raw_line("#[cfg(feature = \"rt\")]");
    bindgen = bindgen.raw_line("pub use rt::*;");
    // bindgen = bindgen.blocklist_item("HCLK_VALUE");
    // bindgen = bindgen.blocklist_item("I2C_SRC_CLK");
    bindgen = bindgen.blocklist_item("MAU_SQRT_TIMEOUT");
    let mut ctx = HackBindgenContext::default();
    ctx.env.hack_is_typename = Some(true);
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
    let pre_define = {
        [
            ("HASH_TRIG_EVT_DMA1_TC0", "EVT_SRC_DMA1_TC0"),
            ("HASH_TRIG_EVT_DMA1_TC1", "EVT_SRC_DMA1_TC1"),
            ("HASH_TRIG_EVT_DMA1_TC2", "EVT_SRC_DMA1_TC2"),
            ("HASH_TRIG_EVT_DMA1_TC3", "EVT_SRC_DMA1_TC3"),
            ("HASH_TRIG_EVT_DMA1_TC4", "EVT_SRC_DMA1_TC4"),
            ("HASH_TRIG_EVT_DMA1_TC5", "EVT_SRC_DMA1_TC5"),
            ("HASH_TRIG_EVT_DMA1_TC6", "EVT_SRC_DMA1_TC6"),
            ("HASH_TRIG_EVT_DMA1_TC7", "EVT_SRC_DMA1_TC7"),
            ("HASH_TRIG_EVT_DMA1_BTC0", "EVT_SRC_DMA1_BTC0"),
            ("HASH_TRIG_EVT_DMA1_BTC1", "EVT_SRC_DMA1_BTC1"),
            ("HASH_TRIG_EVT_DMA1_BTC2", "EVT_SRC_DMA1_BTC2"),
            ("HASH_TRIG_EVT_DMA1_BTC3", "EVT_SRC_DMA1_BTC3"),
            ("HASH_TRIG_EVT_DMA1_BTC4", "EVT_SRC_DMA1_BTC4"),
            ("HASH_TRIG_EVT_DMA1_BTC5", "EVT_SRC_DMA1_BTC5"),
            ("HASH_TRIG_EVT_DMA1_BTC6", "EVT_SRC_DMA1_BTC6"),
            ("HASH_TRIG_EVT_DMA1_BTC7", "EVT_SRC_DMA1_BTC7"),
            ("HASH_TRIG_EVT_DMA2_TC0", "EVT_SRC_DMA2_TC0"),
            ("HASH_TRIG_EVT_DMA2_TC1", "EVT_SRC_DMA2_TC1"),
            ("HASH_TRIG_EVT_DMA2_TC2", "EVT_SRC_DMA2_TC2"),
            ("HASH_TRIG_EVT_DMA2_TC3", "EVT_SRC_DMA2_TC3"),
            ("HASH_TRIG_EVT_DMA2_TC4", "EVT_SRC_DMA2_TC4"),
            ("HASH_TRIG_EVT_DMA2_TC5", "EVT_SRC_DMA2_TC5"),
            ("HASH_TRIG_EVT_DMA2_TC6", "EVT_SRC_DMA2_TC6"),
            ("HASH_TRIG_EVT_DMA2_TC7", "EVT_SRC_DMA2_TC7"),
            ("HASH_TRIG_EVT_DMA2_BTC0", "EVT_SRC_DMA2_BTC0"),
            ("HASH_TRIG_EVT_DMA2_BTC1", "EVT_SRC_DMA2_BTC1"),
            ("HASH_TRIG_EVT_DMA2_BTC2", "EVT_SRC_DMA2_BTC2"),
            ("HASH_TRIG_EVT_DMA2_BTC3", "EVT_SRC_DMA2_BTC3"),
            ("HASH_TRIG_EVT_DMA2_BTC4", "EVT_SRC_DMA2_BTC4"),
            ("HASH_TRIG_EVT_DMA2_BTC5", "EVT_SRC_DMA2_BTC5"),
            ("HASH_TRIG_EVT_DMA2_BTC6", "EVT_SRC_DMA2_BTC6"),
            ("HASH_TRIG_EVT_DMA2_BTC7", "EVT_SRC_DMA2_BTC7"),
        ]
    };
    for (name, field) in pre_define {
        ctx.define_macro(
            name,
            MacroItem::Expression(RustExpression::from_str(field, "en_event_src_t")),
        );
    }
    let callback = HackBindgenCallbacks::new(ctx);
    std::fs::write("src/lib.rs", callback.generate(bindgen).unwrap())?;

    let mut cc = hc32f4xx_cc_base()?;
    cc.define("HC32F4A0", None);
    cc.file("drivers/cmsis/Device/HDSC/hc32f4xx/Source/system_hc32f4a0.c");
    cc.compile("hc32_driver");
    std::fs::copy("build/libhc32_driver.a", "libhc32_driver.a")?;

    hc32f4xx_interrupt("hc32f4a0")?;
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
