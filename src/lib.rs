//! This crate provides a single macro called `if_chain!`.
//!
//! `if_chain!` lets you write long chains of nested `if` and `if let`
//! statements without the associated rightward drift. It also supports multiple
//! patterns (e.g. `if let Foo(a) | Bar(a) = b`) in places where Rust would
//! normally not allow them.
//!
//! See the associated [blog post] for the background behind this crate.
//!
//! [blog post]: https://lambda.xyz/blog/if-chain
//!
//! # Note about recursion limits
//!
//! If you run into "recursion limit reached" errors while using this macro, try
//! adding
//!
//! ```rust,ignore
//! #![recursion_limit = "1000"]
//! ```
//!
//! to the top of your crate.
//!
//! # Examples
//!
//! ## Quick start
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Some(y) = x;
//!     if y.len() == 2;
//!     if let Some(z) = y;
//!     then {
//!         do_stuff_with(z);
//!     }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! if let Some(y) = x {
//!     if y.len() == 2 {
//!         if let Some(z) = y {
//!             do_stuff_with(z);
//!         }
//!     }
//! }
//! ```
//!
//! ## Fallback values with `else`
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Some(y) = x;
//!     if let Some(z) = y;
//!     then {
//!         do_stuff_with(z)
//!     } else {
//!         do_something_else()
//!     }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! if let Some(y) = x {
//!     if let Some(z) = y {
//!         do_stuff_with(z)
//!     } else {
//!         do_something_else()
//!     }
//! } else {
//!     do_something_else()
//! }
//! ```
//!
//! ## Intermediate variables with `let`
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Some(y) = x;
//!     let z = y.some().complicated().expression();
//!     if z == 42;
//!     then {
//!        do_stuff_with(y);
//!     }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! if let Some(y) = x {
//!     let z = y.some().complicated().expression();
//!     if z == 42 {
//!         do_stuff_with(y);
//!     }
//! }
//! ```
//!
//! ## Multiple patterns
//!
//! ```rust,ignore
//! if_chain! {
//!     if let Foo(y) | Bar(y) | Baz(y) = x;
//!     let Bubbles(z) | Buttercup(z) | Blossom(z) = y;
//!     then { do_stuff_with(z) }
//! }
//! ```
//!
//! becomes
//!
//! ```rust,ignore
//! match x {
//!     Foo(y) | Bar(y) | Baz(y) => match y {
//!         Bubbles(z) | Buttercup(z) | Blossom(z) => do_stuff_with(z)
//!     },
//!     _ => {}
//! }
//! ```
//!
//! Note that if you use a plain `let`, then `if_chain!` assumes that the
//! pattern is *irrefutable* (always matches) and doesn't add a fallback branch.

#![cfg_attr(not(test), no_std)]

/// Macro for writing nested `if let` expressions.
///
/// See the crate documentation for information on how to use this macro.
#[macro_export]
macro_rules! if_chain {
    ($($tt:tt)*) => {
        __if_chain! { @init () $($tt)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __if_chain {
    (@init ($($tt:tt)*) then $then:block else $other:block) => {
        __if_chain! { @expand $other $($tt)* then $then }
    };
    (@init ($($tt:tt)*) then $then:block) => {
        __if_chain! { @expand {} $($tt)* then $then }
    };
    (@init ($($tt:tt)*) $head:tt $($tail:tt)*) => {
        __if_chain! { @init ($($tt)* $head) $($tail)* }
    };
    (@expand $other:block let $($pat:pat)|+ = $expr:expr; $($tt:tt)+) => {
        match $expr {
            $($pat)|+ => __if_chain! { @expand $other $($tt)+ }
        }
    };
    (@expand $other:block if let $($pat:pat)|+ = $expr:expr; $($tt:tt)+) => {
        match $expr {
            $($pat)|+ => __if_chain! { @expand $other $($tt)+ },
            _ => $other
        }
    };
    (@expand {} if $expr:expr; $($tt:tt)+) => {
        if $expr {
            __if_chain! { @expand {} $($tt)+ }
        }
    };
    (@expand $other:block if $expr:expr; $($tt:tt)+) => {
        if $expr {
            __if_chain! { @expand $other $($tt)+ }
        } else {
            $other
        }
    };
    (@expand $other:block then $then:block) => {
        $then
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let x: Option<Result<Option<String>, (u32, u32)>> = Some(Err((41, 42)));
        let mut success = false;
        if_chain! {
            if let Some(y) = x;
            if let Err(z) = y;
            let (_, b) = z;
            if b == 42;
            then { success = true; }
        }
        assert!(success);
    }

    #[test]
    fn empty() {
        let success;
        if_chain! {
            then { success = true; }
        }
        assert!(success);
    }

    #[test]
    fn empty_with_else() {
        let success;
        if_chain! {
            then { success = true; }
            else { unreachable!(); }
        }
        assert!(success);
    }

    #[test]
    fn if_let_multiple_patterns() {
        #[derive(Copy, Clone)]
        enum Robot { Nano, Biscuit1, Biscuit2 }
        for &(robot, expected) in &[
            (Robot::Nano, false),
            (Robot::Biscuit1, true),
            (Robot::Biscuit2, true),
        ] {
            let is_biscuit = if_chain! {
                if let Robot::Biscuit1 | Robot::Biscuit2 = robot;
                then { true } else { false }
            };
            assert_eq!(is_biscuit, expected);
        }
    }

    #[test]
    fn let_multiple_patterns() {
        let x: Result<u32, u32> = Ok(42);
        if_chain! {
            let Ok(x) | Err(x) = x;
            then { assert_eq!(x, 42); }
            else { panic!(); }
        }
    }
}
