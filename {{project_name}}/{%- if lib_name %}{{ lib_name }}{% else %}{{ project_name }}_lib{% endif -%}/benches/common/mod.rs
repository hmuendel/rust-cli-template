{% if lib_name -%}
use {{ lib_name }}::*;
{%- else -%}
use {{ project_name }}_lib::*;
{%- endif %}

#[allow(dead_code)]
pub fn init_1_thread() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 1,
        known_primes: vec![],
    })
}

#[allow(dead_code)]
pub fn init_2_threads() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 2,
        known_primes: vec![],
    })
}

#[allow(dead_code)]
pub fn init_3_threads() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 3,
        known_primes: vec![],
    })
}

#[allow(dead_code)]
pub fn init_4_threads() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 4,
        known_primes: vec![],
    })
}

#[allow(dead_code)]
pub fn init_one_per_cpu() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: num_cpus::get(),
        known_primes: vec![],
    })
}

#[allow(dead_code)]
pub fn init_two_per_cpu() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 2 * num_cpus::get(),
        known_primes: vec![],
    })
}

#[allow(dead_code)]
pub fn init_100_threads() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 100,
        known_primes: vec![],
    })
}
