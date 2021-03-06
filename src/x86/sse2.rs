use std::mem;
use std::os::raw::c_void;
use std::ptr;

use simd_llvm::{
    simd_cast, simd_shuffle2, simd_shuffle4, simd_shuffle8, simd_shuffle16,
};
use x86::__m128i;
use v128::*;
use v64::*;

/// Provide a hint to the processor that the code sequence is a spin-wait loop.
///
/// This can help improve the performance and power consumption of spin-wait
/// loops.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_pause() {
    unsafe { pause() }
}

/// Invalidate and flush the cache line that contains `p` from all levels of
/// the cache hierarchy.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_clflush(p: *mut c_void) {
    clflush(p)
}

/// Perform a serializing operation on all load-from-memory instructions
/// that were issued prior to this instruction.
///
/// Guarantees that every load instruction that precedes, in program order, is
/// globally visible before any load instruction which follows the fence in
/// program order.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_lfence() {
    unsafe { lfence() }
}

/// Perform a serializing operation on all load-from-memory and store-to-memory
/// instructions that were issued prior to this instruction.
///
/// Guarantees that every memory access that precedes, in program order, the
/// memory fence instruction is globally visible before any memory instruction
/// which follows the fence in program order.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mfence() {
    unsafe { mfence() }
}

/// Add packed 8-bit integers in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_add_epi8(a: i8x16, b: i8x16) -> i8x16 {
    a + b
}

/// Add packed 16-bit integers in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_add_epi16(a: i16x8, b: i16x8) -> i16x8 {
    a + b
}

/// Add packed 32-bit integers in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_add_epi32(a: i32x4, b: i32x4) -> i32x4 {
    a + b
}

/// Add packed 64-bit integers in `a` and "b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_add_epi64(a: i64x2, b: i64x2) -> i64x2 {
    a + b
}

/// Add packed 8-bit integers in `a` and `b` using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_adds_epi8(a: i8x16, b: i8x16) -> i8x16 {
    unsafe { paddsb(a, b) }
}

/// Add packed 16-bit integers in `a` and `b` using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_adds_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { paddsw(a, b) }
}

/// Add packed unsigned 8-bit integers in `a` and `b` using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_adds_epu8(a: u8x16, b: u8x16) -> u8x16 {
    unsafe { paddsub(a, b) }
}

/// Add packed unsigned 16-bit integers in `a` and `b` using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_adds_epu16(a: u16x8, b: u16x8) -> u16x8 {
    unsafe { paddsuw(a, b) }
}

/// Average packed unsigned 8-bit integers in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_avg_epu8(a: u8x16, b: u8x16) -> u8x16 {
    unsafe { pavgb(a, b) }
}

/// Average packed unsigned 16-bit integers in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_avg_epu16(a: u16x8, b: u16x8) -> u16x8 {
    unsafe { pavgw(a, b) }
}

/// Multiply and then horizontally add signed 16 bit integers in `a` and `b`.
///
/// Multiply packed signed 16-bit integers in `a` and `b`, producing
/// intermediate signed 32-bit integers. Horizontally add adjacent pairs of
/// intermediate 32-bit integers.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_madd_epi16(a: i16x8, b: i16x8) -> i32x4 {
    unsafe { pmaddwd(a, b) }
}

/// Compare packed 16-bit integers in `a` and `b`, and return the packed
/// maximum values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_max_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { pmaxsw(a, b) }
}

/// Compare packed unsigned 8-bit integers in `a` and `b`, and return the
/// packed maximum values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_max_epu8(a: u8x16, b: u8x16) -> u8x16 {
    unsafe { pmaxub(a, b) }
}

/// Compare packed 16-bit integers in `a` and `b`, and return the packed
/// minimum values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_min_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { pminsw(a, b) }
}

/// Compare packed unsigned 8-bit integers in `a` and `b`, and return the
/// packed minimum values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_min_epu8(a: u8x16, b: u8x16) -> u8x16 {
    unsafe { pminub(a, b) }
}

/// Multiply the packed 16-bit integers in `a` and `b`.
///
/// The multiplication produces intermediate 32-bit integers, and returns the
/// high 16 bits of the intermediate integers.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mulhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { pmulhw(a, b) }
}

/// Multiply the packed unsigned 16-bit integers in `a` and `b`.
///
/// The multiplication produces intermediate 32-bit integers, and returns the
/// high 16 bits of the intermediate integers.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mulhi_epu16(a: u16x8, b: u16x8) -> u16x8 {
    unsafe { pmulhuw(a, b) }
}

/// Multiply the packed 16-bit integers in `a` and `b`.
///
/// The multiplication produces intermediate 32-bit integers, and returns the
/// low 16 bits of the intermediate integers.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mullo_epi16(a: i16x8, b: i16x8) -> i16x8 {
    a * b
}

/// Multiply the low unsigned 32-bit integers from each packed 64-bit element
/// in `a` and `b`.
///
/// Return the unsigned 64-bit results.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mul_epu32(a: u32x4, b: u32x4) -> u64x2 {
    unsafe { pmuludq(a, b) }
}

/// Sum the absolute differences of packed unsigned 8-bit integers.
///
/// Compute the absolute differences of packed unsigned 8-bit integers in `a`
/// and `b`, then horizontally sum each consecutive 8 differences to produce
/// two unsigned 16-bit integers, and pack these unsigned 16-bit integers in
/// the low 16 bits of 64-bit elements returned.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sad_epu8(a: u8x16, b: u8x16) -> u64x2 {
    unsafe { psadbw(a, b) }
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sub_epi8(a: i8x16, b: i8x16) -> i8x16 {
    a - b
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sub_epi16(a: i16x8, b: i16x8) -> i16x8 {
    a - b
}

/// Subtract packed 32-bit integers in `b` from packed 32-bit integers in `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sub_epi32(a: i32x4, b: i32x4) -> i32x4 {
    a - b
}

/// Subtract packed 64-bit integers in `b` from packed 64-bit integers in `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sub_epi64(a: i64x2, b: i64x2) -> i64x2 {
    a - b
}

/// Subtract packed 8-bit integers in `b` from packed 8-bit integers in `a`
/// using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_subs_epi8(a: i8x16, b: i8x16) -> i8x16 {
    unsafe { psubsb(a, b) }
}

/// Subtract packed 16-bit integers in `b` from packed 16-bit integers in `a`
/// using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_subs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { psubsw(a, b) }
}

/// Subtract packed unsigned 8-bit integers in `b` from packed unsigned 8-bit
/// integers in `a` using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_subs_epu8(a: u8x16, b: u8x16) -> u8x16 {
    unsafe { psubusb(a, b) }
}

/// Subtract packed unsigned 16-bit integers in `b` from packed unsigned 16-bit
/// integers in `a` using saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_subs_epu16(a: u16x8, b: u16x8) -> u16x8 {
    unsafe { psubusw(a, b) }
}

/// Shift `a` left by `imm8` bytes while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_slli_si128(a: __m128i, imm8: i32) -> __m128i {
    let (zero, imm8) = (__m128i::splat(0), imm8 as u32);
    const fn sub(a: u32, b: u32) -> u32 { a - b }
    macro_rules! shuffle {
        ($shift:expr) => {
            unsafe {
                simd_shuffle16::<__m128i, __m128i>(zero, a, [
                    sub(16, $shift), sub(17, $shift),
                    sub(18, $shift), sub(19, $shift),
                    sub(20, $shift), sub(21, $shift),
                    sub(22, $shift), sub(23, $shift),
                    sub(24, $shift), sub(25, $shift),
                    sub(26, $shift), sub(27, $shift),
                    sub(28, $shift), sub(29, $shift),
                    sub(30, $shift), sub(31, $shift),
                ])
            }
        }
    }
    match imm8 {
        0 => shuffle!(0), 1 => shuffle!(1),
        2 => shuffle!(2), 3 => shuffle!(3),
        4 => shuffle!(4), 5 => shuffle!(5),
        6 => shuffle!(6), 7 => shuffle!(7),
        8 => shuffle!(8), 9 => shuffle!(9),
        10 => shuffle!(10), 11 => shuffle!(11),
        12 => shuffle!(12), 13 => shuffle!(13),
        14 => shuffle!(14), 15 => shuffle!(15),
        _ => shuffle!(16),
    }
}

/// Shift `a` left by `imm8` bytes while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_bslli_si128(a: __m128i, imm8: i32) -> __m128i {
    _mm_slli_si128(a, imm8)
}

/// Shift `a` right by `imm8` bytes while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_bsrli_si128(a: __m128i, imm8: i32) -> __m128i {
    _mm_srli_si128(a, imm8)
}

/// Shift packed 16-bit integers in `a` left by `imm8` while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_slli_epi16(a: i16x8, imm8: i32) -> i16x8  {
    unsafe { pslliw(a, imm8) }
}

/// Shift packed 16-bit integers in `a` left by `count` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sll_epi16(a: i16x8, count: i16x8) -> i16x8 {
    unsafe { psllw(a, count) }
}

/// Shift packed 32-bit integers in `a` left by `imm8` while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_slli_epi32(a: i32x4, imm8: i32) -> i32x4 {
    unsafe { psllid(a, imm8) }
}

/// Shift packed 32-bit integers in `a` left by `count` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sll_epi32(a: i32x4, count: i32x4) -> i32x4 {
    unsafe { pslld(a, count) }
}

/// Shift packed 64-bit integers in `a` left by `imm8` while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_slli_epi64(a: i64x2, imm8: i32) -> i64x2 {
    unsafe { pslliq(a, imm8) }
}

/// Shift packed 64-bit integers in `a` left by `count` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sll_epi64(a: i64x2, count: i64x2) -> i64x2 {
    unsafe { psllq(a, count) }
}

/// Shift packed 16-bit integers in `a` right by `imm8` while shifting in sign
/// bits.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srai_epi16(a: i16x8, imm8: i32) -> i16x8 {
    unsafe { psraiw(a, imm8) }
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in sign
/// bits.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sra_epi16(a: i16x8, count: i16x8) -> i16x8 {
    unsafe { psraw(a, count) }
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in sign
/// bits.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srai_epi32(a: i32x4, imm8: i32) -> i32x4 {
    unsafe { psraid(a, imm8) }
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in sign
/// bits.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sra_epi32(a: i32x4, count: i32x4) -> i32x4 {
    unsafe { psrad(a, count) }
}

