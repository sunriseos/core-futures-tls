//! Libcore wrapper allowing async/await
//!
//! # Usage
//!
//! Put the following in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! core = { package = "core-futures-tls", version = "0.1.0" }
//! ```
//!
//! # Why
//!
//! Currently, async/await is not usable from libcore. Attempting to call .await
//! from a `no_std` crate will yield the following error:
//!
//! ```rust
//! error[E0433]: failed to resolve: could not find `poll_with_tls_context` in `future`
//! error[E0433]: failed to resolve: could not find `from_generator` in `future`
//! ```
//!
//! This is due to await lowering to some code calling the functions
//! `::core::futures::poll_with_tls_context` and `::core::futures::from_generator`
//! in order to setup a per-thread context containing the current task. Those
//! functions, however, do not exist. The equivalent functions are defined in
//! libstd. They set up a thread-local variable which contains the current task
//! being executed. When polling a future, this task will get retrieved in order
//! to call the future's `poll` function.
//!
//! 
//! As mentioned, the libstd version of those functions use a thread-local
//! variable, which is only supported in rust's libstd through the
//! `thread_local!` macro - which doesn't exist in libcore. There is, however,
//! an alternative: The (unstable) `#[thread_local]` attribute, which uses ELF
//! TLS. Note that ELF TLS is not portable to all targets - it needs to be
//! supported by the OS, the loader, etc...
//!
//! Here's a small example of the thread_local attribute in action:
//!
//! ```rust
//! #[thread_local]
//! static TLS_CX: Cell<i32> = Cell::new(1);
//! ```
//!
//! Using this trick, we can copy paste libstd's implementation of the
//! `poll_with_tls_context`/`from_generator` functions, but replacing the
//! `thread_local!` macro with a `#[thread_local]` macro. Ez pz.
//!
//! # Wrapping libcore
//!
//! This trick is nice, but compiling a custom libcore is fastidious. Instead,
//! we're going to wrap libcore, exposing our own libcore that just reexports
//! the real libcore's functions, and adding our own extra stuff. This,
//! surprisingly, can be done simply by declaring a `core` dependency with the
//! `package` attribute set to our "real" crates.io package name. This will
//! trick cargo into giving rustc our wrapper core as if it was the real
//! libcore.
//!
//! So that's it. All this crate does is reexport libcore, adding a couple
//! functions in the future module. You just have to use the following in your
//! Cargo.toml in order to use it, and rust will happily use `core-futures-tls`
//! as if it was the libcore.
//!
//! ```toml
//! [dependencies]
//! core = { package = "core-futures-tls", version = "0.1.0" }
//! ```
//!
//! # Closing thoughts
//!
//! While this crate still uses TLS, it should be possible to create a version
//! that stores the thread local context in a global for single-threaded systems
//! such as microcontrollers. This is left as an exercise to the reader.

#![no_std]
#![feature(thread_local, generator_trait, optin_builtin_traits)]

pub mod future;
pub use core::*;
