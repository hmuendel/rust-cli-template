{% if lib_name -%}
use {{  lib_name | replace(from="-", to="_") }}::*;
{%- else -%}
use {{  project_name | replace(from="-", to="_") }}_lib::*;
{%- endif %}
pub fn init() {
    Config::init_if_possible(Config {
        number_of_iterations: 2,
        number_of_threads: 8,
        known_primes: vec![11, 13],
    });
}
