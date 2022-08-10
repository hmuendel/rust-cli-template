#![feature(test)]

{% if lib_name -%}
use {{ lib_name }}::*;
{%- else -%}
use {{ project_name }}_lib::*;
{%- endif %}
use table_test::table_test;
extern crate test;
use common::init;
use test_log::test;
mod common;

#[test]
fn test_miller_rabin() {
    init();
    let cases = vec![
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (2147483647, true),
        (u32::MAX - 5, false),
        (u32::MAX - 4, true),
        (u32::MAX - 3, false),
        (u32::MAX - 2, false),
        (u32::MAX - 1, false),
        (u32::MAX, false),
    ];
    for (validator, input, expect_prime) in table_test!(cases) {
        let is_prime = rabin_miller(input);
        validator
            .given(&format!("number to test {}", input))
            .then(&format!("{} should be prime: {}", input, expect_prime))
            .assert_eq(expect_prime, is_prime);
    }
}
