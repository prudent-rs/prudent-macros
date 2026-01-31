/*#![cfg_attr(
    feature = "assert_unsafe_methods",
    doc(test(attr(feature(type_alias_impl_trait))))
)]*/
#![doc(test(attr(forbid(unexpected_cfgs))))]
#![doc(test(attr(deny(unused, dead_code))))]
// @TODO check if still needed:
// Workaround for https://github.com/rust-lang/rust/issues/148599
#![doc(test(attr(allow(forbidden_lint_groups))))]
#![doc = include_str!("../README.md")]
//!
//! Implementation notes of macros ARE a part of the documentation. Why?
//! - Users deserve documentation of **how** a macro works, because
//!   - macros are much more difficult to read than Rust non-macro code, and
//!   - macros inject code, so they are not as sandboxed/isolated as non-macro code.
//! - Otherwise it's a pain to edit them/render them in VS Code. Yes, that matters.

#![allow(clippy::useless_attribute)]
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(not(any(doc, test)), no_std)]
#![forbid(unknown_lints)]
#![deny(missing_docs)]
/*
rustdoc lints: https://doc.rust-lang.org/rustdoc/lints.html

rustdoc::missing_doc_code_examples we don't apply here, because it's nightly-only. Instead, we
invoke it by specifying it in `RUSTDOCFLAGS` with `nightly` toolchain. See
.github/workflows/main.yml.

rustdoc::invalid_codeblock_attributes we do NOT warn/deny/forbid here, even though it IS stable.
That's because in `compile_fail` doctests we use error numbers, which are UNSTABLE only. But, to
allow the same doctest also on stable toolchain, we ALLOW it here.

See
https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests.

But, as per that page, error numbers work on `nightly` only. On `stable` they make "code sample
being interpreted as plain text"! So, we have those doctests in separate files, and we include
them twice:
- without the error code, and run as a part of standard doc generation, on `stable`. That makes
  the doctest code formatted on `stable`.
- with the error code, but on an element that has `#[cfg(doctest)]` attribute, so it's NOT a part
  of doc generation, but it is still tested. That way it doesn't matter that `stable` would
  format it as plain text, and it does get tested on `nightly` as a part of CI.

Then on `nightly` we forbid rustdoc::invalid_codeblock_attributes. See
.github/workflows/main.yml.
*/
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

#[cfg(doc)]
extern crate alloc;

pub mod backend;

/// "Frontend" = macros only.
mod frontend;

pub mod prelude;
