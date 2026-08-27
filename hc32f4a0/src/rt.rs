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
    fn IRQ016_Handler();
    fn IRQ017_Handler();
    fn IRQ018_Handler();
    fn IRQ019_Handler();
    fn IRQ020_Handler();
    fn IRQ021_Handler();
    fn IRQ022_Handler();
    fn IRQ023_Handler();
    fn IRQ024_Handler();
    fn IRQ025_Handler();
    fn IRQ026_Handler();
    fn IRQ027_Handler();
    fn IRQ028_Handler();
    fn IRQ029_Handler();
    fn IRQ030_Handler();
    fn IRQ031_Handler();
    fn IRQ032_Handler();
    fn IRQ033_Handler();
    fn IRQ034_Handler();
    fn IRQ035_Handler();
    fn IRQ036_Handler();
    fn IRQ037_Handler();
    fn IRQ038_Handler();
    fn IRQ039_Handler();
    fn IRQ040_Handler();
    fn IRQ041_Handler();
    fn IRQ042_Handler();
    fn IRQ043_Handler();
    fn IRQ044_Handler();
    fn IRQ045_Handler();
    fn IRQ046_Handler();
    fn IRQ047_Handler();
    fn IRQ048_Handler();
    fn IRQ049_Handler();
    fn IRQ050_Handler();
    fn IRQ051_Handler();
    fn IRQ052_Handler();
    fn IRQ053_Handler();
    fn IRQ054_Handler();
    fn IRQ055_Handler();
    fn IRQ056_Handler();
    fn IRQ057_Handler();
    fn IRQ058_Handler();
    fn IRQ059_Handler();
    fn IRQ060_Handler();
    fn IRQ061_Handler();
    fn IRQ062_Handler();
    fn IRQ063_Handler();
    fn IRQ064_Handler();
    fn IRQ065_Handler();
    fn IRQ066_Handler();
    fn IRQ067_Handler();
    fn IRQ068_Handler();
    fn IRQ069_Handler();
    fn IRQ070_Handler();
    fn IRQ071_Handler();
    fn IRQ072_Handler();
    fn IRQ073_Handler();
    fn IRQ074_Handler();
    fn IRQ075_Handler();
    fn IRQ076_Handler();
    fn IRQ077_Handler();
    fn IRQ078_Handler();
    fn IRQ079_Handler();
    fn IRQ080_Handler();
    fn IRQ081_Handler();
    fn IRQ082_Handler();
    fn IRQ083_Handler();
    fn IRQ084_Handler();
    fn IRQ085_Handler();
    fn IRQ086_Handler();
    fn IRQ087_Handler();
    fn IRQ088_Handler();
    fn IRQ089_Handler();
    fn IRQ090_Handler();
    fn IRQ091_Handler();
    fn IRQ092_Handler();
    fn IRQ093_Handler();
    fn IRQ094_Handler();
    fn IRQ095_Handler();
    fn IRQ096_Handler();
    fn IRQ097_Handler();
    fn IRQ098_Handler();
    fn IRQ099_Handler();
    fn IRQ100_Handler();
    fn IRQ101_Handler();
    fn IRQ102_Handler();
    fn IRQ103_Handler();
    fn IRQ104_Handler();
    fn IRQ105_Handler();
    fn IRQ106_Handler();
    fn IRQ107_Handler();
    fn IRQ108_Handler();
    fn IRQ109_Handler();
    fn IRQ110_Handler();
    fn IRQ111_Handler();
    fn IRQ112_Handler();
    fn IRQ113_Handler();
    fn IRQ114_Handler();
    fn IRQ115_Handler();
    fn IRQ116_Handler();
    fn IRQ117_Handler();
    fn IRQ118_Handler();
    fn IRQ119_Handler();
    fn IRQ120_Handler();
    fn IRQ121_Handler();
    fn IRQ122_Handler();
    fn IRQ123_Handler();
    fn IRQ124_Handler();
    fn IRQ125_Handler();
    fn IRQ126_Handler();
    fn IRQ127_Handler();
    fn IRQ128_Handler();
    fn IRQ129_Handler();
    fn IRQ130_Handler();
    fn IRQ131_Handler();
    fn IRQ132_Handler();
    fn IRQ133_Handler();
    fn IRQ134_Handler();
    fn IRQ135_Handler();
    fn IRQ136_Handler();
    fn IRQ137_Handler();
    fn IRQ138_Handler();
    fn IRQ139_Handler();
    fn IRQ140_Handler();
    fn IRQ141_Handler();
    fn IRQ142_Handler();
    fn IRQ143_Handler();
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 144] = [
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
    IRQ016_Handler,
    IRQ017_Handler,
    IRQ018_Handler,
    IRQ019_Handler,
    IRQ020_Handler,
    IRQ021_Handler,
    IRQ022_Handler,
    IRQ023_Handler,
    IRQ024_Handler,
    IRQ025_Handler,
    IRQ026_Handler,
    IRQ027_Handler,
    IRQ028_Handler,
    IRQ029_Handler,
    IRQ030_Handler,
    IRQ031_Handler,
    IRQ032_Handler,
    IRQ033_Handler,
    IRQ034_Handler,
    IRQ035_Handler,
    IRQ036_Handler,
    IRQ037_Handler,
    IRQ038_Handler,
    IRQ039_Handler,
    IRQ040_Handler,
    IRQ041_Handler,
    IRQ042_Handler,
    IRQ043_Handler,
    IRQ044_Handler,
    IRQ045_Handler,
    IRQ046_Handler,
    IRQ047_Handler,
    IRQ048_Handler,
    IRQ049_Handler,
    IRQ050_Handler,
    IRQ051_Handler,
    IRQ052_Handler,
    IRQ053_Handler,
    IRQ054_Handler,
    IRQ055_Handler,
    IRQ056_Handler,
    IRQ057_Handler,
    IRQ058_Handler,
    IRQ059_Handler,
    IRQ060_Handler,
    IRQ061_Handler,
    IRQ062_Handler,
    IRQ063_Handler,
    IRQ064_Handler,
    IRQ065_Handler,
    IRQ066_Handler,
    IRQ067_Handler,
    IRQ068_Handler,
    IRQ069_Handler,
    IRQ070_Handler,
    IRQ071_Handler,
    IRQ072_Handler,
    IRQ073_Handler,
    IRQ074_Handler,
    IRQ075_Handler,
    IRQ076_Handler,
    IRQ077_Handler,
    IRQ078_Handler,
    IRQ079_Handler,
    IRQ080_Handler,
    IRQ081_Handler,
    IRQ082_Handler,
    IRQ083_Handler,
    IRQ084_Handler,
    IRQ085_Handler,
    IRQ086_Handler,
    IRQ087_Handler,
    IRQ088_Handler,
    IRQ089_Handler,
    IRQ090_Handler,
    IRQ091_Handler,
    IRQ092_Handler,
    IRQ093_Handler,
    IRQ094_Handler,
    IRQ095_Handler,
    IRQ096_Handler,
    IRQ097_Handler,
    IRQ098_Handler,
    IRQ099_Handler,
    IRQ100_Handler,
    IRQ101_Handler,
    IRQ102_Handler,
    IRQ103_Handler,
    IRQ104_Handler,
    IRQ105_Handler,
    IRQ106_Handler,
    IRQ107_Handler,
    IRQ108_Handler,
    IRQ109_Handler,
    IRQ110_Handler,
    IRQ111_Handler,
    IRQ112_Handler,
    IRQ113_Handler,
    IRQ114_Handler,
    IRQ115_Handler,
    IRQ116_Handler,
    IRQ117_Handler,
    IRQ118_Handler,
    IRQ119_Handler,
    IRQ120_Handler,
    IRQ121_Handler,
    IRQ122_Handler,
    IRQ123_Handler,
    IRQ124_Handler,
    IRQ125_Handler,
    IRQ126_Handler,
    IRQ127_Handler,
    IRQ128_Handler,
    IRQ129_Handler,
    IRQ130_Handler,
    IRQ131_Handler,
    IRQ132_Handler,
    IRQ133_Handler,
    IRQ134_Handler,
    IRQ135_Handler,
    IRQ136_Handler,
    IRQ137_Handler,
    IRQ138_Handler,
    IRQ139_Handler,
    IRQ140_Handler,
    IRQ141_Handler,
    IRQ142_Handler,
    IRQ143_Handler,
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
    IRQ016_Handler = 16,
    IRQ017_Handler = 17,
    IRQ018_Handler = 18,
    IRQ019_Handler = 19,
    IRQ020_Handler = 20,
    IRQ021_Handler = 21,
    IRQ022_Handler = 22,
    IRQ023_Handler = 23,
    IRQ024_Handler = 24,
    IRQ025_Handler = 25,
    IRQ026_Handler = 26,
    IRQ027_Handler = 27,
    IRQ028_Handler = 28,
    IRQ029_Handler = 29,
    IRQ030_Handler = 30,
    IRQ031_Handler = 31,
    IRQ032_Handler = 32,
    IRQ033_Handler = 33,
    IRQ034_Handler = 34,
    IRQ035_Handler = 35,
    IRQ036_Handler = 36,
    IRQ037_Handler = 37,
    IRQ038_Handler = 38,
    IRQ039_Handler = 39,
    IRQ040_Handler = 40,
    IRQ041_Handler = 41,
    IRQ042_Handler = 42,
    IRQ043_Handler = 43,
    IRQ044_Handler = 44,
    IRQ045_Handler = 45,
    IRQ046_Handler = 46,
    IRQ047_Handler = 47,
    IRQ048_Handler = 48,
    IRQ049_Handler = 49,
    IRQ050_Handler = 50,
    IRQ051_Handler = 51,
    IRQ052_Handler = 52,
    IRQ053_Handler = 53,
    IRQ054_Handler = 54,
    IRQ055_Handler = 55,
    IRQ056_Handler = 56,
    IRQ057_Handler = 57,
    IRQ058_Handler = 58,
    IRQ059_Handler = 59,
    IRQ060_Handler = 60,
    IRQ061_Handler = 61,
    IRQ062_Handler = 62,
    IRQ063_Handler = 63,
    IRQ064_Handler = 64,
    IRQ065_Handler = 65,
    IRQ066_Handler = 66,
    IRQ067_Handler = 67,
    IRQ068_Handler = 68,
    IRQ069_Handler = 69,
    IRQ070_Handler = 70,
    IRQ071_Handler = 71,
    IRQ072_Handler = 72,
    IRQ073_Handler = 73,
    IRQ074_Handler = 74,
    IRQ075_Handler = 75,
    IRQ076_Handler = 76,
    IRQ077_Handler = 77,
    IRQ078_Handler = 78,
    IRQ079_Handler = 79,
    IRQ080_Handler = 80,
    IRQ081_Handler = 81,
    IRQ082_Handler = 82,
    IRQ083_Handler = 83,
    IRQ084_Handler = 84,
    IRQ085_Handler = 85,
    IRQ086_Handler = 86,
    IRQ087_Handler = 87,
    IRQ088_Handler = 88,
    IRQ089_Handler = 89,
    IRQ090_Handler = 90,
    IRQ091_Handler = 91,
    IRQ092_Handler = 92,
    IRQ093_Handler = 93,
    IRQ094_Handler = 94,
    IRQ095_Handler = 95,
    IRQ096_Handler = 96,
    IRQ097_Handler = 97,
    IRQ098_Handler = 98,
    IRQ099_Handler = 99,
    IRQ100_Handler = 100,
    IRQ101_Handler = 101,
    IRQ102_Handler = 102,
    IRQ103_Handler = 103,
    IRQ104_Handler = 104,
    IRQ105_Handler = 105,
    IRQ106_Handler = 106,
    IRQ107_Handler = 107,
    IRQ108_Handler = 108,
    IRQ109_Handler = 109,
    IRQ110_Handler = 110,
    IRQ111_Handler = 111,
    IRQ112_Handler = 112,
    IRQ113_Handler = 113,
    IRQ114_Handler = 114,
    IRQ115_Handler = 115,
    IRQ116_Handler = 116,
    IRQ117_Handler = 117,
    IRQ118_Handler = 118,
    IRQ119_Handler = 119,
    IRQ120_Handler = 120,
    IRQ121_Handler = 121,
    IRQ122_Handler = 122,
    IRQ123_Handler = 123,
    IRQ124_Handler = 124,
    IRQ125_Handler = 125,
    IRQ126_Handler = 126,
    IRQ127_Handler = 127,
    IRQ128_Handler = 128,
    IRQ129_Handler = 129,
    IRQ130_Handler = 130,
    IRQ131_Handler = 131,
    IRQ132_Handler = 132,
    IRQ133_Handler = 133,
    IRQ134_Handler = 134,
    IRQ135_Handler = 135,
    IRQ136_Handler = 136,
    IRQ137_Handler = 137,
    IRQ138_Handler = 138,
    IRQ139_Handler = 139,
    IRQ140_Handler = 140,
    IRQ141_Handler = 141,
    IRQ142_Handler = 142,
    IRQ143_Handler = 143,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
