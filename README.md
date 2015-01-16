
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

    test eval::eval_bench   ... bench:   1383174 ns/iter (+/- 197825)
    test eval::lambda_bench ... bench:    424701 ns/iter (+/- 109524)

Env v3, use HashMap represent env

    test eval::eval_bench   ... bench:   1567936 ns/iter (+/- 195648)
    test eval::lambda_bench ... bench:    475842 ns/iter (+/- 73110)