/// Shift `a` right by `imm8` bytes while shifting in zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srli_si128(a: __m128i, imm8: i32) -> __m128i {
    let (zero, imm8) = (__m128i::splat(0), imm8 as u32);
    const fn add(a: u32, b: u32) -> u32 { a + b }
    macro_rules! shuffle {
        ($shift:expr) => {
            unsafe {
                simd_shuffle16::<__m128i, __m128i>(a, zero, [
                    add(0, $shift), add(1, $shift),
                    add(2, $shift), add(3, $shift),
                    add(4, $shift), add(5, $shift),
                    add(6, $shift), add(7, $shift),
                    add(8, $shift), add(9, $shift),
                    add(10, $shift), add(11, $shift),
                    add(12, $shift), add(13, $shift),
                    add(14, $shift), add(15, $shift),
                ])
            }
        }
    }
    match imm8 {
        0 => shuffle!(0), 1 => shuffle!(1),
        2 => shuffle!(2), 3 => shuffle!(3),
        4 => shuffle!(4), 5 => shuffle!(5),
        6 => shuffle!(6), 7 => shuffle!(7),
        8 => shuffle!(8), 9 => shuffle!(9),
        10 => shuffle!(10), 11 => shuffle!(11),
        12 => shuffle!(12), 13 => shuffle!(13),
        14 => shuffle!(14), 15 => shuffle!(15),
        _ => shuffle!(16),
    }
}

/// Shift packed 16-bit integers in `a` right by `imm8` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srli_epi16(a: i16x8, imm8: i32) -> i16x8  {
    unsafe { psrliw(a, imm8) }
}

/// Shift packed 16-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srl_epi16(a: i16x8, count: i16x8) -> i16x8 {
    unsafe { psrlw(a, count) }
}

/// Shift packed 32-bit integers in `a` right by `imm8` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srli_epi32(a: i32x4, imm8: i32) -> i32x4 {
    unsafe { psrlid(a, imm8) }
}

/// Shift packed 32-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srl_epi32(a: i32x4, count: i32x4) -> i32x4 {
    unsafe { psrld(a, count) }
}

/// Shift packed 64-bit integers in `a` right by `imm8` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srli_epi64(a: i64x2, imm8: i32) -> i64x2 {
    unsafe { psrliq(a, imm8) }
}

/// Shift packed 64-bit integers in `a` right by `count` while shifting in
/// zeros.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_srl_epi64(a: i64x2, count: i64x2) -> i64x2 {
    unsafe { psrlq(a, count) }
}

/// Compute the bitwise AND of 128 bits (representing integer data) in `a` and
/// `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_and_si128(a: __m128i, b: __m128i) -> __m128i {
    a & b
}

/// Compute the bitwise NOT of 128 bits (representing integer data) in `a` and
/// then AND with `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_andnot_si128(a: __m128i, b: __m128i) -> __m128i {
    (!a) & b
}

/// Compute the bitwise OR of 128 bits (representing integer data) in `a` and
/// `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_or_si128(a: __m128i, b: __m128i) -> __m128i {
    a | b
}

/// Compute the bitwise XOR of 128 bits (representing integer data) in `a` and
/// `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_xor_si128(a: __m128i, b: __m128i) -> __m128i {
    a ^ b
}

/// Compare packed 8-bit integers in `a` and `b` for equality.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpeq_epi8(a: i8x16, b: i8x16) -> i8x16 {
    a.eq(b)
}

/// Compare packed 16-bit integers in `a` and `b` for equality.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpeq_epi16(a: i16x8, b: i16x8) -> i16x8 {
    a.eq(b)
}

/// Compare packed 32-bit integers in `a` and `b` for equality.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpeq_epi32(a: i32x4, b: i32x4) -> i32x4 {
    a.eq(b)
}

/// Compare packed 8-bit integers in `a` and `b` for greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpgt_epi8(a: i8x16, b: i8x16) -> i8x16 {
    a.gt(b)
}

/// Compare packed 16-bit integers in `a` and `b` for greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpgt_epi16(a: i16x8, b: i16x8) -> i16x8 {
    a.gt(b)
}

/// Compare packed 32-bit integers in `a` and `b` for greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpgt_epi32(a: i32x4, b: i32x4) -> i32x4 {
    a.gt(b)
}

/// Compare packed 8-bit integers in `a` and `b` for less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmplt_epi8(a: i8x16, b: i8x16) -> i8x16 {
    a.lt(b)
}

/// Compare packed 16-bit integers in `a` and `b` for less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmplt_epi16(a: i16x8, b: i16x8) -> i16x8 {
    a.lt(b)
}

/// Compare packed 32-bit integers in `a` and `b` for less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmplt_epi32(a: i32x4, b: i32x4) -> i32x4 {
    a.lt(b)
}

/// Convert the lower two packed 32-bit integers in `a` to packed
/// double-precision (64-bit) floating-point elements.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtepi32_pd(a: i32x4) -> f64x2  {
    unsafe { simd_cast::<i32x2, f64x2>(simd_shuffle2(a, a, [0, 1])) }
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi32_sd(a: f64x2, b: i32) -> f64x2 {
    a.replace(0, b as f64)
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi64_sd(a: f64x2, b: i64) -> f64x2 {
    a.replace(0, b as f64)
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi64x_sd(a: f64x2, b: i64) -> f64x2 {
    _mm_cvtsi64_sd(a, b)
}

/// Convert packed 32-bit integers in `a` to packed single-precision (32-bit)
/// floating-point elements.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtepi32_ps(a: i32x4) -> f32x4 {
    unsafe { cvtdq2ps(a) }
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi32_si128(a: i32) -> i32x4 {
    i32x4::new(a, 0, 0, 0)
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi64_si128(a: i64) -> i64x2 {
    i64x2::new(a, 0)
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi64x_si128(a: i64) -> i64x2 {
    _mm_cvtsi64_si128(a)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi128_si32(a: i32x4) -> i32 {
    a.extract(0)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi128_si64(a: i64x2) -> i64 {
    a.extract(0)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cvtsi128_si64x(a: i64x2) -> i64 {
    _mm_cvtsi128_si64(a)
}

/// Set packed 64-bit integers with the supplied values, from highest to
/// lowest.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set_epi64x(e1: i64, e0: i64) -> i64x2 {
    i64x2::new(e0, e1)
}

/// Set packed 32-bit integers with the supplied values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
    i32x4::new(e0, e1, e2, e3)
}

/// Set packed 16-bit integers with the supplied values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set_epi16(
    e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16,
) -> i16x8 {
    i16x8::new(e0, e1, e2, e3, e4, e5, e6, e7)
}

/// Set packed 8-bit integers with the supplied values.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set_epi8(
    e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8,
    e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8,
) -> i8x16 {
    i8x16::new(
        e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15,
    )
}

/// Broadcast 64-bit integer `a` to all elements.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set1_epi64x(a: i64) -> i64x2 {
    i64x2::splat(a)
}

/// Broadcast 32-bit integer `a` to all elements.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set1_epi32(a: i32) -> i32x4 {
    i32x4::splat(a)
}

/// Broadcast 16-bit integer `a` to all elements.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set1_epi16(a: i16) -> i16x8 {
    i16x8::splat(a)
}

/// Broadcast 8-bit integer `a` to all elements.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_set1_epi8(a: i8) -> i8x16 {
    i8x16::splat(a)
}

/// Set packed 32-bit integers with the supplied values in reverse order.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_setr_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
    i32x4::new(e3, e2, e1, e0)
}

/// Set packed 16-bit integers with the supplied values in reverse order.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_setr_epi16(
    e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16,
) -> i16x8 {
    i16x8::new(e7, e6, e5, e4, e3, e2, e1, e0)
}

/// Set packed 8-bit integers with the supplied values in reverse order.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_setr_epi8(
    e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8,
    e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8,
) -> i8x16 {
    i8x16::new(
        e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
    )
}

/// Returns a vector with all elements set to zero.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_setzero_si128() -> __m128i {
    __m128i::splat(0)
}

/// Load 64-bit integer from memory into first element of returned vector.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_loadl_epi64(mem_addr: *const i64x2) -> i64x2 {
    i64x2::new((*mem_addr).extract(0), 0)
}

/// Load 128-bits of integer data from memory into a new vector.
///
/// `mem_addr` must be aligned on a 16-byte boundary.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_load_si128(mem_addr: *const __m128i) -> __m128i {
    *mem_addr
}

/// Load 128-bits of integer data from memory into a new vector.
///
/// `mem_addr` does not need to be aligned on any particular boundary.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_loadu_si128(mem_addr: *const __m128i) -> __m128i {
    let mut dst = mem::uninitialized();
    ptr::copy_nonoverlapping(
        mem_addr as *const u8,
        &mut dst as *mut __m128i as *mut u8,
        mem::size_of::<__m128i>());
    dst
}

/// Conditionally store 8-bit integer elements from `a` into memory using
/// `mask`.
///
/// Elements are not stored when the highest bit is not set in the
/// corresponding element.
///
/// `mem_addr` should correspond to a 128-bit memory location and does not need
/// to be aligned on any particular boundary.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_maskmoveu_si128(a: i8x16, mask: i8x16, mem_addr: *mut i8) {
    maskmovdqu(a, mask, mem_addr)
}

/// Store 128-bits of integer data from `a` into memory.
///
/// `mem_addr` must be aligned on a 16-byte boundary.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_store_si128(mem_addr: *mut __m128i, a: __m128i) {
    *mem_addr = a;
}

/// Store 128-bits of integer data from `a` into memory.
///
/// `mem_addr` does not need to be aligned on any particular boundary.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_storeu_si128(mem_addr: *mut __m128i, a: __m128i) {
    ptr::copy_nonoverlapping(
        &a as *const _ as *const u8,
        mem_addr as *mut u8,
        mem::size_of::<__m128i>());
}

/// Store the lower 64-bit integer `a` to a memory location.
///
/// `mem_addr` does not need to be aligned on any particular boundary.
#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_storel_epi64(mem_addr: *mut __m128i, a: __m128i) {
    ptr::copy_nonoverlapping(
        &a as *const _ as *const u8, mem_addr as *mut u8, 8);
}

/// Return a vector where the low element is extracted from `a` and its upper
/// element is zero.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_move_epi64(a: i64x2) -> i64x2 {
    a.replace(1, 0)
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using signed saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_packs_epi16(a: i16x8, b: i16x8) -> i8x16 {
    unsafe { packsswb(a, b) }
}

