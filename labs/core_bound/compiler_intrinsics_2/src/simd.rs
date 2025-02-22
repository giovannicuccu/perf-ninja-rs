use std::arch::x86_64::*; // Intel SIMD intrinsic mappings

#[allow(non_camel_case_types)]
pub type u8x32 = __m256i;
#[allow(non_upper_case_globals)]
pub const u8x32_LENGTH: usize = 32;

/// Return a 256-bit vector containing 8 infinity values of f32
#[inline(always)]
pub fn u8x32_mask() -> u8x32 {
    unsafe { _mm256_set1_epi8(0xa) }
}

#[inline(always)]
pub fn from_u8x32_to_vector_register(pointer: *const u8x32) -> u8x32 {
    unsafe { _mm256_loadu_si256(pointer) }
}

#[inline(always)]
pub fn compare_registers(reg1: u8x32, reg2: u8x32) -> u8x32 {
    unsafe { _mm256_cmpeq_epi8(reg1, reg2) }
}

#[inline(always)]
pub fn register_to_i32(reg1: u8x32) -> i32 {
    unsafe { _mm256_movemask_epi8(reg1) }
}

#[inline(always)]
pub fn trailing_zero(val: u32) -> u32 {
    unsafe { _tzcnt_u32(val) }
}

