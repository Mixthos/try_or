# try_or [![Build Status](https://travis-ci.org/Mixthos/try_or.svg?branch=master)](https://travis-ci.org/Mixthos/try_or)

Contains the macros `try_or!`, `try_or_else!`, `try_opt_or!` and
`try_opt_or_else!`. These are helper macros for unwrapping a `Result` or an
`Option` while returning early for `Err` and `None` values. The semantics
are similar to `unwrap_or` and `unwrap_or_else`.

If you want a `try_opt!` macro, there's already another crate (`try_opt`)
for that.

## [Documentation](http://mixthos.github.com/try_or)

## Usage

Add to your Cargo.toml:

```toml
[dependencies]
try_or = "0.1"
```

Use the macros like this:

```rust
#[macro_use]
extern crate try_or;

fn main() {
	// test try_or!
	assert_eq!({ || try_or!("5".parse::<u32>(), 1) } (), 5);
	assert_eq!({ || try_or!("a".parse::<u32>(), 1) } (), 1);
	
	// test try_or_else!
	assert_eq!({ || try_or_else!("1".parse::<u32>(), |_| 5) } (), 1);
	assert_eq!({ || try_or_else!("a".parse::<u32>(), |_| 5) } (), 5);
	
	// test try_opt_or!
	assert_eq!({ || try_opt_or!(Some(1), 2) } (), 1);
	assert_eq!({ || try_opt_or!(None, 2) } (), 2);
	
	// test try_or_else!
	assert_eq!({ || try_opt_or_else!(Some(1), || 2) } (), 1);
	assert_eq!({ || try_opt_or_else!(None, || 2) } (), 2);
}
```
