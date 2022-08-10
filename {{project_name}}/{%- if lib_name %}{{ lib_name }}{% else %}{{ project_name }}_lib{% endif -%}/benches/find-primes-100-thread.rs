#![feature(test)]

{% if lib_name -%}
use {{ lib_name }}::*;
{%- else -%}
use {{ project_name }}_lib::*;
{%- endif %}
extern crate test;
mod common;
use common::init_100_threads;

#[bench]
fn bench_prime_finding_first_16_with_100_threads(b: &mut test::Bencher) {
    init_100_threads();
    b.iter(|| find_possible_primes(1, 16));
}

// #[bench]
// fn bench_prime_finding_last_16_with_100_threads(b: &mut test::Bencher) {
//     init_100_threads();
//     b.iter(|| find_possible_primes(u32::MAX - 15, u32::MAX));
// }
//
#[bench]
fn bench_prime_finding_first_8096_with_100_threads(b: &mut test::Bencher) {
    init_100_threads();
    b.iter(|| find_possible_primes(1, 8096));
}