/// Convert packed 32-bit integers from `a` and `b` to packed 16-bit integers
/// using signed saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_packs_epi32(a: i32x4, b: i32x4) -> i16x8 {
    unsafe { packssdw(a, b) }
}

/// Convert packed 16-bit integers from `a` and `b` to packed 8-bit integers
/// using unsigned saturation.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_packus_epi16(a: i16x8, b: i16x8) -> u8x16 {
    unsafe { packuswb(a, b) }
}

/// Return the `imm8` element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_extract_epi16(a: i16x8, imm8: i32) -> i32 {
    a.extract(imm8 as u32 & 0b111) as i32
}

/// Return a new vector where the `imm8` element of `a` is replaced with `i`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_insert_epi16(a: i16x8, i: i32, imm8: i32) -> i16x8 {
    a.replace(imm8 as u32 & 0b111, i as i16)
}

/// Return a mask of the most significant bit of each element in `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_movemask_epi8(a: i8x16) -> i32 {
    unsafe { pmovmskb(a) }
}

/// Shuffle 32-bit integers in `a` using the control in `imm8`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_shuffle_epi32(a: i32x4, imm8: i32) -> i32x4 {
    // simd_shuffleX requires that its selector parameter be made up of
    // constant values, but we can't enforce that here. In spirit, we need
    // to write a `match` on all possible values of a byte, and for each value,
    // hard-code the correct `simd_shuffleX` call using only constants. We
    // then hope for LLVM to do the rest.
    //
    // Of course, that's... awful. So we try to use macros to do it for us.
    let imm8 = (imm8 & 0xFF) as u8;

    macro_rules! shuffle_done {
        ($x01:expr, $x23:expr, $x45:expr, $x67:expr) => {
            unsafe {
                simd_shuffle4(a, a, [$x01, $x23, $x45, $x67])
            }
        }
    }
    macro_rules! shuffle_x67 {
        ($x01:expr, $x23:expr, $x45:expr) => {
            match (imm8 >> 6) & 0b11 {
                0b00 => shuffle_done!($x01, $x23, $x45, 0),
                0b01 => shuffle_done!($x01, $x23, $x45, 1),
                0b10 => shuffle_done!($x01, $x23, $x45, 2),
                _ => shuffle_done!($x01, $x23, $x45, 3),
            }
        }
    }
    macro_rules! shuffle_x45 {
        ($x01:expr, $x23:expr) => {
            match (imm8 >> 4) & 0b11 {
                0b00 => shuffle_x67!($x01, $x23, 0),
                0b01 => shuffle_x67!($x01, $x23, 1),
                0b10 => shuffle_x67!($x01, $x23, 2),
                _ => shuffle_x67!($x01, $x23, 3),
            }
        }
    }
    macro_rules! shuffle_x23 {
        ($x01:expr) => {
            match (imm8 >> 2) & 0b11 {
                0b00 => shuffle_x45!($x01, 0),
                0b01 => shuffle_x45!($x01, 1),
                0b10 => shuffle_x45!($x01, 2),
                _ => shuffle_x45!($x01, 3),
            }
        }
    }
    match imm8 & 0b11 {
        0b00 => shuffle_x23!(0),
        0b01 => shuffle_x23!(1),
        0b10 => shuffle_x23!(2),
        _ => shuffle_x23!(3),
    }
}

/// Shuffle 16-bit integers in the high 64 bits of `a` using the control in
/// `imm8`.
///
/// Put the results in the high 64 bits of the returned vector, with the low 64
/// bits being copied from from `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_shufflehi_epi16(a: i16x8, imm8: i32) -> i16x8 {
    // See _mm_shuffle_epi32.
    let imm8 = (imm8 & 0xFF) as u8;
    const fn add4(x: u32) -> u32 { x + 4 }

    macro_rules! shuffle_done {
        ($x01:expr, $x23:expr, $x45:expr, $x67:expr) => {
            unsafe {
                simd_shuffle8(a, a, [
                    0, 1, 2, 3, add4($x01), add4($x23), add4($x45), add4($x67),
                ])
            }
        }
    }
    macro_rules! shuffle_x67 {
        ($x01:expr, $x23:expr, $x45:expr) => {
            match (imm8 >> 6) & 0b11 {
                0b00 => shuffle_done!($x01, $x23, $x45, 0),
                0b01 => shuffle_done!($x01, $x23, $x45, 1),
                0b10 => shuffle_done!($x01, $x23, $x45, 2),
                _ => shuffle_done!($x01, $x23, $x45, 3),
            }
        }
    }
    macro_rules! shuffle_x45 {
        ($x01:expr, $x23:expr) => {
            match (imm8 >> 4) & 0b11 {
                0b00 => shuffle_x67!($x01, $x23, 0),
                0b01 => shuffle_x67!($x01, $x23, 1),
                0b10 => shuffle_x67!($x01, $x23, 2),
                _ => shuffle_x67!($x01, $x23, 3),
            }
        }
    }
    macro_rules! shuffle_x23 {
        ($x01:expr) => {
            match (imm8 >> 2) & 0b11 {
                0b00 => shuffle_x45!($x01, 0),
                0b01 => shuffle_x45!($x01, 1),
                0b10 => shuffle_x45!($x01, 2),
                _ => shuffle_x45!($x01, 3),
            }
        }
    }
    match imm8 & 0b11 {
        0b00 => shuffle_x23!(0),
        0b01 => shuffle_x23!(1),
        0b10 => shuffle_x23!(2),
        _ => shuffle_x23!(3),
    }
}

/// Shuffle 16-bit integers in the low 64 bits of `a` using the control in
/// `imm8`.
///
/// Put the results in the low 64 bits of the returned vector, with the high 64
/// bits being copied from from `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_shufflelo_epi16(a: i16x8, imm8: i32) -> i16x8 {
    // See _mm_shuffle_epi32.
    let imm8 = (imm8 & 0xFF) as u8;

    macro_rules! shuffle_done {
        ($x01:expr, $x23:expr, $x45:expr, $x67:expr) => {
            unsafe {
                simd_shuffle8(a, a, [$x01, $x23, $x45, $x67, 4, 5, 6, 7])
            }
        }
    }
    macro_rules! shuffle_x67 {
        ($x01:expr, $x23:expr, $x45:expr) => {
            match (imm8 >> 6) & 0b11 {
                0b00 => shuffle_done!($x01, $x23, $x45, 0),
                0b01 => shuffle_done!($x01, $x23, $x45, 1),
                0b10 => shuffle_done!($x01, $x23, $x45, 2),
                _ => shuffle_done!($x01, $x23, $x45, 3),
            }
        }
    }
    macro_rules! shuffle_x45 {
        ($x01:expr, $x23:expr) => {
            match (imm8 >> 4) & 0b11 {
                0b00 => shuffle_x67!($x01, $x23, 0),
                0b01 => shuffle_x67!($x01, $x23, 1),
                0b10 => shuffle_x67!($x01, $x23, 2),
                _ => shuffle_x67!($x01, $x23, 3),
            }
        }
    }
    macro_rules! shuffle_x23 {
        ($x01:expr) => {
            match (imm8 >> 2) & 0b11 {
                0b00 => shuffle_x45!($x01, 0),
                0b01 => shuffle_x45!($x01, 1),
                0b10 => shuffle_x45!($x01, 2),
                _ => shuffle_x45!($x01, 3),
            }
        }
    }
    match imm8 & 0b11 {
        0b00 => shuffle_x23!(0),
        0b01 => shuffle_x23!(1),
        0b10 => shuffle_x23!(2),
        _ => shuffle_x23!(3),
    }
}

/// Unpack and interleave 8-bit integers from the high half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpackhi_epi8(a: i8x16, b: i8x16) -> i8x16 {
    unsafe {
        simd_shuffle16(a, b, [
            8, 24, 9, 25, 10, 26, 11, 27,
            12, 28, 13, 29, 14, 30, 15, 31,
        ])
    }
}

/// Unpack and interleave 16-bit integers from the high half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpackhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { simd_shuffle8(a, b, [4, 12, 5, 13, 6, 14, 7, 15]) }
}

/// Unpack and interleave 32-bit integers from the high half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpackhi_epi32(a: i32x4, b: i32x4) -> i32x4 {
    unsafe { simd_shuffle4(a, b, [2, 6, 3, 7]) }
}

/// Unpack and interleave 64-bit integers from the high half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpackhi_epi64(a: i64x2, b: i64x2) -> i64x2 {
    unsafe { simd_shuffle2(a, b, [1, 3]) }
}

/// Unpack and interleave 8-bit integers from the low half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpacklo_epi8(a: i8x16, b: i8x16) -> i8x16 {
    unsafe {
        simd_shuffle16(a, b, [
            0, 16, 1, 17, 2, 18, 3, 19,
            4, 20, 5, 21, 6, 22, 7, 23,
        ])
    }
}

/// Unpack and interleave 16-bit integers from the low half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpacklo_epi16(a: i16x8, b: i16x8) -> i16x8 {
    unsafe { simd_shuffle8(a, b, [0, 8, 1, 9, 2, 10, 3, 11]) }
}

/// Unpack and interleave 32-bit integers from the low half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpacklo_epi32(a: i32x4, b: i32x4) -> i32x4 {
    unsafe { simd_shuffle4(a, b, [0, 4, 1, 5]) }
}

/// Unpack and interleave 64-bit integers from the low half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_unpacklo_epi64(a: i64x2, b: i64x2) -> i64x2 {
    unsafe { simd_shuffle2(a, b, [0, 2]) }
}

/// Return a new vector with the low element of `a` replaced by the sum of the
/// low elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_add_sd(a: f64x2, b: f64x2) -> f64x2 {
    a.replace(0, a.extract(0) + b.extract(0))
}

/// Add packed double-precision (64-bit) floating-point elements in `a` and
/// `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_add_pd(a: f64x2, b: f64x2) -> f64x2 {
    a + b
}

/// Return a new vector with the low element of `a` replaced by the result of
/// diving the lower element of `a` by the lower element of `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_div_sd(a: f64x2, b: f64x2) -> f64x2 {
    a.replace(0, a.extract(0) / b.extract(0))
}

/// Divide packed double-precision (64-bit) floating-point elements in `a` by
/// packed elements in `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_div_pd(a: f64x2, b: f64x2) -> f64x2 {
    a / b
}

/// Return a new vector with the low element of `a` replaced by the maximum
/// of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_max_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { maxsd(a, b) }
}

/// Return a new vector with the maximum values from corresponding elements in
/// `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_max_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { maxpd(a, b) }
}

