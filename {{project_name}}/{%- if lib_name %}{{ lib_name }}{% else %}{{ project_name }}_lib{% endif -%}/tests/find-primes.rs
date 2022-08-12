{% if lib_name -%}
use {{  lib_name | replace(from="-", to="_") }}::*;
{%- else -%}
use {{  project_name | replace(from="-", to="_") }}_lib::*;
{%- endif %}
use table_test::table_test;
use test_log::test;
mod common;
use common::init;

#[test]
fn test_inverted_range() {
    assert_eq!(find_possible_primes(10, 0), vec![]);
}
#[test]
fn test_finding_primes() {
    init();
    let cases = vec![
        ((0, 0), vec![]),
        ((1, 1), vec![]),
        ((2, 2), vec![2]),
        ((3, 3), vec![3]),
        ((4, 4), vec![]),
        ((1, 10), vec![2, 3, 5, 7]),
        ((2, 10), vec![2, 3, 5, 7]),
        ((3, 10), vec![3, 5, 7]),
        ((4, 10), vec![5, 7]),
        ((1, 15), vec![2, 3, 5, 7, 11, 13]),
        ((5, 15), vec![5, 7, 11, 13]),
        ((1, 17), vec![2, 3, 5, 7, 11, 13, 17]),
        ((u32::MAX - 10, u32::MAX), vec![u32::MAX - 4]),
    ];
    for (validator, (from, to), expected_primes) in table_test!(cases) {
        let mut actual_primes = find_possible_primes(from, to);
        actual_primes.sort();
        validator
            .given(&format!("finding primes between {} and {}", from, to))
            .then(&format!("expecting {:#?} ", expected_primes))
            .assert_eq(expected_primes, actual_primes);
    }
}
