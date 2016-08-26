//! Contains the macros `try_or!`, `try_or_else!`, `try_opt_or!` and
//! `try_opt_or_else!`. These are helper macros for unwrapping a `Result` or an
//! `Option` while returning early for `Err` and `None` values. The semantics
//! are similar to `unwrap_or` and `unwrap_or_else`.
//!
//! If you want a `try_opt!` macro, there's already another crate (`try_opt`)
//! for that.

#![no_std]


/// Helper macro for unwrapping `Result` values. Returns early with the value
/// of the second parameter if the first parameter is `Err`.
#[macro_export]
macro_rules! try_or {
	( $expr:expr , $or:expr ) => {{
		match { $expr } {
			Ok(res) => res,
			Err(_) => { return { $or }; },
		}
	}}
}

/// Unwraps a `Result`. If the result is `Err`, calls the function `$or_fn` with
/// its value and returns early with its result.
#[macro_export]
macro_rules! try_or_else {
	( $expr:expr , $or_fn:expr ) => {{
		match { $expr } {
			Ok(res) => res,
			Err(err) => { return { $or_fn }(err); },
		}
	}}
}


/*/// Unwraps an `Option`, returning `None` if the option is `None`.
#[macro_export]
macro_rules! try_opt {
	( $expr:expr ) => {{
		match { $expr } {
			Some(val) => val,
			None => return None,
		}
	}}
}*/

/// Unwraps an `Option`, returning the second parameter if the option is `None`.
#[macro_export]
macro_rules! try_opt_or {
	( $expr:expr , $or:expr ) => {{
		match { $expr } {
			Some(res) => res,
			None => return { $or },
		}
	}}
}

/// Unwraps an `Option`. If the result is `None`, calls the function `$or_fn`
/// and returns its result.
#[macro_export]
macro_rules! try_opt_or_else {
	( $expr:expr , $or_fn:expr ) => {{
		match { $expr } {
			Some(res) => res,
			None => return { $or_fn }(),
		}
	}}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
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
}
