use crate::*;

use core::ffi::*;

pub unsafe fn HCLK_VALUE() -> c_ulong {
    SystemCoreClock
        >> ((((*CM_CMU).SCFGR & CMU_SCFGR_HCLKS) >> (CMU_SCFGR_HCLKS_POS as i32)) as i32)
}

pub unsafe fn I2C_SRC_CLK() -> c_ulong {
    SystemCoreClock
        >> ((((*CM_CMU).SCFGR & CMU_SCFGR_PCLK3S) >> (CMU_SCFGR_PCLK0S_POS as i32)) as i32)
}

pub const fn DEC2BCD(x: c_ulong) -> c_ulong {
    ((x / 10) << 4) + (x % 10)
}

pub const fn BCD2DEC(x: c_ulong) -> c_ulong {
    (x >> 4) * 10 + (x & 0x0F)
}

pub const fn ARRAY_SZ<T>(x: &[T]) -> usize {
    x.len()
}

pub use core::cmp::min as LL_MIN;

pub use core::cmp::max as LL_MAX;

pub const fn IS_FUNCTIONAL_STATE(state: c_uint) -> bool {
    state == DISABLE || state == ENABLE
}

pub const fn IS_ADDR_ALIGN(addr: uint32_t, align: uint32_t) -> bool {
    addr & (align - 1) == 0
}

pub const fn IS_ADDR_ALIGN_HALFWORD(addr: uint32_t) -> bool {
    addr & 0x1 == 0
}

pub const fn IS_ADDR_ALIGN_WORD(addr: uint32_t) -> bool {
    addr & 0x3 == 0
}

pub const fn __REG_OFS(regAddr: uint32_t) -> uint32_t {
    regAddr - __PERIPH_BASE
}

pub const fn __BIT_BAND_ADDR(regAddr: uint32_t, pos: uint32_t) -> uint32_t {
    (__REG_OFS(regAddr) << 5) + (pos << 2) + __PERIPH_BIT_BAND_BASE
}

pub const fn PERIPH_BIT_BAND(regAddr: uint32_t, pos: uint32_t) -> *mut uint32_t {
    __BIT_BAND_ADDR(regAddr, pos) as *mut uint32_t
}

pub unsafe fn RW_MEM8(addr: uint32_t) -> uint8_t {
    core::ptr::read_volatile(addr as *mut uint8_t)
}

pub unsafe fn RW_MEM16(addr: uint32_t) -> uint16_t {
    core::ptr::read_volatile(addr as *mut uint16_t)
}

pub unsafe fn RW_MEM32(addr: uint32_t) -> uint32_t {
    core::ptr::read_volatile(addr as *mut uint32_t)
}

pub const fn EFM_SECTOR_ADDR(x: uint32_t) -> uint32_t {
    EFM_SECTOR_SIZE * x
}

pub const fn EFM_OTP_BLOCK_LOCKADDR(x: uint32_t) -> uint32_t {
    EFM_OTP_LOCK_ADDR + 0x04 * x
}

pub const fn RTC_DEC2BCD(data: uint32_t) -> uint32_t {
    ((data / 10) << 4) + (data % 10)
}

pub const fn RTC_BCD2DEC(data: uint32_t) -> uint32_t {
    (data >> 4) * 10 + (data & 0x0F)
}
