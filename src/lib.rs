// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(cfg_target_feature)]
#![feature(specialization)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(inclusive_range))]

#[cfg(test)] extern crate test;

extern crate stdsimd;

mod vecs;
mod iters;
mod intrin;


pub use iters::*;
pub use vecs::{u8s, i8s, u16s, i16s, u32s, i32s, f32s, u64s, i64s, f64s, Packed};
pub use intrin::*;

#[cfg(test)]
mod tests {
    use super::*;
    use super::vecs::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_nop_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 128][..]).simd_iter().simd_map(|v| v).scalar_collect())
        });
    }

    #[bench]
    fn bench_nop_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[0u8; 128][..]).iter().map(|e| *e).collect::<Vec<u8>>())
        });
    }

    #[bench]
    fn bench_map_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 128][..]).simd_iter()
                    .simd_map(|v| { f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() -
                                    f32s::splat(4.0) - f32s::splat(2.0) })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_map_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 127][..]).simd_iter()
                    .simd_map(|v| { f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt() -
                                    f32s::splat(4.0) - f32s::splat(2.0) })
                    .scalar_collect())
        })
    }

    #[bench]
    fn bench_map_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 128][..]).iter()
                    .map(|v| { 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt() -
                               4.0 - 2.0 })
                    .collect::<Vec<f32>>())
        });
    }

    #[bench]
    fn bench_reduce_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 128][..]).simd_iter()
                    .simd_reduce(f32s::splat(0.0), |a, v| *a + f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()).sum())
        })
    }

    #[bench]
    fn bench_reduce_uneven_simd(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 127][..]).simd_iter()
                    .simd_reduce(f32s::splat(0.0), |a, v| *a + f32s::splat(9.0) * v.abs().sqrt().rsqrt().ceil().sqrt()).sum())
        })
    }

    #[bench]
    fn bench_reduce_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(
                (&[-123.456f32; 128][..]).iter()
                    .fold(0.0, |a, v| a + 9.0 * v.abs().sqrt().sqrt().recip().ceil().sqrt()))
        })
    }
}