/// Return a new vector with the low element of `a` replaced by the minimum
/// of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_min_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { minsd(a, b) }
}

/// Return a new vector with the minimum values from corresponding elements in
/// `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_min_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { minpd(a, b) }
}

/// Return a new vector with the low element of `a` replaced by multiplying the
/// low elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mul_sd(a: f64x2, b: f64x2) -> f64x2 {
    a.replace(0, a.extract(0) * b.extract(0))
}

/// Multiply packed double-precision (64-bit) floating-point elements in `a`
/// and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_mul_pd(a: f64x2, b: f64x2) -> f64x2 {
    a * b
}

/// Return a new vector with the low element of `a` replaced by the square
/// root of the lower element `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sqrt_sd(a: f64x2, b: f64x2) -> f64x2 {
    a.replace(0, unsafe { sqrtsd(b).extract(0) })
}

/// Return a new vector with the square root of each of the values in `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sqrt_pd(a: f64x2) -> f64x2 {
    unsafe { sqrtpd(a) }
}

/// Return a new vector with the low element of `a` replaced by subtracting the
/// low element by `b` from the low element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sub_sd(a: f64x2, b: f64x2) -> f64x2 {
    a.replace(0, a.extract(0) - b.extract(0))
}

/// Subtract packed double-precision (64-bit) floating-point elements in `b`
/// from `a`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_sub_pd(a: f64x2, b: f64x2) -> f64x2 {
    a - b
}

/// Compute the bitwise AND of packed double-precision (64-bit) floating-point
/// elements in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_and_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe {
        let a: i64x2 = mem::transmute(a);
        let b: i64x2 = mem::transmute(b);
        mem::transmute(a & b)
    }
}

/// Compute the bitwise NOT of `a` and then AND with `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_andnot_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe {
        let a: i64x2 = mem::transmute(a);
        let b: i64x2 = mem::transmute(b);
        mem::transmute((!a) & b)
    }
}

/// Compute the bitwise OR of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_or_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe {
        let a: i64x2 = mem::transmute(a);
        let b: i64x2 = mem::transmute(b);
        mem::transmute(a | b)
    }
}

/// Compute the bitwise OR of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_xor_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe {
        let a: i64x2 = mem::transmute(a);
        let b: i64x2 = mem::transmute(b);
        mem::transmute(a ^ b)
    }
}

/// Return a new vector with the low element of `a` replaced by the equality
/// comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpeq_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 0) }
}

/// Return a new vector with the low element of `a` replaced by the less-than
/// comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmplt_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 1) }
}

/// Return a new vector with the low element of `a` replaced by the
/// less-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmple_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 2) }
}

/// Return a new vector with the low element of `a` replaced by the
/// greater-than comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpgt_sd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmplt_sd(b, a).replace(1, a.extract(1))
}

/// Return a new vector with the low element of `a` replaced by the
/// greater-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpge_sd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmple_sd(b, a).replace(1, a.extract(1))
}

/// Return a new vector with the low element of `a` replaced by the result
/// of comparing both of the lower elements of `a` and `b` to `NaN`. If
/// neither are equal to `NaN` then `0xFFFFFFFFFFFFFFFF` is used and `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpord_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 7) }
}

/// Return a new vector with the low element of `a` replaced by the result of
/// comparing both of the lower elements of `a` and `b` to `NaN`. If either is
/// equal to `NaN` then `0xFFFFFFFFFFFFFFFF` is used and `0` otherwise.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpunord_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 3) }
}

/// Return a new vector with the low element of `a` replaced by the not-equal
/// comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpneq_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 4) }
}

/// Return a new vector with the low element of `a` replaced by the
/// not-less-than comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpnlt_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 5) }
}

/// Return a new vector with the low element of `a` replaced by the
/// not-less-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpnle_sd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmpsd(a, b, 6) }
}

/// Return a new vector with the low element of `a` replaced by the
/// not-greater-than comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpngt_sd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmpnlt_sd(b, a).replace(1, a.extract(1))
}

/// Return a new vector with the low element of `a` replaced by the
/// not-greater-than-or-equal comparison of the lower elements of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpnge_sd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmpnle_sd(b, a).replace(1, a.extract(1))
}

/// Compare corresponding elements in `a` and `b` for equality.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpeq_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 0) }
}

/// Compare corresponding elements in `a` and `b` for less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmplt_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 1) }
}

/// Compare corresponding elements in `a` and `b` for less-than-or-equal
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmple_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 2) }
}

/// Compare corresponding elements in `a` and `b` for greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpgt_pd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmplt_pd(b, a)
}

/// Compare corresponding elements in `a` and `b` for greater-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpge_pd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmple_pd(b, a)
}

/// Compare corresponding elements in `a` and `b` to see if neither is `NaN`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpord_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 7) }
}

/// Compare corresponding elements in `a` and `b` to see if either is `NaN`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpunord_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 3) }
}

/// Compare corresponding elements in `a` and `b` for not-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpneq_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 4) }
}

/// Compare corresponding elements in `a` and `b` for not-less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpnlt_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 5) }
}

/// Compare corresponding elements in `a` and `b` for not-less-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpnle_pd(a: f64x2, b: f64x2) -> f64x2 {
    unsafe { cmppd(a, b, 6) }
}

/// Compare corresponding elements in `a` and `b` for not-greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpngt_pd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmpnlt_pd(b, a)
}

/// Compare corresponding elements in `a` and `b` for
/// not-greater-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_cmpnge_pd(a: f64x2, b: f64x2) -> f64x2 {
    _mm_cmpnle_pd(b, a)
}

/// Compare the lower element of `a` and `b` for equality.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_comieq_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(comieqsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_comilt_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(comiltsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for less-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_comile_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(comilesd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_comigt_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(comigtsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for greater-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_comige_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(comigesd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for not-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_comineq_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(comineqsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for equality.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_ucomieq_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(ucomieqsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for less-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_ucomilt_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(ucomiltsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for less-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_ucomile_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(ucomilesd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for greater-than.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_ucomigt_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(ucomigtsd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for greater-than-or-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_ucomige_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(ucomigesd(a, b) as u8) }
}

/// Compare the lower element of `a` and `b` for not-equal.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_ucomineq_sd(a: f64x2, b: f64x2) -> bool {
    unsafe { mem::transmute(ucomineqsd(a, b) as u8) }
}

/// Return a mask of the most significant bit of each element in `a`.
///
/// The mask is stored in the 2 least significant bits of the return value.
/// All other bits are set to `0`.
#[inline(always)]
#[target_feature = "+sse2"]
pub fn _mm_movemask_pd(a: f64x2) -> i32 {
    unsafe { movmskpd(a) }
}




#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_load_pd(mem_addr: *const f64) -> f64x2 {
    *(mem_addr as *const f64x2)
}

#[inline(always)]
#[target_feature = "+sse2"]
pub unsafe fn _mm_store_pd(mem_addr: *mut f64, a: f64x2) {
    *(mem_addr as *mut f64x2) = a;
}

