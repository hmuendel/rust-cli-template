//! We are trying to find possible prime numbers in a range of numbers.
//! For this we use a probabilistic prime testing algorithms, on all the numbers
//! in the range, to get candiates with high probability. In our case we use the
//! [Rabin Miller primality test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
//! the algorithm can be implemented in pseudocode like this.
//! ```text
//! Input #1: n > 3, an odd integer to be tested for primality
//! Input #2: k, the number of rounds of testing to perform
//! Output: “composite” if n is found to be composite, “probably prime” otherwise
//!
//! write n as 2s·d + 1 with d odd (by factoring out powers of 2 from n − 1)
//! WitnessLoop: repeat k times:
//!     pick a random integer a in the range [2, n − 2]
//!     x ← ad mod n
//!     if x = 1 or x = n − 1 then
//!         continue WitnessLoop
//!     repeat s − 1 times:
//!         x ← x2 mod n
//!         if x = n − 1 then
//!             continue WitnessLoop
//!     return “composite”
//! return “probably prime”
//! ```
//!
#![feature(test)]
use once_cell::sync::OnceCell;
use rand::{self, Rng};
use tracing::{debug, error, info, instrument, span, trace, warn};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug)]
pub struct Config {
    pub number_of_threads: usize,
    pub number_of_iterations: usize,
    pub known_primes: Vec<u32>,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Config: number_of_threads: {}, number_of_iterations: {}, known_primes: {:?} ",
            self.number_of_threads, self.number_of_iterations, self.known_primes
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            number_of_threads: num_cpus::get(),
            number_of_iterations: 100,
            known_primes: vec![],
        }
    }
}

impl Config {
    pub fn init(cfg: Self) -> Result<(), Config> {
        CONFIG.set(cfg)?;
        Ok(())
    }
    pub fn init_if_possible(cfg: Self) {
        _ = CONFIG.set(cfg);
    }

    pub fn init_default() -> Result<(), Config> {
        CONFIG.set(Self::default())?;
        Ok(())
    }

    pub fn init_default_if_possible() {
        _ = CONFIG.set(Self::default());
    }
}

#[instrument(level = "trace")]
fn modular_exponentiation(base: u32, exponent: u32, modulus: u32) -> u32 {
    let mut result = 1;
    let mut base = base as u64;
    let mut exponent = exponent as u32;
    let modulus = modulus as u64;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
        .try_into()
        .expect("modular exponentiation result is too large to fit into u32")
}

/// Factors out powers of 2 from n to bring it to the form 2^s·d + 1 with d odd.
/// Returns (s, d).
#[instrument(level = "trace")]
fn factor_out_2(mut n: u32) -> (u32, u32) {
    if n == 0 {
        trace!("n is zero");
        return (0, 0);
    }
    let mut exponent = 0;
    while n & 1 == 0 {
        n = n >> 1;
        exponent += 1;
        trace!("can be written as 2^{}*{}", exponent, n);
    }
    trace!("done factoring");
    (exponent, n)
}

#[instrument(level = "trace")]
pub fn rabin_miller(n: u32) -> bool {
    let number_of_iterations = CONFIG.wait().number_of_iterations;
    let known_primes = &CONFIG.wait().known_primes;
    match n {
        0 | 1 => {
            return false;
        }
        2 => {
            return true;
        }
        3 => {
            return true;
        }
        //filter even numbers
        _ if n & 1 == 0 => {
            return false;
        }
        _ if known_primes[..].contains(&n) => {
            trace!("{} is a known prime", n);
            return true;
        }
        _ => {
            let mut rng = rand::thread_rng();
            let (s, d) = factor_out_2(n - 1);
            'witness_loop: for i in 0..number_of_iterations {
                let _loop_span =
                    span!(tracing::Level::TRACE, "wittness_loop", i = i, s = s, d = d).entered();
                let a = rng.gen_range(2..n - 1);
                trace!("picking random a = {} from [{}, {}]", a, 2, n - 2);
                let mut x = modular_exponentiation(a, d, n);
                trace!("a^d mod n = {}^{} mod {} = x = {}", a, d, n, x);
                if x == 1 || x == n - 1 {
                    trace!("x = {} is 1 or n-1 = {}", x, n - 1);
                    continue;
                }
                trace!("x = {} is not 1 or n-1 = {}", x, n - 1);
                for j in 0..s - 1 {
                    let _inner_loop_span =
                        span!(tracing::Level::TRACE, "inner_loop", j = j).entered();
                    x = modular_exponentiation(x, 2, n);
                    trace!("x = x^2 mod n = {}", x);
                    if x == n - 1 {
                        continue 'witness_loop;
                    }
                }
                return false;
            }
            true
        }
    }
}

