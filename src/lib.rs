//! To use the crate without lints:
//! 1. Invoke this macro at the top of your crate (`lib.rs`, or in your binary crates if they don't
//!    have `lib.rs`). Like this, **with** the leading double colon `::`
//!    ```
//!    ::prudent::load!();
//!    ```
//!    But, from here on, never refer to `::prudent`. Instead, use `crate::prudent`. (You could use
//!    `self::prudent` at the top level of your `lib.rs` (or in the top level of your binary
//!    crates), but `crate::prudent` works everywhere.)
//! 2. Wildcard import. This must be **without** any leading double colon `::`!
//!    ```ignore
//!    use crate::prudent::*;
//!    ```
//!
//! If you need lints
//! - in doctests or custom integration tests (even if your crate is published on <crates.io>); or
//! - if your crate is not published on <crates.io>
//!
//! then pass the first parameter, a relative/absolute file path to your local clone/git submodule
//! copy/other copy of `src/frontend_linted.rs`. So, instead of `#1` above, have something like:
//! ```ignore
//!    ::prudent::load!("../../prudent/src/frontend_linted.rs");
//!    use crate::prudent::*;
//! ```
//!
//! Pass a second parameter, after `=>`, if you want the loaded module to have name of your choice
//! (other than `prudent`). For example:
//! ```ignore
//!    ::prudent::load!("../../prudent/src/frontend_linted.rs" => prudentish);
//!    use crate::prudentish::*;
//! ```
#![allow(clippy::useless_attribute)]
#![allow(clippy::needless_doctest_main)]
//! # Examples (linted)
#![doc  = internal_coverage_positive!("any: \"frontend_linted.rs\"") ]
//! # Examples (not linted)
#![doc  = internal_coverage_positive!("") ]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test)), no_std)]
#![forbid(unknown_lints)]
// We can't `#![forbid(unused)]`, because our macros issue `#[allow(unused_unsafe)]`. Without that
// unsafe_method! existed only as multiple specialized macros: unsafe_method_ref!,
// unsafe_method_mut!... And there were problems with unintended duplicates of Copy `self` when
// invoking methods with the receiver being &self, that is, a shared reference.
//
// @TODO:
//
//#![deny(unused)]
//#![forbid(dead_code)]
#![deny(missing_docs)]
// rustdoc lints: https://doc.rust-lang.org/rustdoc/lints.html
//
// rustdoc::missing_doc_code_examples we don't apply here, because it's nightly-only. Instead, we
// invoke it with `nightly` toolchain from .github/workflows/main.yml.
//
// rustdoc::invalid_codeblock_attributes we do NOT warn/deny/forbid here, even though it IS stable.
// That's because in `compile_fail` doctests we use error numbers, which are UNSTABLE only. But, to
// allow the same doctest also on stable toolchain, we ALLOW it here.
//
// See
// https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests.
//
// But, as per that page, error numbers work on `nightly` only. On `stable` they make "code sample
// being interpreted as plain text"! So, we have those doctests in separate files, and we include
// them twice:
// - without the error code, and run as a part of standard doc generation, on `stable`. That makes
//   the doctest code formatted on `stable`.
// - with the error code, but on an element that has `#[cfg(doctest)]` attribute, so it's NOT a part
//   of doc generation, but it is still tested. That way it doesn't matter that `stable` would
//   format it as plain text, and it does get tested on `nightly` as a part of CI.
//
// Then on `nightly` we forbid rustdoc::invalid_codeblock_attributes. See
// .github/workflows/main.yml.
#![forbid(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks,
    rustdoc::redundant_explicit_links
)]
// Do not inject `extern crate prudent` to doctests, because load!() defines module `prudent`.
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(unused, dead_code))))]
// Workaround for https://github.com/rust-lang/rust/issues/148599
#![doc(test(attr(allow(forbidden_lint_groups))))]

#[cfg(doc)]
extern crate alloc;

#[doc(hidden)]
#[macro_export]
macro_rules! internal_coverage_positive {
    (
        $load_params:literal
    ) => {
        $crate::internal_coverage_positive!(
            $load_params,
            "# unsafe_fn" -> "../coverage_positive/fn.rs",
            "# unsafe_method\n## unsafe_method > self: shared reference" -> "../coverage_positive/md-shared_ref.rs"
        )
    };
    (
        $load_params:literal,
        $( $description:literal -> $file:literal ),*
    ) => {
        ::core::concat!(
        $(
            $description,
            "\n```\n",
            "::prudent::load!(", $load_params, ");\n",
            ::core::include_str!($file),
            // just in case the file doesn't end with a new line, inject it anyway:
            "\n```\n",
        )*
        "\n"
        )
    };
}

pub mod backend;

#[cfg(feature = "internal_use_frontend_linted")]
compile_error!("Use feature internal_use_frontend_linted only for easier editing.");

/// Frontend macros.
#[cfg(not(feature = "internal_use_frontend_linted"))]
#[path = "frontend_unlinted.rs"]
mod frontend_untested;
#[cfg(feature = "internal_use_frontend_linted")]
#[path = "frontend_linted.rs"]
mod frontend_untested;

#[path = "frontend_with_compile_fail_tests.rs"]
#[doc(hidden)]
pub mod frontend;

/// No need to be public. The only functionality is macros, which are exported even if private.
mod frontend_loader;

const _VERIFY_CRATE_NAME: () = {
    let path = core::module_path!().as_bytes();
    if !matches!(path, b"prudent") {
        panic!(
            "Do NOT rename `prudent` crate. That is not possible because of rust-lang/rust#110613."
        );
    }
};
