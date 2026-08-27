unsafe extern "C" {
    fn DefaultHandler();
    fn IRQ000_Handler();
    fn IRQ001_Handler();
    fn IRQ002_Handler();
    fn IRQ003_Handler();
    fn IRQ004_Handler();
    fn IRQ005_Handler();
    fn IRQ006_Handler();
    fn IRQ007_Handler();
    fn IRQ008_Handler();
    fn IRQ009_Handler();
    fn IRQ010_Handler();
    fn IRQ011_Handler();
    fn IRQ012_Handler();
    fn IRQ013_Handler();
    fn IRQ014_Handler();
    fn IRQ015_Handler();
    fn EXTINT00_SWINT16_Handler();
    fn EXTINT01_SWINT17_Handler();
    fn EXTINT02_SWINT18_Handler();
    fn EXTINT03_SWINT19_Handler();
    fn EXTINT04_SWINT20_Handler();
    fn EXTINT05_SWINT21_Handler();
    fn EXTINT06_SWINT22_Handler();
    fn EXTINT07_SWINT23_Handler();
    fn EXTINT08_SWINT24_Handler();
    fn EXTINT09_SWINT25_Handler();
    fn EXTINT10_SWINT26_Handler();
    fn EXTINT11_SWINT27_Handler();
    fn EXTINT12_SWINT28_Handler();
    fn EXTINT13_SWINT29_Handler();
    fn EXTINT14_SWINT30_Handler();
    fn EXTINT15_SWINT31_Handler();
    fn DMA1_Error_Handler();
    fn DMA1_TC0_BTC0_Handler();
    fn DMA1_TC1_BTC1_Handler();
    fn DMA1_TC2_BTC2_Handler();
    fn DMA1_TC3_BTC3_Handler();
    fn DMA1_TC4_BTC4_Handler();
    fn DMA1_TC5_BTC5_Handler();
    fn EFM_PEError_ReadCol_Handler();
    fn EFM_OpEnd_Handler();
    fn QSPI_Handler();
    fn DCU1_Handler();
    fn DCU2_Handler();
    fn DCU3_Handler();
    fn DCU4_Handler();
    fn DMA2_Error_Handler();
    fn DMA2_TC0_BTC0_Handler();
    fn DMA2_TC1_BTC1_Handler();
    fn DMA2_TC2_BTC2_Handler();
    fn DMA2_TC3_BTC3_Handler();
    fn DMA2_TC4_BTC4_Handler();
    fn DMA2_TC5_BTC5_Handler();
    fn TMR0_1_Handler();
    fn TMR0_2_Handler();
    fn RTC_Handler();
    fn CLK_XtalStop_Handler();
    fn PWC_WKTM_Handler();
    fn SWDT_Handler();
    fn TMR6_1_GCmp_Handler();
    fn TMR6_1_Ovf_Udf_Handler();
    fn TMR6_1_Dte_Handler();
    fn TMR6_1_SCmp_Handler();
    fn TMRA_1_Ovf_Udf_Handler();
    fn TMRA_1_Cmp_Handler();
    fn TMR6_2_GCmp_Handler();
    fn TMR6_2_Ovf_Udf_Handler();
    fn TMR6_2_Dte_Handler();
    fn TMR6_2_SCmp_Handler();
    fn TMRA_2_Ovf_Udf_Handler();
    fn TMRA_2_Cmp_Handler();
    fn TMRA_3_Ovf_Udf_Handler();
    fn TMRA_3_Cmp_Handler();
    fn TMRA_4_Ovf_Udf_Handler();
    fn TMRA_4_Cmp_Handler();
    fn TMR4_1_GCmp_Handler();
    fn TMR4_1_Ovf_Udf_Handler();
    fn TMR4_1_Reload_Handler();
    fn TMR4_1_SCmp_Handler();
    fn TMR4_2_GCmp_Handler();
    fn TMR4_2_Ovf_Udf_Handler();
    fn TMR4_2_Reload_Handler();
    fn TMR4_2_SCmp_Handler();
    fn TMR4_3_GCmp_Handler();
    fn TMR4_3_Ovf_Udf_Handler();
    fn TMR4_3_Reload_Handler();
    fn TMR4_3_SCmp_Handler();
    fn I2C1_Handler();
    fn I2C2_Handler();
    fn CMP1_Handler();
    fn CMP2_Handler();
    fn CMP3_Handler();
    fn CMP4_Handler();
    fn USART1_Handler();
    fn USART1_TxComplete_Handler();
    fn USART2_Handler();
    fn USART2_TxComplete_Handler();
    fn SPI1_Handler();
    fn TMRA_5_Ovf_Udf_Handler();
    fn TMRA_5_Cmp_Handler();
    fn EVENT_PORT1_Handler();
    fn EVENT_PORT2_Handler();
    fn EVENT_PORT3_Handler();
    fn EVENT_PORT4_Handler();
    fn USART3_Handler();
    fn USART3_TxComplete_Handler();
    fn USART4_Handler();
    fn USART4_TxComplete_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn EMB_GR0_Handler();
    fn EMB_GR1_Handler();
    fn EMB_GR2_Handler();
    fn EMB_GR3_Handler();
    fn USART5_Handler();
    fn USART5_TxComplete_Handler();
    fn USART6_Handler();
    fn USART6_TxComplete_Handler();
    fn MCAN1_INT0_Handler();
    fn MCAN1_INT1_Handler();
    fn MCAN2_INT0_Handler();
    fn MCAN2_INT1_Handler();
    fn USART1_WKUP_Handler();
    fn PWC_LVD1_Handler();
    fn PWC_LVD2_Handler();
    fn FCM_Handler();
    fn WDT_Handler();
    fn CTC_Handler();
    fn ADC1_Handler();
    fn ADC2_Handler();
    fn ADC3_Handler();
    fn TRNG_Handler();
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 132] = [
    IRQ000_Handler,
    IRQ001_Handler,
    IRQ002_Handler,
    IRQ003_Handler,
    IRQ004_Handler,
    IRQ005_Handler,
    IRQ006_Handler,
    IRQ007_Handler,
    IRQ008_Handler,
    IRQ009_Handler,
    IRQ010_Handler,
    IRQ011_Handler,
    IRQ012_Handler,
    IRQ013_Handler,
    IRQ014_Handler,
    IRQ015_Handler,
    EXTINT00_SWINT16_Handler,
    EXTINT01_SWINT17_Handler,
    EXTINT02_SWINT18_Handler,
    EXTINT03_SWINT19_Handler,
    EXTINT04_SWINT20_Handler,
    EXTINT05_SWINT21_Handler,
    EXTINT06_SWINT22_Handler,
    EXTINT07_SWINT23_Handler,
    EXTINT08_SWINT24_Handler,
    EXTINT09_SWINT25_Handler,
    EXTINT10_SWINT26_Handler,
    EXTINT11_SWINT27_Handler,
    EXTINT12_SWINT28_Handler,
    EXTINT13_SWINT29_Handler,
    EXTINT14_SWINT30_Handler,
    EXTINT15_SWINT31_Handler,
    DMA1_Error_Handler,
    DMA1_TC0_BTC0_Handler,
    DMA1_TC1_BTC1_Handler,
    DMA1_TC2_BTC2_Handler,
    DMA1_TC3_BTC3_Handler,
    DMA1_TC4_BTC4_Handler,
    DMA1_TC5_BTC5_Handler,
    EFM_PEError_ReadCol_Handler,
    EFM_OpEnd_Handler,
    QSPI_Handler,
    DCU1_Handler,
    DCU2_Handler,
    DCU3_Handler,
    DCU4_Handler,
    DMA2_Error_Handler,
    DMA2_TC0_BTC0_Handler,
    DMA2_TC1_BTC1_Handler,
    DMA2_TC2_BTC2_Handler,
    DMA2_TC3_BTC3_Handler,
    DMA2_TC4_BTC4_Handler,
    DMA2_TC5_BTC5_Handler,
    TMR0_1_Handler,
    TMR0_2_Handler,
    RTC_Handler,
    CLK_XtalStop_Handler,
    PWC_WKTM_Handler,
    SWDT_Handler,
    TMR6_1_GCmp_Handler,
    TMR6_1_Ovf_Udf_Handler,
    TMR6_1_Dte_Handler,
    TMR6_1_SCmp_Handler,
    TMRA_1_Ovf_Udf_Handler,
    TMRA_1_Cmp_Handler,
    TMR6_2_GCmp_Handler,
    TMR6_2_Ovf_Udf_Handler,
    TMR6_2_Dte_Handler,
    TMR6_2_SCmp_Handler,
    TMRA_2_Ovf_Udf_Handler,
    TMRA_2_Cmp_Handler,
    TMRA_3_Ovf_Udf_Handler,
    TMRA_3_Cmp_Handler,
    TMRA_4_Ovf_Udf_Handler,
    TMRA_4_Cmp_Handler,
    TMR4_1_GCmp_Handler,
    TMR4_1_Ovf_Udf_Handler,
    TMR4_1_Reload_Handler,
    TMR4_1_SCmp_Handler,
    TMR4_2_GCmp_Handler,
    TMR4_2_Ovf_Udf_Handler,
    TMR4_2_Reload_Handler,
    TMR4_2_SCmp_Handler,
    TMR4_3_GCmp_Handler,
    TMR4_3_Ovf_Udf_Handler,
    TMR4_3_Reload_Handler,
    TMR4_3_SCmp_Handler,
    I2C1_Handler,
    I2C2_Handler,
    CMP1_Handler,
    CMP2_Handler,
    CMP3_Handler,
    CMP4_Handler,
    USART1_Handler,
    USART1_TxComplete_Handler,
    USART2_Handler,
    USART2_TxComplete_Handler,
    SPI1_Handler,
    TMRA_5_Ovf_Udf_Handler,
    TMRA_5_Cmp_Handler,
    EVENT_PORT1_Handler,
    EVENT_PORT2_Handler,
    EVENT_PORT3_Handler,
    EVENT_PORT4_Handler,
    USART3_Handler,
    USART3_TxComplete_Handler,
    USART4_Handler,
    USART4_TxComplete_Handler,
    SPI2_Handler,
    SPI3_Handler,
    EMB_GR0_Handler,
    EMB_GR1_Handler,
    EMB_GR2_Handler,
    EMB_GR3_Handler,
    USART5_Handler,
    USART5_TxComplete_Handler,
    USART6_Handler,
    USART6_TxComplete_Handler,
    MCAN1_INT0_Handler,
    MCAN1_INT1_Handler,
    MCAN2_INT0_Handler,
    MCAN2_INT1_Handler,
    USART1_WKUP_Handler,
    PWC_LVD1_Handler,
    PWC_LVD2_Handler,
    FCM_Handler,
    WDT_Handler,
    CTC_Handler,
    ADC1_Handler,
    ADC2_Handler,
    ADC3_Handler,
    TRNG_Handler,
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    IRQ000_Handler = 0,
    IRQ001_Handler = 1,
    IRQ002_Handler = 2,
    IRQ003_Handler = 3,
    IRQ004_Handler = 4,
    IRQ005_Handler = 5,
    IRQ006_Handler = 6,
    IRQ007_Handler = 7,
    IRQ008_Handler = 8,
    IRQ009_Handler = 9,
    IRQ010_Handler = 10,
    IRQ011_Handler = 11,
    IRQ012_Handler = 12,
    IRQ013_Handler = 13,
    IRQ014_Handler = 14,
    IRQ015_Handler = 15,
    EXTINT00_SWINT16_Handler = 16,
    EXTINT01_SWINT17_Handler = 17,
    EXTINT02_SWINT18_Handler = 18,
    EXTINT03_SWINT19_Handler = 19,
    EXTINT04_SWINT20_Handler = 20,
    EXTINT05_SWINT21_Handler = 21,
    EXTINT06_SWINT22_Handler = 22,
    EXTINT07_SWINT23_Handler = 23,
    EXTINT08_SWINT24_Handler = 24,
    EXTINT09_SWINT25_Handler = 25,
    EXTINT10_SWINT26_Handler = 26,
    EXTINT11_SWINT27_Handler = 27,
    EXTINT12_SWINT28_Handler = 28,
    EXTINT13_SWINT29_Handler = 29,
    EXTINT14_SWINT30_Handler = 30,
    EXTINT15_SWINT31_Handler = 31,
    DMA1_Error_Handler = 32,
    DMA1_TC0_BTC0_Handler = 33,
    DMA1_TC1_BTC1_Handler = 34,
    DMA1_TC2_BTC2_Handler = 35,
    DMA1_TC3_BTC3_Handler = 36,
    DMA1_TC4_BTC4_Handler = 37,
    DMA1_TC5_BTC5_Handler = 38,
    EFM_PEError_ReadCol_Handler = 39,
    EFM_OpEnd_Handler = 40,
    QSPI_Handler = 41,
    DCU1_Handler = 42,
    DCU2_Handler = 43,
    DCU3_Handler = 44,
    DCU4_Handler = 45,
    DMA2_Error_Handler = 46,
    DMA2_TC0_BTC0_Handler = 47,
    DMA2_TC1_BTC1_Handler = 48,
    DMA2_TC2_BTC2_Handler = 49,
    DMA2_TC3_BTC3_Handler = 50,
    DMA2_TC4_BTC4_Handler = 51,
    DMA2_TC5_BTC5_Handler = 52,
    TMR0_1_Handler = 53,
    TMR0_2_Handler = 54,
    RTC_Handler = 55,
    CLK_XtalStop_Handler = 56,
    PWC_WKTM_Handler = 57,
    SWDT_Handler = 58,
    TMR6_1_GCmp_Handler = 59,
    TMR6_1_Ovf_Udf_Handler = 60,
    TMR6_1_Dte_Handler = 61,
    TMR6_1_SCmp_Handler = 62,
    TMRA_1_Ovf_Udf_Handler = 63,
    TMRA_1_Cmp_Handler = 64,
    TMR6_2_GCmp_Handler = 65,
    TMR6_2_Ovf_Udf_Handler = 66,
    TMR6_2_Dte_Handler = 67,
    TMR6_2_SCmp_Handler = 68,
    TMRA_2_Ovf_Udf_Handler = 69,
    TMRA_2_Cmp_Handler = 70,
    TMRA_3_Ovf_Udf_Handler = 71,
    TMRA_3_Cmp_Handler = 72,
    TMRA_4_Ovf_Udf_Handler = 73,
    TMRA_4_Cmp_Handler = 74,
    TMR4_1_GCmp_Handler = 75,
    TMR4_1_Ovf_Udf_Handler = 76,
    TMR4_1_Reload_Handler = 77,
    TMR4_1_SCmp_Handler = 78,
    TMR4_2_GCmp_Handler = 79,
    TMR4_2_Ovf_Udf_Handler = 80,
    TMR4_2_Reload_Handler = 81,
    TMR4_2_SCmp_Handler = 82,
    TMR4_3_GCmp_Handler = 83,
    TMR4_3_Ovf_Udf_Handler = 84,
    TMR4_3_Reload_Handler = 85,
    TMR4_3_SCmp_Handler = 86,
    I2C1_Handler = 87,
    I2C2_Handler = 88,
    CMP1_Handler = 89,
    CMP2_Handler = 90,
    CMP3_Handler = 91,
    CMP4_Handler = 92,
    USART1_Handler = 93,
    USART1_TxComplete_Handler = 94,
    USART2_Handler = 95,
    USART2_TxComplete_Handler = 96,
    SPI1_Handler = 97,
    TMRA_5_Ovf_Udf_Handler = 98,
    TMRA_5_Cmp_Handler = 99,
    EVENT_PORT1_Handler = 100,
    EVENT_PORT2_Handler = 101,
    EVENT_PORT3_Handler = 102,
    EVENT_PORT4_Handler = 103,
    USART3_Handler = 104,
    USART3_TxComplete_Handler = 105,
    USART4_Handler = 106,
    USART4_TxComplete_Handler = 107,
    SPI2_Handler = 108,
    SPI3_Handler = 109,
    EMB_GR0_Handler = 110,
    EMB_GR1_Handler = 111,
    EMB_GR2_Handler = 112,
    EMB_GR3_Handler = 113,
    USART5_Handler = 114,
    USART5_TxComplete_Handler = 115,
    USART6_Handler = 116,
    USART6_TxComplete_Handler = 117,
    MCAN1_INT0_Handler = 118,
    MCAN1_INT1_Handler = 119,
    MCAN2_INT0_Handler = 120,
    MCAN2_INT1_Handler = 121,
    USART1_WKUP_Handler = 122,
    PWC_LVD1_Handler = 123,
    PWC_LVD2_Handler = 124,
    FCM_Handler = 125,
    WDT_Handler = 126,
    CTC_Handler = 127,
    ADC1_Handler = 128,
    ADC2_Handler = 129,
    ADC3_Handler = 130,
    TRNG_Handler = 131,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
