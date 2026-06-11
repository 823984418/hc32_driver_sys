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
    bindgen = bindgen.raw_line(
        r###"
pub const CM_ADC1: *mut CM_ADC_TypeDef = CM_ADC1_BASE as _;
pub const CM_ADC2: *mut CM_ADC_TypeDef = CM_ADC2_BASE as _;
pub const CM_ADC3: *mut CM_ADC_TypeDef = CM_ADC3_BASE as _;
pub const CM_AES: *mut CM_AES_TypeDef = CM_AES_BASE as _;
pub const CM_AOS: *mut CM_AOS_TypeDef = CM_AOS_BASE as _;
pub const CM_CMP1: *mut CM_CMP_TypeDef = CM_CMP1_BASE as _;
pub const CM_CMP2: *mut CM_CMP_TypeDef = CM_CMP2_BASE as _;
pub const CM_CMP3: *mut CM_CMP_TypeDef = CM_CMP3_BASE as _;
pub const CM_CMP4: *mut CM_CMP_TypeDef = CM_CMP4_BASE as _;
pub const CM_CMU: *mut CM_CMU_TypeDef = CM_CMU_BASE as _;
pub const CM_CRC: *mut CM_CRC_TypeDef = CM_CRC_BASE as _;
pub const CM_CTC: *mut CM_CTC_TypeDef = CM_CTC_BASE as _;
pub const CM_DAC: *mut CM_DAC_TypeDef = CM_DAC_BASE as _;
pub const CM_DBGC: *mut CM_DBGC_TypeDef = CM_DBGC_BASE as _;
pub const CM_DCU1: *mut CM_DCU_TypeDef = CM_DCU1_BASE as _;
pub const CM_DCU2: *mut CM_DCU_TypeDef = CM_DCU2_BASE as _;
pub const CM_DCU3: *mut CM_DCU_TypeDef = CM_DCU3_BASE as _;
pub const CM_DCU4: *mut CM_DCU_TypeDef = CM_DCU4_BASE as _;
pub const CM_DMA1: *mut CM_DMA_TypeDef = CM_DMA1_BASE as _;
pub const CM_DMA2: *mut CM_DMA_TypeDef = CM_DMA2_BASE as _;
pub const CM_EFM: *mut CM_EFM_TypeDef = CM_EFM_BASE as _;
pub const CM_EMB0: *mut CM_EMB_TypeDef = CM_EMB0_BASE as _;
pub const CM_EMB1: *mut CM_EMB_TypeDef = CM_EMB1_BASE as _;
pub const CM_EMB2: *mut CM_EMB_TypeDef = CM_EMB2_BASE as _;
pub const CM_EMB3: *mut CM_EMB_TypeDef = CM_EMB3_BASE as _;
pub const CM_FCM: *mut CM_FCM_TypeDef = CM_FCM_BASE as _;
pub const CM_GPIO: *mut CM_GPIO_TypeDef = CM_GPIO_BASE as _;
pub const CM_HASH: *mut CM_HASH_TypeDef = CM_HASH_BASE as _;
pub const CM_I2C1: *mut CM_I2C_TypeDef = CM_I2C1_BASE as _;
pub const CM_I2C2: *mut CM_I2C_TypeDef = CM_I2C2_BASE as _;
pub const CM_ICG: *mut CM_ICG_TypeDef = CM_ICG_BASE as _;
pub const CM_INTC: *mut CM_INTC_TypeDef = CM_INTC_BASE as _;
pub const CM_KEYSCAN: *mut CM_KEYSCAN_TypeDef = CM_KEYSCAN_BASE as _;
pub const CM_MCAN1: *mut CM_MCAN_TypeDef = CM_MCAN1_BASE as _;
pub const CM_MCAN2: *mut CM_MCAN_TypeDef = CM_MCAN2_BASE as _;
pub const CM_MPU: *mut CM_MPU_TypeDef = CM_MPU_BASE as _;
pub const CM_PERIC: *mut CM_PERIC_TypeDef = CM_PERIC_BASE as _;
pub const CM_PWC: *mut CM_PWC_TypeDef = CM_PWC_BASE as _;
pub const CM_QSPI: *mut CM_QSPI_TypeDef = CM_QSPI_BASE as _;
pub const CM_RMU: *mut CM_RMU_TypeDef = CM_RMU_BASE as _;
pub const CM_RTC: *mut CM_RTC_TypeDef = CM_RTC_BASE as _;
pub const CM_SMC: *mut CM_SMC_TypeDef = CM_SMC_BASE as _;
pub const CM_SPI1: *mut CM_SPI_TypeDef = CM_SPI1_BASE as _;
pub const CM_SPI2: *mut CM_SPI_TypeDef = CM_SPI2_BASE as _;
pub const CM_SPI3: *mut CM_SPI_TypeDef = CM_SPI3_BASE as _;
pub const CM_SRAMC: *mut CM_SRAMC_TypeDef = CM_SRAMC_BASE as _;
pub const CM_SWDT: *mut CM_SWDT_TypeDef = CM_SWDT_BASE as _;
pub const CM_TMR0_1: *mut CM_TMR0_TypeDef = CM_TMR0_1_BASE as _;
pub const CM_TMR0_2: *mut CM_TMR0_TypeDef = CM_TMR0_2_BASE as _;
pub const CM_TMR4_1: *mut CM_TMR4_TypeDef = CM_TMR4_1_BASE as _;
pub const CM_TMR4_2: *mut CM_TMR4_TypeDef = CM_TMR4_2_BASE as _;
pub const CM_TMR4_3: *mut CM_TMR4_TypeDef = CM_TMR4_3_BASE as _;
pub const CM_TMR6_1: *mut CM_TMR6_TypeDef = CM_TMR6_1_BASE as _;
pub const CM_TMR6_2: *mut CM_TMR6_TypeDef = CM_TMR6_2_BASE as _;
pub const CM_TMR6_COMMON: *mut CM_TMR6_COMMON_TypeDef = CM_TMR6_COMMON_BASE as _;
pub const CM_TMRA_1: *mut CM_TMRA_TypeDef = CM_TMRA_1_BASE as _;
pub const CM_TMRA_2: *mut CM_TMRA_TypeDef = CM_TMRA_2_BASE as _;
pub const CM_TMRA_3: *mut CM_TMRA_TypeDef = CM_TMRA_3_BASE as _;
pub const CM_TMRA_4: *mut CM_TMRA_TypeDef = CM_TMRA_4_BASE as _;
pub const CM_TMRA_5: *mut CM_TMRA_TypeDef = CM_TMRA_5_BASE as _;
pub const CM_TRNG: *mut CM_TRNG_TypeDef = CM_TRNG_BASE as _;
pub const CM_USART1: *mut CM_USART_TypeDef = CM_USART1_BASE as _;
pub const CM_USART2: *mut CM_USART_TypeDef = CM_USART2_BASE as _;
pub const CM_USART3: *mut CM_USART_TypeDef = CM_USART3_BASE as _;
pub const CM_USART4: *mut CM_USART_TypeDef = CM_USART4_BASE as _;
pub const CM_USART5: *mut CM_USART_TypeDef = CM_USART5_BASE as _;
pub const CM_USART6: *mut CM_USART_TypeDef = CM_USART6_BASE as _;
pub const CM_WDT: *mut CM_WDT_TypeDef = CM_WDT_BASE as _;
"###,
    );
    bindgen = bindgen.clang_arg("-DHC32F448");
    bindgen.generate()?.write_to_file("src/lib.rs")?;

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
    bindgen = bindgen.raw_line(
        r###"
pub const CM_ADC1: *mut CM_ADC_TypeDef = CM_ADC1_BASE as _;
pub const CM_ADC2: *mut CM_ADC_TypeDef = CM_ADC2_BASE as _;
pub const CM_AES: *mut CM_AES_TypeDef = CM_AES_BASE as _;
pub const CM_AOS: *mut CM_AOS_TypeDef = CM_AOS_BASE as _;
pub const CM_CAN: *mut CM_CAN_TypeDef = CM_CAN_BASE as _;
pub const CM_CMP1: *mut CM_CMP_TypeDef = CM_CMP1_BASE as _;
pub const CM_CMP2: *mut CM_CMP_TypeDef = CM_CMP2_BASE as _;
pub const CM_CMP3: *mut CM_CMP_TypeDef = CM_CMP3_BASE as _;
pub const CM_CMP_COMMON: *mut CM_CMP_COMMON_TypeDef = CM_CMP_COMMON_BASE as _;
pub const CM_CMU: *mut CM_CMU_TypeDef = CM_CMU_BASE as _;
pub const CM_CRC: *mut CM_CRC_TypeDef = CM_CRC_BASE as _;
pub const CM_DBGC: *mut CM_DBGC_TypeDef = CM_DBGC_BASE as _;
pub const CM_DCU1: *mut CM_DCU_TypeDef = CM_DCU1_BASE as _;
pub const CM_DCU2: *mut CM_DCU_TypeDef = CM_DCU2_BASE as _;
pub const CM_DCU3: *mut CM_DCU_TypeDef = CM_DCU3_BASE as _;
pub const CM_DCU4: *mut CM_DCU_TypeDef = CM_DCU4_BASE as _;
pub const CM_DMA1: *mut CM_DMA_TypeDef = CM_DMA1_BASE as _;
pub const CM_DMA2: *mut CM_DMA_TypeDef = CM_DMA2_BASE as _;
pub const CM_EFM: *mut CM_EFM_TypeDef = CM_EFM_BASE as _;
pub const CM_EMB0: *mut CM_EMB_TypeDef = CM_EMB0_BASE as _;
pub const CM_EMB1: *mut CM_EMB_TypeDef = CM_EMB1_BASE as _;
pub const CM_EMB2: *mut CM_EMB_TypeDef = CM_EMB2_BASE as _;
pub const CM_EMB3: *mut CM_EMB_TypeDef = CM_EMB3_BASE as _;
pub const CM_FCM: *mut CM_FCM_TypeDef = CM_FCM_BASE as _;
pub const CM_GPIO: *mut CM_GPIO_TypeDef = CM_GPIO_BASE as _;
pub const CM_HASH: *mut CM_HASH_TypeDef = CM_HASH_BASE as _;
pub const CM_I2C1: *mut CM_I2C_TypeDef = CM_I2C1_BASE as _;
pub const CM_I2C2: *mut CM_I2C_TypeDef = CM_I2C2_BASE as _;
pub const CM_I2C3: *mut CM_I2C_TypeDef = CM_I2C3_BASE as _;
pub const CM_I2S1: *mut CM_I2S_TypeDef = CM_I2S1_BASE as _;
pub const CM_I2S2: *mut CM_I2S_TypeDef = CM_I2S2_BASE as _;
pub const CM_I2S3: *mut CM_I2S_TypeDef = CM_I2S3_BASE as _;
pub const CM_I2S4: *mut CM_I2S_TypeDef = CM_I2S4_BASE as _;
pub const CM_ICG: *mut CM_ICG_TypeDef = CM_ICG_BASE as _;
pub const CM_INTC: *mut CM_INTC_TypeDef = CM_INTC_BASE as _;
pub const CM_KEYSCAN: *mut CM_KEYSCAN_TypeDef = CM_KEYSCAN_BASE as _;
pub const CM_MPU: *mut CM_MPU_TypeDef = CM_MPU_BASE as _;
pub const CM_OTS: *mut CM_OTS_TypeDef = CM_OTS_BASE as _;
pub const CM_PERIC: *mut CM_PERIC_TypeDef = CM_PERIC_BASE as _;
pub const CM_PWC: *mut CM_PWC_TypeDef = CM_PWC_BASE as _;
pub const CM_QSPI: *mut CM_QSPI_TypeDef = CM_QSPI_BASE as _;
pub const CM_RMU: *mut CM_RMU_TypeDef = CM_RMU_BASE as _;
pub const CM_RTC: *mut CM_RTC_TypeDef = CM_RTC_BASE as _;
pub const CM_SDIOC1: *mut CM_SDIOC_TypeDef = CM_SDIOC1_BASE as _;
pub const CM_SDIOC2: *mut CM_SDIOC_TypeDef = CM_SDIOC2_BASE as _;
pub const CM_SPI1: *mut CM_SPI_TypeDef = CM_SPI1_BASE as _;
pub const CM_SPI2: *mut CM_SPI_TypeDef = CM_SPI2_BASE as _;
pub const CM_SPI3: *mut CM_SPI_TypeDef = CM_SPI3_BASE as _;
pub const CM_SPI4: *mut CM_SPI_TypeDef = CM_SPI4_BASE as _;
pub const CM_SRAMC: *mut CM_SRAMC_TypeDef = CM_SRAMC_BASE as _;
pub const CM_SWDT: *mut CM_SWDT_TypeDef = CM_SWDT_BASE as _;
pub const CM_TMR0_1: *mut CM_TMR0_TypeDef = CM_TMR0_1_BASE as _;
pub const CM_TMR0_2: *mut CM_TMR0_TypeDef = CM_TMR0_2_BASE as _;
pub const CM_TMR4_1: *mut CM_TMR4_TypeDef = CM_TMR4_1_BASE as _;
pub const CM_TMR4_2: *mut CM_TMR4_TypeDef = CM_TMR4_2_BASE as _;
pub const CM_TMR4_3: *mut CM_TMR4_TypeDef = CM_TMR4_3_BASE as _;
pub const CM_TMR4_ECER: *mut CM_TMR4_ECER_TypeDef = CM_TMR4_ECER_BASE as _;
pub const CM_TMR6_1: *mut CM_TMR6_TypeDef = CM_TMR6_1_BASE as _;
pub const CM_TMR6_2: *mut CM_TMR6_TypeDef = CM_TMR6_2_BASE as _;
pub const CM_TMR6_3: *mut CM_TMR6_TypeDef = CM_TMR6_3_BASE as _;
pub const CM_TMR6_COMMON: *mut CM_TMR6_COMMON_TypeDef = CM_TMR6_COMMON_BASE as _;
pub const CM_TMRA_1: *mut CM_TMRA_TypeDef = CM_TMRA_1_BASE as _;
pub const CM_TMRA_2: *mut CM_TMRA_TypeDef = CM_TMRA_2_BASE as _;
pub const CM_TMRA_3: *mut CM_TMRA_TypeDef = CM_TMRA_3_BASE as _;
pub const CM_TMRA_4: *mut CM_TMRA_TypeDef = CM_TMRA_4_BASE as _;
pub const CM_TMRA_5: *mut CM_TMRA_TypeDef = CM_TMRA_5_BASE as _;
pub const CM_TMRA_6: *mut CM_TMRA_TypeDef = CM_TMRA_6_BASE as _;
pub const CM_TRNG: *mut CM_TRNG_TypeDef = CM_TRNG_BASE as _;
pub const CM_USART1: *mut CM_USART_TypeDef = CM_USART1_BASE as _;
pub const CM_USART2: *mut CM_USART_TypeDef = CM_USART2_BASE as _;
pub const CM_USART3: *mut CM_USART_TypeDef = CM_USART3_BASE as _;
pub const CM_USART4: *mut CM_USART_TypeDef = CM_USART4_BASE as _;
pub const CM_USBFS: *mut CM_USBFS_TypeDef = CM_USBFS_BASE as _;
pub const CM_WDT: *mut CM_WDT_TypeDef = CM_WDT_BASE as _;
"###,
    );
    bindgen = bindgen.clang_arg("-DHC32F460");
    bindgen = bindgen.allowlist_file(".*usb[^\\\\/]*");
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
                "hc32f460" => hc32f460()?,
                _ => println!("Unknown MCU {}", i),
            }
        }
    }
    Ok(())
}
