//! Contains the macros `try_or!` and `try_or_else!`, which are to
//! `try!` what `unwrap_or` and `unwrap_or_else` are to `unwrap()`.

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
    }
}
