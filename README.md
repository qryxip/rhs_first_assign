# rhs\_first\_assign

[![CI](https://github.com/qryxip/rhs_first_assign/workflows/CI/badge.svg)](https://github.com/qryxip/rhs_first_assign/actions?workflow=CI)
[![codecov](https://codecov.io/gh/qryxip/rhs_first_assign/branch/master/graph/badge.svg)](https://codecov.io/gh/qryxip/rhs_first_assign/branch/master)
[![dependency status](https://deps.rs/repo/github/qryxip/rhs_first_assign/status.svg)](https://deps.rs/repo/github/qryxip/rhs_first_assign)
[![Crates.io](https://img.shields.io/crates/v/rhs_first_assign.svg)](https://crates.io/crates/rhs_first_assign)
[![Crates.io](https://img.shields.io/crates/l/rhs_first_assign.svg)](https://crates.io/crates/rhs_first_assign)

An attribute macro to hack compound assignment.

## Motivation

```rust
use std::num::Wrapping;

fn main() {
    let mut xs = vec![Wrapping(1), Wrapping(2)];

    // OK
    xs[1] = xs[0] + xs[1];

    // Error
    xs[1] += xs[0];
}
```

```
error[E0502]: cannot borrow `xs` as immutable because it is also borrowed as mutable
  --> src/main.rs:10:14
   |
10 |     xs[1] += xs[0];
   |     ---------^^---
   |     |        |
   |     |        immutable borrow occurs here
   |     mutable borrow occurs here
   |     mutable borrow later used here
```

## Usage

```rust
use rhs_first_assign::rhs_first_assign;

use std::num::Wrapping;

#[rhs_first_assign]
fn main() {
    let mut xs = vec![Wrapping(1), Wrapping(2)];

    xs[1] = xs[0] + xs[1];

    xs[1] += xs[0];
}
```

↓

```rust
use std::num::Wrapping;

fn main() {
    let mut xs = vec![Wrapping(1), Wrapping(2)];

    xs[1] = xs[0] + xs[1];

    {
        let __rhs_first_assign_rhs_l11_c10 = xs[0];
        xs[1] += __rhs_first_assign_rhs_l11_c10;
    };
}
```

## License

Licensed under <code>[MIT](https://opensource.org/licenses/MIT) OR [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)</code>.
