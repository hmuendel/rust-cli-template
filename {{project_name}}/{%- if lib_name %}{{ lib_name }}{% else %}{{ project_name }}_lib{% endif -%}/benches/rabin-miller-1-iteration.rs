#![feature(test)]

{% if lib_name -%}
use {{ lib_name }}::*;
{%- else -%}
use {{ project_name }}_lib::*;
{%- endif %}
extern crate test;
mod common;
use common::init_two_per_cpu;

#[bench]
fn bench_prime_finding_first_16_with_two_thread_per_cpu(b: &mut test::Bencher) {
    init_two_per_cpu();
    b.iter(|| find_possible_primes(1, 16));
}
#[bench]
fn bench_prime_finding_last_16_with_two_thread_per_cpu(b: &mut test::Bencher) {
    init_two_per_cpu();
    b.iter(|| find_possible_primes(u32::MAX - 15, u32::MAX));
}
#[bench]
fn bench_prime_finding_first_8096_with_two_thread_per_cpu(b: &mut test::Bencher) {
    init_two_per_cpu();
    b.iter(|| find_possible_primes(1, 8096));
}
