# Self-Ref [![Build Status](https://travis-ci.org/Mixthos/try_or.svg?branch=master)](https://travis-ci.org/Mixthos/try_or)

Contains the macros `try_or!` and `try_or_else!`, which are to `try!` what `unwrap_or` and `unwrap_or_else` are to `unwrap()`.

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
}
```