#[allow(improper_ctypes)]
extern {
    #[link_name = "llvm.x86.sse2.pause"]
    fn pause();
    #[link_name = "llvm.x86.sse2.clflush"]
    fn clflush(p: *mut c_void);
    #[link_name = "llvm.x86.sse2.lfence"]
    fn lfence();
    #[link_name = "llvm.x86.sse2.mfence"]
    fn mfence();
    #[link_name = "llvm.x86.sse2.padds.b"]
    fn paddsb(a: i8x16, b: i8x16) -> i8x16;
    #[link_name = "llvm.x86.sse2.padds.w"]
    fn paddsw(a: i16x8, b: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.paddus.b"]
    fn paddsub(a: u8x16, b: u8x16) -> u8x16;
    #[link_name = "llvm.x86.sse2.paddus.w"]
    fn paddsuw(a: u16x8, b: u16x8) -> u16x8;
    #[link_name = "llvm.x86.sse2.pavg.b"]
    fn pavgb(a: u8x16, b: u8x16) -> u8x16;
    #[link_name = "llvm.x86.sse2.pavg.w"]
    fn pavgw(a: u16x8, b: u16x8) -> u16x8;
    #[link_name = "llvm.x86.sse2.pmadd.wd"]
    fn pmaddwd(a: i16x8, b: i16x8) -> i32x4;
    #[link_name = "llvm.x86.sse2.pmaxs.w"]
    fn pmaxsw(a: i16x8, b: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.pmaxu.b"]
    fn pmaxub(a: u8x16, b: u8x16) -> u8x16;
    #[link_name = "llvm.x86.sse2.pmins.w"]
    fn pminsw(a: i16x8, b: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.pminu.b"]
    fn pminub(a: u8x16, b: u8x16) -> u8x16;
    #[link_name = "llvm.x86.sse2.pmulh.w"]
    fn pmulhw(a: i16x8, b: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.pmulhu.w"]
    fn pmulhuw(a: u16x8, b: u16x8) -> u16x8;
    #[link_name = "llvm.x86.sse2.pmulu.dq"]
    fn pmuludq(a: u32x4, b: u32x4) -> u64x2;
    #[link_name = "llvm.x86.sse2.psad.bw"]
    fn psadbw(a: u8x16, b: u8x16) -> u64x2;
    #[link_name = "llvm.x86.sse2.psubs.b"]
    fn psubsb(a: i8x16, b: i8x16) -> i8x16;
    #[link_name = "llvm.x86.sse2.psubs.w"]
    fn psubsw(a: i16x8, b: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.psubus.b"]
    fn psubusb(a: u8x16, b: u8x16) -> u8x16;
    #[link_name = "llvm.x86.sse2.psubus.w"]
    fn psubusw(a: u16x8, b: u16x8) -> u16x8;
    #[link_name = "llvm.x86.sse2.pslli.w"]
    fn pslliw(a: i16x8, imm8: i32) -> i16x8;
    #[link_name = "llvm.x86.sse2.psll.w"]
    fn psllw(a: i16x8, count: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.pslli.d"]
    fn psllid(a: i32x4, imm8: i32) -> i32x4;
    #[link_name = "llvm.x86.sse2.psll.d"]
    fn pslld(a: i32x4, count: i32x4) -> i32x4;
    #[link_name = "llvm.x86.sse2.pslli.q"]
    fn pslliq(a: i64x2, imm8: i32) -> i64x2;
    #[link_name = "llvm.x86.sse2.psll.q"]
    fn psllq(a: i64x2, count: i64x2) -> i64x2;
    #[link_name = "llvm.x86.sse2.psrai.w"]
    fn psraiw(a: i16x8, imm8: i32) -> i16x8;
    #[link_name = "llvm.x86.sse2.psra.w"]
    fn psraw(a: i16x8, count: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.psrai.d"]
    fn psraid(a: i32x4, imm8: i32) -> i32x4;
    #[link_name = "llvm.x86.sse2.psra.d"]
    fn psrad(a: i32x4, count: i32x4) -> i32x4;
    #[link_name = "llvm.x86.sse2.psrli.w"]
    fn psrliw(a: i16x8, imm8: i32) -> i16x8;
    #[link_name = "llvm.x86.sse2.psrl.w"]
    fn psrlw(a: i16x8, count: i16x8) -> i16x8;
    #[link_name = "llvm.x86.sse2.psrli.d"]
    fn psrlid(a: i32x4, imm8: i32) -> i32x4;
    #[link_name = "llvm.x86.sse2.psrl.d"]
    fn psrld(a: i32x4, count: i32x4) -> i32x4;
    #[link_name = "llvm.x86.sse2.psrli.q"]
    fn psrliq(a: i64x2, imm8: i32) -> i64x2;
    #[link_name = "llvm.x86.sse2.psrl.q"]
    fn psrlq(a: i64x2, count: i64x2) -> i64x2;
    #[link_name = "llvm.x86.sse2.cvtdq2ps"]
    fn cvtdq2ps(a: i32x4) -> f32x4;
    #[link_name = "llvm.x86.sse2.maskmov.dqu"]
    fn maskmovdqu(a: i8x16, mask: i8x16, mem_addr: *mut i8);
    #[link_name = "llvm.x86.sse2.packsswb.128"]
    fn packsswb(a: i16x8, b: i16x8) -> i8x16;
    #[link_name = "llvm.x86.sse2.packssdw.128"]
    fn packssdw(a: i32x4, b: i32x4) -> i16x8;
    #[link_name = "llvm.x86.sse2.packuswb.128"]
    fn packuswb(a: i16x8, b: i16x8) -> u8x16;
    #[link_name = "llvm.x86.sse2.pmovmskb.128"]
    fn pmovmskb(a: i8x16) -> i32;
    #[link_name = "llvm.x86.sse2.max.sd"]
    fn maxsd(a: f64x2, b: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse2.max.pd"]
    fn maxpd(a: f64x2, b: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse2.min.sd"]
    fn minsd(a: f64x2, b: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse2.min.pd"]
    fn minpd(a: f64x2, b: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse2.sqrt.sd"]
    fn sqrtsd(a: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse2.sqrt.pd"]
    fn sqrtpd(a: f64x2) -> f64x2;
    #[link_name = "llvm.x86.sse2.cmp.sd"]
    fn cmpsd(a: f64x2, b: f64x2, imm8: i8) -> f64x2;
    #[link_name = "llvm.x86.sse2.cmp.pd"]
    fn cmppd(a: f64x2, b: f64x2, imm8: i8) -> f64x2;
    #[link_name = "llvm.x86.sse2.comieq.sd"]
    fn comieqsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.comilt.sd"]
    fn comiltsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.comile.sd"]
    fn comilesd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.comigt.sd"]
    fn comigtsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.comige.sd"]
    fn comigesd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.comineq.sd"]
    fn comineqsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.ucomieq.sd"]
    fn ucomieqsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.ucomilt.sd"]
    fn ucomiltsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.ucomile.sd"]
    fn ucomilesd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.ucomigt.sd"]
    fn ucomigtsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.ucomige.sd"]
    fn ucomigesd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.ucomineq.sd"]
    fn ucomineqsd(a: f64x2, b: f64x2) -> i32;
    #[link_name = "llvm.x86.sse2.movmsk.pd"]
    fn movmskpd(a: f64x2) -> i32;
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;

    use v128::*;
    use x86::{__m128i, sse2};

    #[test]
    fn _mm_pause() {
        sse2::_mm_pause();
    }

    #[test]
    fn _mm_clflush() {
        let x = 0;
        unsafe { sse2::_mm_clflush(&x as *const _ as *mut c_void); }
    }

    #[test]
    fn _mm_lfence() {
        sse2::_mm_lfence();
    }

    #[test]
    fn _mm_mfence() {
        sse2::_mm_mfence();
    }

    #[test]
    fn _mm_add_epi8() {
        let a = i8x16::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let b = i8x16::new(
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let r = sse2::_mm_add_epi8(a, b);
        let e = i8x16::new(
            16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_add_epi8_overflow() {
        let a = i8x16::splat(0x7F);
        let b = i8x16::splat(1);
        let r = sse2::_mm_add_epi8(a, b);
        assert_eq!(r, i8x16::splat(-128));
    }

    #[test]
    fn _mm_add_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        let b = i16x8::new(8, 9, 10, 11, 12, 13, 14, 15);
        let r = sse2::_mm_add_epi16(a, b);
        let e = i16x8::new(8, 10, 12, 14, 16, 18, 20, 22);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_add_epi32() {
        let a = i32x4::new(0, 1, 2, 3);
        let b = i32x4::new(4, 5, 6, 7);
        let r = sse2::_mm_add_epi32(a, b);
        let e = i32x4::new(4, 6, 8, 10);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_add_epi64() {
        let a = i64x2::new(0, 1);
        let b = i64x2::new(2, 3);
        let r = sse2::_mm_add_epi64(a, b);
        let e = i64x2::new(2, 4);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_adds_epi8() {
        let a = i8x16::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let b = i8x16::new(
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let r = sse2::_mm_adds_epi8(a, b);
        let e = i8x16::new(
            16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_adds_epi8_saturate_positive() {
        let a = i8x16::splat(0x7F);
        let b = i8x16::splat(1);
        let r = sse2::_mm_adds_epi8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_adds_epi8_saturate_negative() {
        let a = i8x16::splat(-0x80);
        let b = i8x16::splat(-1);
        let r = sse2::_mm_adds_epi8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_adds_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        let b = i16x8::new(8, 9, 10, 11, 12, 13, 14, 15);
        let r = sse2::_mm_adds_epi16(a, b);
        let e = i16x8::new(8, 10, 12, 14, 16, 18, 20, 22);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_adds_epi16_saturate_positive() {
        let a = i16x8::splat(0x7FFF);
        let b = i16x8::splat(1);
        let r = sse2::_mm_adds_epi16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_adds_epi16_saturate_negative() {
        let a = i16x8::splat(-0x8000);
        let b = i16x8::splat(-1);
        let r = sse2::_mm_adds_epi16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_adds_epu8() {
        let a = u8x16::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let b = u8x16::new(
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let r = sse2::_mm_adds_epu8(a, b);
        let e = u8x16::new(
            16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_adds_epu8_saturate() {
        let a = u8x16::splat(0xFF);
        let b = u8x16::splat(1);
        let r = sse2::_mm_adds_epu8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_adds_epu16() {
        let a = u16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        let b = u16x8::new(8, 9, 10, 11, 12, 13, 14, 15);
        let r = sse2::_mm_adds_epu16(a, b);
        let e = u16x8::new(8, 10, 12, 14, 16, 18, 20, 22);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_adds_epu16_saturate() {
        let a = u16x8::splat(0xFFFF);
        let b = u16x8::splat(1);
        let r = sse2::_mm_adds_epu16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_avg_epu8() {
        let (a, b) = (u8x16::splat(3), u8x16::splat(9));
        let r = sse2::_mm_avg_epu8(a, b);
        assert_eq!(r, u8x16::splat(6));
    }

    #[test]
    fn _mm_avg_epu16() {
        let (a, b) = (u16x8::splat(3), u16x8::splat(9));
        let r = sse2::_mm_avg_epu16(a, b);
        assert_eq!(r, u16x8::splat(6));
    }

    #[test]
    fn _mm_madd_epi16() {
        let a = i16x8::new(1, 2, 3, 4, 5, 6, 7, 8);
        let b = i16x8::new(9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_madd_epi16(a, b);
        let e = i32x4::new(29, 81, 149, 233);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_max_epi16() {
        let a = i16x8::splat(1);
        let b = i16x8::splat(-1);
        let r = sse2::_mm_max_epi16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_max_epu8() {
        let a = u8x16::splat(1);
        let b = u8x16::splat(255);
        let r = sse2::_mm_max_epu8(a, b);
        assert_eq!(r, b);
    }

    #[test]
    fn _mm_min_epi16() {
        let a = i16x8::splat(1);
        let b = i16x8::splat(-1);
        let r = sse2::_mm_min_epi16(a, b);
        assert_eq!(r, b);
    }

    #[test]
    fn _mm_min_epu8() {
        let a = u8x16::splat(1);
        let b = u8x16::splat(255);
        let r = sse2::_mm_min_epu8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_mulhi_epi16() {
        let (a, b) = (i16x8::splat(1000), i16x8::splat(-1001));
        let r = sse2::_mm_mulhi_epi16(a, b);
        assert_eq!(r, i16x8::splat(-16));
    }

    #[test]
    fn _mm_mulhi_epu16() {
        let (a, b) = (u16x8::splat(1000), u16x8::splat(1001));
        let r = sse2::_mm_mulhi_epu16(a, b);
        assert_eq!(r, u16x8::splat(15));
    }

    #[test]
    fn _mm_mullo_epi16() {
        let (a, b) = (i16x8::splat(1000), i16x8::splat(-1001));
        let r = sse2::_mm_mullo_epi16(a, b);
        assert_eq!(r, i16x8::splat(-17960));
    }

    #[test]
    fn _mm_mul_epu32() {
        let a = u32x4::from(u64x2::new(1_000_000_000, 1 << 34));
        let b = u32x4::from(u64x2::new(1_000_000_000, 1 << 35));
        let r = sse2::_mm_mul_epu32(a, b);
        let e = u64x2::new(1_000_000_000 * 1_000_000_000, 0);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_sad_epu8() {
        let a = u8x16::new(
            255, 254, 253, 252, 1, 2, 3, 4,
            155, 154, 153, 152, 1, 2, 3, 4);
        let b = u8x16::new(
            0, 0, 0, 0, 2, 1, 2, 1,
            1, 1, 1, 1, 1, 2, 1, 2);
        let r = sse2::_mm_sad_epu8(a, b);
        let e = u64x2::new(1020, 614);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_sub_epi8() {
        let (a, b) = (i8x16::splat(5), i8x16::splat(6));
        let r = sse2::_mm_sub_epi8(a, b);
        assert_eq!(r, i8x16::splat(-1));
    }

    #[test]
    fn _mm_sub_epi16() {
        let (a, b) = (i16x8::splat(5), i16x8::splat(6));
        let r = sse2::_mm_sub_epi16(a, b);
        assert_eq!(r, i16x8::splat(-1));
    }

    #[test]
    fn _mm_sub_epi32() {
        let (a, b) = (i32x4::splat(5), i32x4::splat(6));
        let r = sse2::_mm_sub_epi32(a, b);
        assert_eq!(r, i32x4::splat(-1));
    }

    #[test]
    fn _mm_sub_epi64() {
        let (a, b) = (i64x2::splat(5), i64x2::splat(6));
        let r = sse2::_mm_sub_epi64(a, b);
        assert_eq!(r, i64x2::splat(-1));
    }

    #[test]
    fn _mm_subs_epi8() {
        let (a, b) = (i8x16::splat(5), i8x16::splat(2));
        let r = sse2::_mm_subs_epi8(a, b);
        assert_eq!(r, i8x16::splat(3));
    }

    #[test]
    fn _mm_subs_epi8_saturate_positive() {
        let a = i8x16::splat(0x7F);
        let b = i8x16::splat(-1);
        let r = sse2::_mm_subs_epi8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_subs_epi8_saturate_negative() {
        let a = i8x16::splat(-0x80);
        let b = i8x16::splat(1);
        let r = sse2::_mm_subs_epi8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_subs_epi16() {
        let (a, b) = (i16x8::splat(5), i16x8::splat(2));
        let r = sse2::_mm_subs_epi16(a, b);
        assert_eq!(r, i16x8::splat(3));
    }

    #[test]
    fn _mm_subs_epi16_saturate_positive() {
        let a = i16x8::splat(0x7FFF);
        let b = i16x8::splat(-1);
        let r = sse2::_mm_subs_epi16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_subs_epi16_saturate_negative() {
        let a = i16x8::splat(-0x8000);
        let b = i16x8::splat(1);
        let r = sse2::_mm_subs_epi16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_subs_epu8() {
        let (a, b) = (u8x16::splat(5), u8x16::splat(2));
        let r = sse2::_mm_subs_epu8(a, b);
        assert_eq!(r, u8x16::splat(3));
    }

    #[test]
    fn _mm_subs_epu8_saturate() {
        let a = u8x16::splat(0);
        let b = u8x16::splat(1);
        let r = sse2::_mm_subs_epu8(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_subs_epu16() {
        let (a, b) = (u16x8::splat(5), u16x8::splat(2));
        let r = sse2::_mm_subs_epu16(a, b);
        assert_eq!(r, u16x8::splat(3));
    }

    #[test]
    fn _mm_subs_epu16_saturate() {
        let a = u16x8::splat(0);
        let b = u16x8::splat(1);
        let r = sse2::_mm_subs_epu16(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_slli_si128() {
        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_slli_si128(a, 1);
        let e = __m128i::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        assert_eq!(r, e);

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_slli_si128(a, 15);
        let e = __m128i::new(
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1);
        assert_eq!(r, e);

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_slli_si128(a, 16);
        assert_eq!(r, __m128i::splat(0));

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_slli_si128(a, -1);
        assert_eq!(r, __m128i::splat(0));

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_slli_si128(a, -0x80000000);
        assert_eq!(r, __m128i::splat(0));
    }

    #[test]
    fn _mm_slli_epi16() {
        let a = i16x8::new(
            0xFFFF as u16 as i16, 0x0FFF, 0x00FF, 0x000F, 0, 0, 0, 0);
        let r = sse2::_mm_slli_epi16(a, 4);
        let e = i16x8::new(
            0xFFF0 as u16 as i16,
            0xFFF0 as u16 as i16, 0x0FF0, 0x00F0, 0, 0, 0, 0);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_sll_epi16() {
        let a = i16x8::new(0xFF, 0, 0, 0, 0, 0, 0, 0);
        let r = sse2::_mm_sll_epi16(a, i16x8::new(4, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(r, i16x8::new(0xFF0, 0, 0, 0, 0, 0, 0, 0));
        let r = sse2::_mm_sll_epi16(a, i16x8::new(0, 0, 0, 0, 4, 0, 0, 0));
        assert_eq!(r, i16x8::new(0xFF, 0, 0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn _mm_slli_epi32() {
        assert_eq!(
            sse2::_mm_slli_epi32(i32x4::splat(0xFFFF), 4),
            i32x4::splat(0xFFFF0));
    }

    #[test]
    fn _mm_sll_epi32() {
        assert_eq!(
            sse2::_mm_sll_epi32(i32x4::splat(0xFFFF), i32x4::new(4, 0, 0, 0)),
            i32x4::splat(0xFFFF0));
    }

    #[test]
    fn _mm_slli_epi64() {
        assert_eq!(
            sse2::_mm_slli_epi64(i64x2::splat(0xFFFFFFFF), 4),
            i64x2::splat(0xFFFFFFFF0));
    }

    #[test]
    fn _mm_sll_epi64() {
        assert_eq!(
            sse2::_mm_sll_epi64(
                i64x2::splat(0xFFFFFFFF), i64x2::new(4, 0)),
            i64x2::splat(0xFFFFFFFF0));
    }

    #[test]
    fn _mm_srai_epi16() {
        assert_eq!(
            sse2::_mm_srai_epi16(i16x8::splat(-1), 1), i16x8::splat(-1));
    }

    #[test]
    fn _mm_sra_epi16() {
        assert_eq!(
            sse2::_mm_sra_epi16(
                i16x8::splat(-1), i16x8::new(1, 0, 0, 0, 0, 0, 0, 0)),
            i16x8::splat(-1));
    }

    #[test]
    fn _mm_srai_epi32() {
        assert_eq!(
            sse2::_mm_srai_epi32(i32x4::splat(-1), 1), i32x4::splat(-1));
    }

    #[test]
    fn _mm_sra_epi32() {
        assert_eq!(
            sse2::_mm_sra_epi32(
                i32x4::splat(-1), i32x4::new(1, 0, 0, 0)),
            i32x4::splat(-1));
    }

    #[test]
    fn _mm_srli_si128() {
        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_srli_si128(a, 1);
        let e = __m128i::new(
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0);
        assert_eq!(r, e);

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_srli_si128(a, 15);
        let e = __m128i::new(
            16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        assert_eq!(r, e);

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_srli_si128(a, 16);
        assert_eq!(r, __m128i::splat(0));

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_srli_si128(a, -1);
        assert_eq!(r, __m128i::splat(0));

        let a = __m128i::new(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        let r = sse2::_mm_srli_si128(a, -0x80000000);
        assert_eq!(r, __m128i::splat(0));
    }

    #[test]
    fn _mm_srli_epi16() {
        let a = i16x8::new(
            0xFFFF as u16 as i16, 0x0FFF, 0x00FF, 0x000F, 0, 0, 0, 0);
        let r = sse2::_mm_srli_epi16(a, 4);
        let e = i16x8::new(
            0xFFF as u16 as i16,
            0xFF as u16 as i16, 0xF, 0, 0, 0, 0, 0);
        assert_eq!(r, e);
    }

    #[test]
    fn _mm_srl_epi16() {
        let a = i16x8::new(0xFF, 0, 0, 0, 0, 0, 0, 0);
        let r = sse2::_mm_srl_epi16(a, i16x8::new(4, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(r, i16x8::new(0xF, 0, 0, 0, 0, 0, 0, 0));
        let r = sse2::_mm_srl_epi16(a, i16x8::new(0, 0, 0, 0, 4, 0, 0, 0));
        assert_eq!(r, i16x8::new(0xFF, 0, 0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn _mm_srli_epi32() {
        assert_eq!(
            sse2::_mm_srli_epi32(i32x4::splat(0xFFFF), 4),
            i32x4::splat(0xFFF));
    }

    #[test]
    fn _mm_srl_epi32() {
        assert_eq!(
            sse2::_mm_srl_epi32(i32x4::splat(0xFFFF), i32x4::new(4, 0, 0, 0)),
            i32x4::splat(0xFFF));
    }

    #[test]
    fn _mm_srli_epi64() {
        assert_eq!(
            sse2::_mm_srli_epi64(i64x2::splat(0xFFFFFFFF), 4),
            i64x2::splat(0xFFFFFFF));
    }

    #[test]
    fn _mm_srl_epi64() {
        assert_eq!(
            sse2::_mm_srl_epi64(
                i64x2::splat(0xFFFFFFFF), i64x2::new(4, 0)),
            i64x2::splat(0xFFFFFFF));
    }

    #[test]
    fn _mm_and_si128() {
        assert_eq!(
            sse2::_mm_and_si128(__m128i::splat(5), __m128i::splat(3)),
            __m128i::splat(1));
    }

    #[test]
    fn _mm_andnot_si128() {
        assert_eq!(
            sse2::_mm_andnot_si128(__m128i::splat(5), __m128i::splat(3)),
            __m128i::splat(2));
    }

    #[test]
    fn _mm_or_si128() {
        assert_eq!(
            sse2::_mm_or_si128(__m128i::splat(5), __m128i::splat(3)),
            __m128i::splat(7));
    }

    #[test]
    fn _mm_xor_si128() {
        assert_eq!(
            sse2::_mm_xor_si128(__m128i::splat(5), __m128i::splat(3)),
            __m128i::splat(6));
    }

    #[test]
    fn _mm_cmpeq_epi8() {
        let a = i8x16::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let b = i8x16::new(
            15, 14, 2, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);
        let r = sse2::_mm_cmpeq_epi8(a, b);
        assert_eq!(r, i8x16::new(
            0, 0, 0xFFu8 as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn _mm_cmpeq_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        let b = i16x8::new(7, 6, 2, 4, 3, 2, 1, 0);
        let r = sse2::_mm_cmpeq_epi16(a, b);
        assert_eq!(r, i16x8::splat(0).replace(2, 0xFFFFu16 as i16));
    }

    #[test]
    fn _mm_cmpeq_epi32() {
        let a = i32x4::new(0, 1, 2, 3);
        let b = i32x4::new(3, 2, 2, 0);
        let r = sse2::_mm_cmpeq_epi32(a, b);
        assert_eq!(r, i32x4::splat(0).replace(2, 0xFFFFFFFFu32 as i32));
    }

    #[test]
    fn _mm_cmpgt_epi8() {
        let a = i8x16::splat(0).replace(0, 5);
        let b = i8x16::splat(0);
        let r = sse2::_mm_cmpgt_epi8(a, b);
        assert_eq!(r, i8x16::splat(0).replace(0, 0xFFu8 as i8));
    }

    #[test]
    fn _mm_cmpgt_epi16() {
        let a = i16x8::splat(0).replace(0, 5);
        let b = i16x8::splat(0);
        let r = sse2::_mm_cmpgt_epi16(a, b);
        assert_eq!(r, i16x8::splat(0).replace(0, 0xFFFFu16 as i16));
    }

    #[test]
    fn _mm_cmpgt_epi32() {
        let a = i32x4::splat(0).replace(0, 5);
        let b = i32x4::splat(0);
        let r = sse2::_mm_cmpgt_epi32(a, b);
        assert_eq!(r, i32x4::splat(0).replace(0, 0xFFFFFFFFu32 as i32));
    }

    #[test]
    fn _mm_cmplt_epi8() {
        let a = i8x16::splat(0);
        let b = i8x16::splat(0).replace(0, 5);
        let r = sse2::_mm_cmplt_epi8(a, b);
        assert_eq!(r, i8x16::splat(0).replace(0, 0xFFu8 as i8));
    }

    #[test]
    fn _mm_cmplt_epi16() {
        let a = i16x8::splat(0);
        let b = i16x8::splat(0).replace(0, 5);
        let r = sse2::_mm_cmplt_epi16(a, b);
        assert_eq!(r, i16x8::splat(0).replace(0, 0xFFFFu16 as i16));
    }

    #[test]
    fn _mm_cmplt_epi32() {
        let a = i32x4::splat(0);
        let b = i32x4::splat(0).replace(0, 5);
        let r = sse2::_mm_cmplt_epi32(a, b);
        assert_eq!(r, i32x4::splat(0).replace(0, 0xFFFFFFFFu32 as i32));
    }

    #[test]
    fn _mm_cvtepi32_pd() {
        let a = sse2::_mm_set_epi32(35, 25, 15, 5);
        let r = sse2::_mm_cvtepi32_pd(a);
        assert_eq!(r, f64x2::new(5.0, 15.0));
    }

    #[test]
    fn _mm_cvtsi32_sd() {
        let a = f64x2::splat(3.5);
        assert_eq!(sse2::_mm_cvtsi32_sd(a, 5), f64x2::new(5.0, 3.5));
    }

    #[test]
    fn _mm_cvtsi64_sd() {
        let a = f64x2::splat(3.5);
        assert_eq!(sse2::_mm_cvtsi64_sd(a, 5), f64x2::new(5.0, 3.5));
    }

    #[test]
    fn _mm_cvtepi32_ps() {
        let a = i32x4::new(1, 2, 3, 4);
        assert_eq!(sse2::_mm_cvtepi32_ps(a), f32x4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn _mm_cvtsi32_si128() {
        assert_eq!(sse2::_mm_cvtsi32_si128(5), i32x4::new(5, 0, 0, 0));
    }

    #[test]
    fn _mm_cvtsi64_si128() {
        assert_eq!(sse2::_mm_cvtsi64_si128(5), i64x2::new(5, 0));
    }

    #[test]
    fn _mm_cvtsi128_si32() {
        assert_eq!(sse2::_mm_cvtsi128_si32(i32x4::new(5, 0, 0, 0)), 5);
    }

    #[test]
    fn _mm_cvtsi128_si64() {
        assert_eq!(sse2::_mm_cvtsi128_si64(i64x2::new(5, 0)), 5);
    }

    #[test]
    fn _mm_set_epi64x() {
        assert_eq!(sse2::_mm_set_epi64x(0, 1), i64x2::new(1, 0));
    }

    #[test]
    fn _mm_set_epi32() {
        assert_eq!(sse2::_mm_set_epi32(0, 1, 2, 3), i32x4::new(3, 2, 1, 0));
    }

    #[test]
    fn _mm_set_epi16() {
        assert_eq!(
            sse2::_mm_set_epi16(0, 1, 2, 3, 4, 5, 6, 7),
            i16x8::new(7, 6, 5, 4, 3, 2, 1, 0));
    }

    #[test]
    fn _mm_set_epi8() {
        assert_eq!(
            sse2::_mm_set_epi8(
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
            i8x16::new(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0));
    }

    #[test]
    fn _mm_set1_epi64x() {
        assert_eq!(sse2::_mm_set1_epi64x(1), i64x2::splat(1));
    }

    #[test]
    fn _mm_set1_epi32() {
        assert_eq!(sse2::_mm_set1_epi32(1), i32x4::splat(1));
    }

    #[test]
    fn _mm_set1_epi16() {
        assert_eq!(sse2::_mm_set1_epi16(1), i16x8::splat(1));
    }

    #[test]
    fn _mm_set1_epi8() {
        assert_eq!(sse2::_mm_set1_epi8(1), i8x16::splat(1));
    }

    #[test]
    fn _mm_setr_epi32() {
        assert_eq!(sse2::_mm_setr_epi32(0, 1, 2, 3), i32x4::new(0, 1, 2, 3));
    }

    #[test]
    fn _mm_setr_epi16() {
        assert_eq!(
            sse2::_mm_setr_epi16(0, 1, 2, 3, 4, 5, 6, 7),
            i16x8::new(0, 1, 2, 3, 4, 5, 6, 7));
    }

    #[test]
    fn _mm_setr_epi8() {
        assert_eq!(
            sse2::_mm_setr_epi8(
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
            i8x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
    }

    #[test]
    fn _mm_setzero_si128() {
        assert_eq!(sse2::_mm_setzero_si128(), __m128i::from(i64x2::splat(0)));
    }

    #[test]
    fn _mm_loadl_epi64() {
        let a = i64x2::new(6, 5);
        let r = unsafe { sse2::_mm_loadl_epi64(&a as *const _) };
        assert_eq!(r, i64x2::new(6, 0));
    }

    #[test]
    fn _mm_load_si128() {
        let a = sse2::_mm_set_epi64x(5, 6);
        let r = unsafe { sse2::_mm_load_si128(&a as *const _ as *const _) };
        assert_eq!(a, i64x2::from(r));
    }

    #[test]
    fn _mm_loadu_si128() {
        let a = sse2::_mm_set_epi64x(5, 6);
        let r = unsafe { sse2::_mm_loadu_si128(&a as *const _ as *const _) };
        assert_eq!(a, i64x2::from(r));
    }

    #[test]
    fn _mm_maskmoveu_si128() {
        let a = i8x16::splat(9);
        let mask = i8x16::splat(0).replace(2, 0x80u8 as i8);
        let mut r = i8x16::splat(0);
        unsafe {
            sse2::_mm_maskmoveu_si128(a, mask, &mut r as *mut _ as *mut i8);
        }
        assert_eq!(r, i8x16::splat(0).replace(2, 9));
    }

    #[test]
    fn _mm_store_si128() {
        let a = __m128i::splat(9);
        let mut r = __m128i::splat(0);
        unsafe {
            sse2::_mm_store_si128(&mut r as *mut _ as *mut __m128i, a);
        }
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_storeu_si128() {
        let a = __m128i::splat(9);
        let mut r = __m128i::splat(0);
        unsafe {
            sse2::_mm_storeu_si128(&mut r as *mut _ as *mut __m128i, a);
        }
        assert_eq!(r, a);
    }

    #[test]
    fn _mm_storel_epi64() {
        let a = __m128i::from(i64x2::new(2, 9));
        let mut r = __m128i::splat(0);
        unsafe {
            sse2::_mm_storel_epi64(&mut r as *mut _ as *mut __m128i, a);
        }
        assert_eq!(r, __m128i::from(i64x2::new(2, 0)));
    }

    #[test]
    fn _mm_move_epi64() {
        let a = i64x2::new(5, 6);
        assert_eq!(sse2::_mm_move_epi64(a), i64x2::new(5, 0));
    }

    #[test]
    fn _mm_packs_epi16() {
        let a = i16x8::new(0x80, -0x81, 0, 0, 0, 0, 0, 0);
        let b = i16x8::new(0, 0, 0, 0, 0, 0, -0x81, 0x80);
        let r = sse2::_mm_packs_epi16(a, b);
        assert_eq!(r, i8x16::new(
            0x7F, -0x80, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, -0x80, 0x7F));
    }

    #[test]
    fn _mm_packs_epi32() {
        let a = i32x4::new(0x8000, -0x8001, 0, 0);
        let b = i32x4::new(0, 0, -0x8001, 0x8000);
        let r = sse2::_mm_packs_epi32(a, b);
        assert_eq!(
            r, i16x8::new(0x7FFF, -0x8000, 0, 0, 0, 0, -0x8000, 0x7FFF));
    }

    #[test]
    fn _mm_packus_epi16() {
        let a = i16x8::new(0x100, -1, 0, 0, 0, 0, 0, 0);
        let b = i16x8::new(0, 0, 0, 0, 0, 0, -1, 0x100);
        let r = sse2::_mm_packus_epi16(a, b);
        assert_eq!(r, u8x16::new(
            0xFF, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0xFF));
    }

    #[test]
    fn _mm_extract_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        assert_eq!(sse2::_mm_extract_epi16(a, 5), 5);
    }

    #[test]
    fn _mm_insert_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        assert_eq!(sse2::_mm_insert_epi16(a, 9, 0), a.replace(0, 9));
    }

    #[test]
    fn _mm_movemask_epi8() {
        let a = i8x16::from(u8x16::new(
            0b1000_0000, 0b0, 0b1000_0000, 0b01, 0b0101, 0b1111_0000, 0, 0,
            0, 0, 0b1111_0000, 0b0101, 0b01, 0b1000_0000, 0b0, 0b1000_0000));
        assert_eq!(sse2::_mm_movemask_epi8(a), 0b10100100_00100101);
    }

    #[test]
    fn _mm_shuffle_epi32() {
        let a = i32x4::new(5, 10, 15, 20);
        let e = i32x4::new(20, 10, 10, 5);
        assert_eq!(sse2::_mm_shuffle_epi32(a, 0b00_01_01_11), e);
    }

    #[test]
    fn _mm_shufflehi_epi16() {
        let a = i16x8::new(1, 2, 3, 4, 5, 10, 15, 20);
        let e = i16x8::new(1, 2, 3, 4, 20, 10, 10, 5);
        assert_eq!(sse2::_mm_shufflehi_epi16(a, 0b00_01_01_11), e);
    }

    #[test]
    fn _mm_shufflelo_epi16() {
        let a = i16x8::new(5, 10, 15, 20, 1, 2, 3, 4);
        let e = i16x8::new(20, 10, 10, 5, 1, 2, 3, 4);
        assert_eq!(sse2::_mm_shufflelo_epi16(a, 0b00_01_01_11), e);
    }

    #[test]
    fn _mm_unpackhi_epi8() {
        let a = i8x16::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let b = i8x16::new(
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let e = i8x16::new(
            8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31);
        assert_eq!(sse2::_mm_unpackhi_epi8(a, b), e);
    }

    #[test]
    fn _mm_unpackhi_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        let b = i16x8::new(8, 9, 10, 11, 12, 13, 14, 15);
        let e = i16x8::new(4, 12, 5, 13, 6, 14, 7, 15);
        assert_eq!(sse2::_mm_unpackhi_epi16(a, b), e);
    }

    #[test]
    fn _mm_unpackhi_epi32() {
        let a = i32x4::new(0, 1, 2, 3);
        let b = i32x4::new(4, 5, 6, 7);
        let e = i32x4::new(2, 6, 3, 7);
        assert_eq!(sse2::_mm_unpackhi_epi32(a, b), e);
    }

    #[test]
    fn _mm_unpackhi_epi64() {
        let a = i64x2::new(0, 1);
        let b = i64x2::new(2, 3);
        let e = i64x2::new(1, 3);
        assert_eq!(sse2::_mm_unpackhi_epi64(a, b), e);
    }

    #[test]
    fn _mm_unpacklo_epi8() {
        let a = i8x16::new(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let b = i8x16::new(
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let e = i8x16::new(
            0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23);
        assert_eq!(sse2::_mm_unpacklo_epi8(a, b), e);
    }

    #[test]
    fn _mm_unpacklo_epi16() {
        let a = i16x8::new(0, 1, 2, 3, 4, 5, 6, 7);
        let b = i16x8::new(8, 9, 10, 11, 12, 13, 14, 15);
        let e = i16x8::new(0, 8, 1, 9, 2, 10, 3, 11);
        assert_eq!(sse2::_mm_unpacklo_epi16(a, b), e);
    }

    #[test]
    fn _mm_unpacklo_epi32() {
        let a = i32x4::new(0, 1, 2, 3);
        let b = i32x4::new(4, 5, 6, 7);
        let e = i32x4::new(0, 4, 1, 5);
        assert_eq!(sse2::_mm_unpacklo_epi32(a, b), e);
    }

    #[test]
    fn _mm_unpacklo_epi64() {
        let a = i64x2::new(0, 1);
        let b = i64x2::new(2, 3);
        let e = i64x2::new(0, 2);
        assert_eq!(sse2::_mm_unpacklo_epi64(a, b), e);
    }

    #[test]
    fn _mm_add_sd() {
        assert_eq!(
            sse2::_mm_add_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(6.0, 2.0));
    }

    #[test]
    fn _mm_add_pd() {
        assert_eq!(
            sse2::_mm_add_pd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(6.0, 12.0));
    }

    #[test]
    fn _mm_div_sd() {
        assert_eq!(
            sse2::_mm_div_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(0.2, 2.0));
    }

    #[test]
    fn _mm_div_pd() {
        assert_eq!(
            sse2::_mm_div_pd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(0.2, 0.2));
    }

    #[test]
    fn _mm_max_sd() {
        assert_eq!(
            sse2::_mm_max_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(5.0, 2.0));
    }

    #[test]
    fn _mm_max_pd() {
        assert_eq!(
            sse2::_mm_max_pd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(5.0, 10.0));
    }

    #[test]
    fn _mm_min_sd() {
        assert_eq!(
            sse2::_mm_min_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(1.0, 2.0));
    }

    #[test]
    fn _mm_min_pd() {
        assert_eq!(
            sse2::_mm_min_pd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(1.0, 2.0));
    }

    #[test]
    fn _mm_mul_sd() {
        assert_eq!(
            sse2::_mm_mul_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(5.0, 2.0));
    }

    #[test]
    fn _mm_mul_pd() {
        assert_eq!(
            sse2::_mm_mul_pd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(5.0, 20.0));
    }

    #[test]
    fn _mm_sqrt_sd() {
        assert_eq!(
            sse2::_mm_sqrt_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(5.0f64.sqrt(), 2.0));
    }

    #[test]
    fn _mm_sqrt_pd() {
        assert_eq!(
            sse2::_mm_sqrt_pd(f64x2::new(1.0, 2.0)),
            f64x2::new(1.0f64.sqrt(), 2.0f64.sqrt()));
    }

    #[test]
    fn _mm_sub_sd() {
        assert_eq!(
            sse2::_mm_sub_sd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(-4.0, 2.0));
    }

    #[test]
    fn _mm_sub_pd() {
        assert_eq!(
            sse2::_mm_sub_pd(f64x2::new(1.0, 2.0), f64x2::new(5.0, 10.0)),
            f64x2::new(-4.0, -8.0));
    }

    #[test]
    fn _mm_and_pd() {
        use std::mem::transmute;

        unsafe {
            let a: f64x2 = transmute(i64x2::splat(5));
            let b: f64x2 = transmute(i64x2::splat(3));
            let e: f64x2 = transmute(i64x2::splat(1));
            assert_eq!(sse2::_mm_and_pd(a, b), e);
        }
    }

    #[test]
    fn _mm_andnot_pd() {
        use std::mem::transmute;

        unsafe {
            let a: f64x2 = transmute(i64x2::splat(5));
            let b: f64x2 = transmute(i64x2::splat(3));
            let e: f64x2 = transmute(i64x2::splat(2));
            assert_eq!(sse2::_mm_andnot_pd(a, b), e);
        }
    }

    #[test]
    fn _mm_or_pd() {
        use std::mem::transmute;

        unsafe {
            let a: f64x2 = transmute(i64x2::splat(5));
            let b: f64x2 = transmute(i64x2::splat(3));
            let e: f64x2 = transmute(i64x2::splat(7));
            assert_eq!(sse2::_mm_or_pd(a, b), e);
        }
    }

    #[test]
    fn _mm_xor_pd() {
        use std::mem::transmute;

        unsafe {
            let a: f64x2 = transmute(i64x2::splat(5));
            let b: f64x2 = transmute(i64x2::splat(3));
            let e: f64x2 = transmute(i64x2::splat(6));
            assert_eq!(sse2::_mm_xor_pd(a, b), e);
        }
    }

    #[test]
    fn _mm_cmpeq_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpeq_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmplt_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmplt_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmple_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmple_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpgt_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(5.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpgt_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpge_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpge_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpord_sd() {
        use std::f64::NAN;
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(NAN, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpord_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpunord_sd() {
        use std::f64::NAN;
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(NAN, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpunord_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpneq_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(!0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpneq_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpnlt_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpnlt_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpnle_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpnle_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpngt_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(5.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpngt_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpnge_sd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, transmute(2.0f64));
            let r: u64x2 = transmute(sse2::_mm_cmpnge_sd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpeq_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, 0);
            let r: u64x2 = transmute(sse2::_mm_cmpeq_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmplt_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, !0);
            let r: u64x2 = transmute(sse2::_mm_cmplt_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmple_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, !0);
            let r: u64x2 = transmute(sse2::_mm_cmple_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpgt_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, 0);
            let r: u64x2 = transmute(sse2::_mm_cmpgt_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpge_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(!0, 0);
            let r: u64x2 = transmute(sse2::_mm_cmpge_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpord_pd() {
        use std::f64::NAN;
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(NAN, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(0, !0);
            let r: u64x2 = transmute(sse2::_mm_cmpord_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpunord_pd() {
        use std::f64::NAN;
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(NAN, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(!0, 0);
            let r: u64x2 = transmute(sse2::_mm_cmpunord_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpneq_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(!0, !0);
            let r: u64x2 = transmute(sse2::_mm_cmpneq_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpnlt_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(5.0, 3.0));
            let e = u64x2::new(0, 0);
            let r: u64x2 = transmute(sse2::_mm_cmpnlt_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpnle_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, 0);
            let r: u64x2 = transmute(sse2::_mm_cmpnle_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpngt_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(5.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, !0);
            let r: u64x2 = transmute(sse2::_mm_cmpngt_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_cmpnge_pd() {
        use std::mem::transmute;

        unsafe {
            let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
            let e = u64x2::new(0, !0);
            let r: u64x2 = transmute(sse2::_mm_cmpnge_pd(a, b));
            assert_eq!(r, e);
        }
    }

    #[test]
    fn _mm_comieq_sd() {
        use std::f64::NAN;

        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(sse2::_mm_comieq_sd(a, b));

        let (a, b) = (f64x2::new(NAN, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_comieq_sd(a, b));
    }

    #[test]
    fn _mm_comilt_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_comilt_sd(a, b));
    }

    #[test]
    fn _mm_comile_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(sse2::_mm_comile_sd(a, b));
    }

    #[test]
    fn _mm_comigt_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_comigt_sd(a, b));
    }

    #[test]
    fn _mm_comige_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(sse2::_mm_comige_sd(a, b));
    }

    #[test]
    fn _mm_comineq_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_comineq_sd(a, b));
    }

    #[test]
    fn _mm_ucomieq_sd() {
        use std::f64::NAN;

        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(sse2::_mm_ucomieq_sd(a, b));

        let (a, b) = (f64x2::new(NAN, 2.0), f64x2::new(NAN, 3.0));
        assert!(!sse2::_mm_ucomieq_sd(a, b));
    }

    #[test]
    fn _mm_ucomilt_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_ucomilt_sd(a, b));
    }

    #[test]
    fn _mm_ucomile_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(sse2::_mm_ucomile_sd(a, b));
    }

    #[test]
    fn _mm_ucomigt_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_ucomigt_sd(a, b));
    }

    #[test]
    fn _mm_ucomige_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(sse2::_mm_ucomige_sd(a, b));
    }

    #[test]
    fn _mm_ucomineq_sd() {
        let (a, b) = (f64x2::new(1.0, 2.0), f64x2::new(1.0, 3.0));
        assert!(!sse2::_mm_ucomineq_sd(a, b));
    }

    #[test]
    fn _mm_movemask_pd() {
        let r = sse2::_mm_movemask_pd(f64x2::new(-1.0, 5.0));
        assert_eq!(r, 0b01);

        let r = sse2::_mm_movemask_pd(f64x2::new(-1.0, -5.0));
        assert_eq!(r, 0b11);
    }
}
