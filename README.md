
[![Build Status](https://travis-ci.org/chenyukang/rust-scm.svg)](https://travis-ci.org/chenyukang/rust-scm)

# Rust-scm

* A mini Scheme interpreter using Rust
* My first Rust program, should not be the best Rust programming idiom.

# Benchmarks

* 0.1

Env use Vec, should be very slow.

    test eval::eval_bench   ... bench:   1500071 ns/iter (+/- 256219)
    test eval::lambda_bench ... bench:    469318 ns/iter (+/- 86833)

Env v2, use String as env key, not the SymboNode

    test env::env_bench      ... bench:   8108095 ns/iter (+/- 1626045)
    test env::env_bench_iter ... bench:    623332 ns/iter (+/- 639534)
    test eval::eval_bench    ... bench:   1469169 ns/iter (+/- 333122)
    test eval::lambda_bench  ... bench:    487463 ns/iter (+/- 91102)

Env v3, use HashMap represent env

    test env::env_bench      ... bench:   1220791 ns/iter (+/- 174220)
    test env::env_bench_iter ... bench:    773957 ns/iter (+/- 128140)
    test eval::eval_bench    ... bench:   1517271 ns/iter (+/- 267982)
    test eval::lambda_bench  ... bench:    458025 ns/iter (+/- 100774)
