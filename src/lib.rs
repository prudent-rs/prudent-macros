// @TODO feature NOT actually needed in the crate itself, only in consumers
//
//#![cfg_attr(feature = "assert_unsafe_methods", feature(type_alias_impl_trait))]

// @TODO e-considr:
//
//#![doc(test(no_crate_inject))]
//
// #![cfg_attr(feature = "assert_unsafe_methods", doc(test(attr = ::prudent::top_header_assert_unsafe_methods!())))]
//
//#![doc(test(attr = ::prudent::top_header_assert_unsafe_methods!()))]
#![cfg_attr(
    feature = "assert_unsafe_methods",
    doc(test(attr(feature(type_alias_impl_trait))))
)]
#![allow(clippy::useless_attribute)]
#![allow(clippy::needless_doctest_main)]
//! # Examples
#![doc  = internal_coverage_positive!() ]
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
// invoke it by specifying it in `RUSTDOCFLAGS` with `nightly` toolchain. See
// .github/workflows/main.yml.
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
#![doc(test(attr(deny(unused, dead_code))))]
// @TODO check if still needed:
// Workaround for https://github.com/rust-lang/rust/issues/148599
#![doc(test(attr(allow(forbidden_lint_groups))))]

#[cfg(doc)]
extern crate alloc;

#[cfg(feature = "assert_unsafe_methods")]
/// Enable a necessary nightly feature IF prudent is configured to use it.
#[macro_export]
macro_rules! top_header_assert_unsafe_methods {
    () => {
        "#![feature(type_alias_impl_trait)]"
    };
}
#[cfg(not(feature = "assert_unsafe_methods"))]
/// Enable a necessary nightly feature IF prudent is configured to use it.
#[macro_export]
macro_rules! top_header_assert_unsafe_methods {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! internal_coverage_positive {
    (
    ) => {
        $crate::internal_coverage_positive!(
            "# unsafe_fn" -> "../coverage_positive/fn.rs",
            "# unsafe_method\n## unsafe_method > self: shared reference" -> "../coverage_positive/md-shared_ref.rs"
        )
    };
    (
        $( $description:literal -> $file:literal ),*
    ) => {
        ::core::concat!(
        $(
            $description,
            "\n```\n",
            ::core::include_str!($file),
            // just in case the file doesn't end with a new line, inject it anyway:
            "\n```\n",
        )*
        "\n"
        )
    };
}

pub mod backend;

/// "Frontend" macros.
mod frontend;

#[doc(hidden)]
pub mod frontend_with_compile_fail_tests;
