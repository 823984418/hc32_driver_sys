#include "drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_adc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_aes.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_aos.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_clk.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_cmp.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_crc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_ctc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_dac.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_dbgc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_dcu.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_def.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_dma.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_efm.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_emb.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_event_port.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_fcg.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_fcm.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_gpio.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_hash.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_i2c.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_icg.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_interrupts.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_keyscan.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_mcan.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_mpu.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_pwc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_qspi.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_rmu.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_rtc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_smc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_spi.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_sram.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_swdt.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmr0.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmr4.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmr6.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmra.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_trng.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_usart.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_utility.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_wdt.h"

// Static wrappers

void AOS_SW_Trigger__extern(void) { AOS_SW_Trigger(); }
void CTC_Start__extern(void) { CTC_Start(); }
void CTC_Stop__extern(void) { CTC_Stop(); }
void EFM_REG_Unlock__extern(void) { EFM_REG_Unlock(); }
void EFM_REG_Lock__extern(void) { EFM_REG_Lock(); }
void EFM_REMAP_Unlock__extern(void) { EFM_REMAP_Unlock(); }
void EFM_REMAP_Lock__extern(void) { EFM_REMAP_Lock(); }
void EFM_OTP_REG_Unlock__extern(void) { EFM_OTP_REG_Unlock(); }
void EFM_OTP_REG_Lock__extern(void) { EFM_OTP_REG_Lock(); }
void FCM_SetUpperLimit__extern(CM_FCM_TypeDef *FCMx, uint16_t u16Limit) { FCM_SetUpperLimit(FCMx, u16Limit); }
void FCM_SetLowerLimit__extern(CM_FCM_TypeDef *FCMx, uint16_t u16Limit) { FCM_SetLowerLimit(FCMx, u16Limit); }
void GPIO_REG_Lock__extern(void) { GPIO_REG_Lock(); }
void GPIO_REG_Unlock__extern(void) { GPIO_REG_Unlock(); }
uint32_t KEYSCAN_GetKeyoutIdx__extern(void) { return KEYSCAN_GetKeyoutIdx(); }
void MPU_REG_Unlock__extern(void) { MPU_REG_Unlock(); }
void MPU_REG_Lock__extern(void) { MPU_REG_Lock(); }
void PWC_REG_Lock__extern(uint16_t u16Module) { PWC_REG_Lock(u16Module); }
void PWC_REG_Unlock__extern(uint16_t u16Module) { PWC_REG_Unlock(u16Module); }
void PWC_FCG0_REG_Lock__extern(void) { PWC_FCG0_REG_Lock(); }
void PWC_FCG0_REG_Unlock__extern(void) { PWC_FCG0_REG_Unlock(); }
uint8_t QSPI_ReadDirectCommValue__extern(void) { return QSPI_ReadDirectCommValue(); }
void EXMC_SMC_EntryLowPower__extern(void) { EXMC_SMC_EntryLowPower(); }
void EXMC_SMC_ExitLowPower__extern(void) { EXMC_SMC_ExitLowPower(); }
uint32_t EXMC_SMC_GetStatus__extern(void) { return EXMC_SMC_GetStatus(); }
void SRAM_REG_Lock__extern(void) { SRAM_REG_Lock(); }
void SRAM_REG_Unlock__extern(void) { SRAM_REG_Unlock(); }
uint16_t SWDT_GetCountValue__extern(void) { return SWDT_GetCountValue(); }
uint32_t TMR6_GetSWSyncStartStatus__extern(void) { return TMR6_GetSWSyncStartStatus(); }
uint16_t WDT_GetCountValue__extern(void) { return WDT_GetCountValue(); }