#[instrument(level = "info")]
pub fn find_possible_primes(from: u32, to: u32) -> Vec<u32> {
    if from > to {
        error!("from ({}) is greater than to ({})", from, to);
        return vec![];
    }
    let number_of_threads = CONFIG.wait().number_of_threads;
    info!("finding possible primes between {} and {}", from, to);
    // the orde of the addition matters if to is max int, it might overflow
    let mut interval_size = ((to - from) + 1) / number_of_threads as u32;
    if interval_size == 0 {
        warn!("more threads than numbers to check");
        interval_size = 1;
    }
    debug!("interval size = {}", interval_size);
    let mut threads = vec![];
    for i in 0..number_of_threads {
        let _thread_loop_span = span!(tracing::Level::DEBUG, "thread_loop", i = i).entered();
        let from = from + i as u32 * interval_size;
        if from > to {
            break;
        }
        let to = if i == number_of_threads - 1 {
            debug!("last thread");
            to
        } else {
            debug!("not last thread");
            from + interval_size - 1
        };
        debug!("starting thread {} with range [{}, {})", i, from, to);
        let handle = std::thread::spawn(move || {
            let _thread_span =
                span!(tracing::Level::INFO, "thread", i = i, from = from, to = to).entered();
            let mut possible_primes = Vec::new();
            for n in from..to {
                if rabin_miller(n) {
                    trace!("{} is a possible prime", n);
                    possible_primes.push(n);
                }
            }
            if rabin_miller(to) {
                trace!("{} is a possible prime", to);
                possible_primes.push(to);
            }
            return possible_primes;
        });
        threads.push(handle);
    }
    let mut possible_primes = Vec::new();
    for handle in threads {
        let thread_possible_primes = handle.join().unwrap();
        possible_primes.extend(thread_possible_primes);
    }
    possible_primes
}

#[cfg(test)]
mod tests {
    use super::*;
    use table_test::table_test;
    extern crate test;
    use test_log::test;
    #[test]
    fn test_even_factorisations() {
        let cases = vec![
            (0, (0, 0)),
            (2, (1, 1)),
            (4, (2, 1)),
            (6, (1, 3)),
            (8, (3, 1)),
            (10, (1, 5)),
            (12, (2, 3)),
            (14, (1, 7)),
            (16, (4, 1)),
            (18, (1, 9)),
        ];
        for (validator, input, (exponent, remainder)) in table_test!(cases) {
            let (actual_exp, actual_remainder) = factor_out_2(input);
            validator
                .given(&format!("factor_out_2({})", input))
                .then(&format!(
                    "(the exponent should be {}, the remainder should be {} so that 2^{0}·{1}={2})",
                    exponent, remainder, input
                ))
                .assert_eq((exponent, remainder), (actual_exp, actual_remainder));
        }
    }
    #[test]
    fn test_odd_factorisations() {
        let cases = vec![
            (1, (0, 1)),
            (3, (0, 3)),
            (5, (0, 5)),
            (7, (0, 7)),
            (9, (0, 9)),
            (11, (0, 11)),
            (13, (0, 13)),
            (15, (0, 15)),
            (17, (0, 17)),
        ];
        for (validator, input, (exponent, remainder)) in table_test!(cases) {
            let (actual_exp, actual_remainder) = factor_out_2(input);
            validator
                .given(&format!("factor_out_2({})", input))
                .then(&format!(
                    "(the exponent should be {}, the remainder should be {} so that 2^{0}·{1}={2})",
                    exponent, remainder, input
                ))
                .assert_eq((exponent, remainder), (actual_exp, actual_remainder));
        }
    }
    #[bench]
    fn bench_factorisation(b: &mut test::Bencher) {
        b.iter(|| {
            let n = rand::random::<u32>();
            factor_out_2(n);
        });
    }
    #[test]
    fn test_modular_exponentiation() {
        let cases = vec![
            ((1, 1, 1), 0),
            ((2, 1, 1), 0),
            ((22108, 1, 1), 0),
            ((9143147, 14080184, 4108408124), 3038037621),
            ((u32::MAX, u32::MAX, 1), 0),
            ((u32::MAX, u32::MAX, 2), 1),
            ((u32::MAX, u32::MAX, 14080184), 4477615),
            ((u32::MAX, u32::MAX, u32::MAX - 4), 1024),
            ((u32::MAX, u32::MAX, u32::MAX - 3), 4261108347),
            ((u32::MAX, u32::MAX, u32::MAX - 2), 2072831017),
            ((u32::MAX, u32::MAX, u32::MAX - 1), 1),
            ((u32::MAX, u32::MAX, u32::MAX), 0),
        ];
        for (validator, (base, exponent, modulus), expected) in table_test!(cases) {
            let actual = modular_exponentiation(base, exponent, modulus);
            validator
                .given(&format!("{}^{} mod {}", base, exponent, modulus))
                .then(&format!("should be {}", expected))
                .assert_eq(expected, actual);
        }
    }
}
