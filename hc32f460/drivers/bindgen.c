#include "drivers/cmsis/Device/HDSC/hc32f4xx/Include/hc32f4xx.h"
#include "drivers/hc32_ll_driver/inc\hc32f460_ll_interrupts_share.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_adc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_aes.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_aos.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_can.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_clk.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_cmp.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_crc.h"
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
#include "drivers/hc32_ll_driver/inc\hc32_ll_i2s.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_icg.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_interrupts.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_keyscan.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_mpu.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_ots.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_pwc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_qspi.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_rmu.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_rtc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_sdioc.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_spi.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_sram.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_swdt.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmr0.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmr4.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmr6.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_tmra.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_trng.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_usart.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_usb.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_utility.h"
#include "drivers/hc32_ll_driver/inc\hc32_ll_wdt.h"

// Static wrappers

void AOS_SW_Trigger__extern(void) { AOS_SW_Trigger(); }
void EFM_REG_Unlock__extern(void) { EFM_REG_Unlock(); }
void EFM_REG_Lock__extern(void) { EFM_REG_Lock(); }
void EFM_REMAP_Unlock__extern(void) { EFM_REMAP_Unlock(); }
void EFM_REMAP_Lock__extern(void) { EFM_REMAP_Lock(); }
void FCM_SetUpperLimit__extern(CM_FCM_TypeDef *FCMx, uint16_t u16Limit) { FCM_SetUpperLimit(FCMx, u16Limit); }
void FCM_SetLowerLimit__extern(CM_FCM_TypeDef *FCMx, uint16_t u16Limit) { FCM_SetLowerLimit(FCMx, u16Limit); }
void GPIO_REG_Lock__extern(void) { GPIO_REG_Lock(); }
void GPIO_REG_Unlock__extern(void) { GPIO_REG_Unlock(); }
uint32_t KEYSCAN_GetKeyoutIdx__extern(void) { return KEYSCAN_GetKeyoutIdx(); }
void MPU_REG_Unlock__extern(void) { MPU_REG_Unlock(); }
void MPU_REG_Lock__extern(void) { MPU_REG_Lock(); }
void OTS_Start__extern(void) { OTS_Start(); }
void OTS_Stop__extern(void) { OTS_Stop(); }
void PWC_REG_Lock__extern(uint16_t u16Module) { PWC_REG_Lock(u16Module); }
void PWC_REG_Unlock__extern(uint16_t u16Module) { PWC_REG_Unlock(u16Module); }
void PWC_FCG0_REG_Lock__extern(void) { PWC_FCG0_REG_Lock(); }
void PWC_FCG0_REG_Unlock__extern(void) { PWC_FCG0_REG_Unlock(); }
void QSPI_WriteDirectCommValue__extern(uint8_t u8Value) { QSPI_WriteDirectCommValue(u8Value); }
uint8_t QSPI_ReadDirectCommValue__extern(void) { return QSPI_ReadDirectCommValue(); }
void SRAM_REG_Lock__extern(void) { SRAM_REG_Lock(); }
void SRAM_REG_Unlock__extern(void) { SRAM_REG_Unlock(); }
uint16_t SWDT_GetCountValue__extern(void) { return SWDT_GetCountValue(); }
uint32_t TMR6_GetSWSyncStartStatus__extern(void) { return TMR6_GetSWSyncStartStatus(); }
uint8_t usb_getcurmod__extern(LL_USB_TypeDef *USBx) { return usb_getcurmod(USBx); }
void usb_normalinten__extern(LL_USB_TypeDef *USBx) { usb_normalinten(USBx); }
void usb_clrandmskepint__extern(LL_USB_TypeDef *USBx) { usb_clrandmskepint(USBx); }
void usb_coreconn__extern(LL_USB_TypeDef *USBx) { usb_coreconn(USBx); }
void usb_runtestmode__extern(LL_USB_TypeDef *USBx, uint32_t reg) { usb_runtestmode(USBx, reg); }
void usb_ginten__extern(LL_USB_TypeDef *USBx) { usb_ginten(USBx); }
void usb_gintdis__extern(LL_USB_TypeDef *USBx) { usb_gintdis(USBx); }
uint32_t usb_getcoreintr__extern(LL_USB_TypeDef *USBx) { return usb_getcoreintr(USBx); }
uint32_t usb_getalloepintr__extern(LL_USB_TypeDef *USBx) { return usb_getalloepintr(USBx); }
uint32_t usb_getoepintbit__extern(LL_USB_TypeDef *USBx, uint8_t epnum) { return usb_getoepintbit(USBx, epnum); }
uint32_t usb_getalliepintr__extern(LL_USB_TypeDef *USBx) { return usb_getalliepintr(USBx); }
void usb_devaddrset__extern(LL_USB_TypeDef *USBx, uint8_t address) { usb_devaddrset(USBx, address); }
void usb_PhySelect__extern(LL_USB_TypeDef *USBx, uint8_t PhyType) { usb_PhySelect(USBx, PhyType); }
void usb_DevPhySelect__extern(LL_USB_TypeDef *USBx, uint8_t PhyType) { usb_DevPhySelect(USBx, PhyType); }
void usb_DmaCmd__extern(LL_USB_TypeDef *USBx, uint8_t DmaCmd) { usb_DmaCmd(USBx, DmaCmd); }
void usb_BurstLenConfig__extern(LL_USB_TypeDef *USBx, uint8_t len) { usb_BurstLenConfig(USBx, len); }
void usb_FrameIntervalConfig__extern(LL_USB_TypeDef *USBx, uint32_t interval) { usb_FrameIntervalConfig(USBx, interval); }
uint16_t WDT_GetCountValue__extern(void) { return WDT_GetCountValue(); }
