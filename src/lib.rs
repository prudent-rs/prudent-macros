#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test)), no_std)]
#![forbid(unknown_lints)]
// We can't `#![forbid(dead_code)]`, because we use `#[allow(unused_unsafe)]`. Without that
// unsafe_method! existed only as multiple specialized macros: unsafe_method_ref!,
// unsafe_method_mut!... And there were problems with unintended duplicates of Copy `self` when
// invoking methods with the receiver being &self, that is, a shared reference.
#![deny(unused)]
#![forbid(dead_code)]
// docs
#![forbid(missing_docs)]
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
#![doc(test(attr(forbid(unused, dead_code))))]
// Workaround for https://github.com/rust-lang/rust/issues/148599
#![doc(test(attr(allow(forbidden_lint_groups))))]

#[cfg(doc)]
extern crate alloc;

/// For casting/ensuring that a user-provided function is unsafe. Used by `unsafe_fn`.
///
/// Internal - NOT a part of public API.
#[doc(hidden)]
#[allow(clippy::module_inception)]
pub mod expecting_unsafe_fn {
    /// For casting/ensuring that a user-provided function is unsafe. Used by `unsafe_fn`.
    pub unsafe fn fun<R>() -> R {
        unreachable!()
    }
    /// Function with one argument.
    pub mod arg {
        /// Used by `unsafe_fn`.
        pub unsafe fn fun<A1, R>(_: A1) -> R {
            unreachable!()
        }

        /// Two arguments.
        #[allow(clippy::module_inception)]
        pub mod arg {
            #[allow(clippy::module_inception)]
            /// Used by `unsafe_fn`.
            pub unsafe fn fun<A1, A2, R>(_: A1, _: A2) -> R {
                unreachable!()
            }

            /// Three arguments.
            #[allow(clippy::module_inception)]
            pub mod arg {
                #[allow(clippy::module_inception)]
                /// Used by `unsafe_fn`.
                pub unsafe fn fun<A1, A2, A3, R>(_: A1, _: A2, _: A3) -> R {
                    unreachable!()
                }

                /// Four arguments.
                #[allow(clippy::module_inception)]
                pub mod arg {
                    #[allow(clippy::module_inception)]
                    /// Used by `unsafe_fn`.
                    pub unsafe fn fun<A1, A2, A3, A4, R>(_: A1, _: A2, _: A3, _: A4) -> R {
                        unreachable!()
                    }

                    /// Five arguments.
                    #[allow(clippy::module_inception)]
                    pub mod arg {
                        /// Used by `unsafe_fn`.
                        pub unsafe fn fun<A1, A2, A3, A4, A5, R>(
                            _: A1,
                            _: A2,
                            _: A3,
                            _: A4,
                            _: A5,
                        ) -> R {
                            unreachable!()
                        }

                        /// Six arguments.
                        #[allow(clippy::module_inception)]
                        pub mod arg {
                            /// Used by `unsafe_fn`.
                            pub unsafe fn fun<A1, A2, A3, A4, A5, A6, R>(
                                _: A1,
                                _: A2,
                                _: A3,
                                _: A4,
                                _: A5,
                                _: A6,
                            ) -> R {
                                unreachable!()
                            }

                            /// Seven arguments.
                            #[allow(clippy::module_inception)]
                            pub mod arg {
                                /// Used by `unsafe_fn`.
                                pub unsafe fn fun<A1, A2, A3, A4, A5, A6, A7, R>(
                                    _: A1,
                                    _: A2,
                                    _: A3,
                                    _: A4,
                                    _: A5,
                                    _: A6,
                                    _: A7,
                                ) -> R {
                                    unreachable!()
                                }

                                /// Eight arguments.
                                #[allow(clippy::module_inception)]
                                pub mod arg {
                                    /// Used by `unsafe_fn`.
                                    #[allow(clippy::too_many_arguments)]
                                    pub unsafe fn fun<A1, A2, A3, A4, A5, A6, A7, A8, R>(
                                        _: A1,
                                        _: A2,
                                        _: A3,
                                        _: A4,
                                        _: A5,
                                        _: A6,
                                        _: A7,
                                        _: A8,
                                    ) -> R {
                                        unreachable!()
                                    }

                                    /// Nine arguments.
                                    #[allow(clippy::module_inception)]
                                    pub mod arg {
                                        /// Used by `unsafe_fn`.
                                        #[allow(clippy::too_many_arguments)]
                                        pub unsafe fn fun<A1, A2, A3, A4, A5, A6, A7, A8, A9, R>(
                                            _: A1,
                                            _: A2,
                                            _: A3,
                                            _: A4,
                                            _: A5,
                                            _: A6,
                                            _: A7,
                                            _: A8,
                                            _: A9,
                                        ) -> R {
                                            unreachable!()
                                        }

                                        /// Ten arguments.
                                        #[allow(clippy::module_inception)]
                                        pub mod arg {
                                            /// Used by `unsafe_fn`.
                                            #[allow(clippy::too_many_arguments)]
                                            pub unsafe fn fun<
                                                A1,
                                                A2,
                                                A3,
                                                A4,
                                                A5,
                                                A6,
                                                A7,
                                                A8,
                                                A9,
                                                A10,
                                                R,
                                            >(
                                                _: A1,
                                                _: A2,
                                                _: A3,
                                                _: A4,
                                                _: A5,
                                                _: A6,
                                                _: A7,
                                                _: A8,
                                                _: A9,
                                                _: A10,
                                            ) -> R {
                                                unreachable!()
                                            }

                                            /// Eleven arguments.
                                            #[allow(clippy::module_inception)]
                                            pub mod arg {
                                                /// Used by `unsafe_fn`.
                                                #[allow(clippy::too_many_arguments)]
                                                pub unsafe fn fun<
                                                    A1,
                                                    A2,
                                                    A3,
                                                    A4,
                                                    A5,
                                                    A6,
                                                    A7,
                                                    A8,
                                                    A9,
                                                    A10,
                                                    A11,
                                                    R,
                                                >(
                                                    _: A1,
                                                    _: A2,
                                                    _: A3,
                                                    _: A4,
                                                    _: A5,
                                                    _: A6,
                                                    _: A7,
                                                    _: A8,
                                                    _: A9,
                                                    _: A10,
                                                    _: A11,
                                                ) -> R
                                                {
                                                    unreachable!()
                                                }
                                                /// Twelve arguments.
                                                #[allow(clippy::module_inception)]
                                                pub mod arg {
                                                    /// Used by `unsafe_fn`.
                                                    #[allow(clippy::too_many_arguments)]
                                                    pub unsafe fn fun<
                                                        A1,
                                                        A2,
                                                        A3,
                                                        A4,
                                                        A5,
                                                        A6,
                                                        A7,
                                                        A8,
                                                        A9,
                                                        A10,
                                                        A11,
                                                        A12,
                                                        R,
                                                    >(
                                                        _: A1,
                                                        _: A2,
                                                        _: A3,
                                                        _: A4,
                                                        _: A5,
                                                        _: A6,
                                                        _: A7,
                                                        _: A8,
                                                        _: A9,
                                                        _: A10,
                                                        _: A11,
                                                        _: A12,
                                                    ) -> R
                                                    {
                                                        unreachable!()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generate path to `fun` under [expecting_unsafe_fn::arg], or [expecting_unsafe_fn::arg::arg], or
/// [expecting_unsafe_fn::arg::arg::arg] etc, as appropriate for given number of argument(s).
///
/// Internal - NOT a part of public API.
#[macro_export]
#[doc(hidden)]
macro_rules! expecting_unsafe_fn_path {
    ( $( $arg:expr ),+ ) => {
        $crate::expecting_unsafe_fn_path!( ~ { $( $arg ),+ }, $crate::expecting_unsafe_fn )
    };
    ( ~ { $arg_first:expr, $( $arg_rest:expr ),+ }, $( $path_part:tt )+ ) => {
        $crate::expecting_unsafe_fn_path!( ~ { $( $arg_rest ),+ }, $( $path_part )+ ::arg )
    };
    ( ~ { $arg_last:expr }, $( $path_part:tt )+ ) => {
        $( $path_part )+ ::arg::fun
    };
}

// Implementation notes ARE a part of the documentation:
// - Otherwise it's a pain to edit them.
// - Users deserve documentation of how a macro works, because
//   - macros are much more difficult to read than Rust non-macro code, and
//   - macros inject code.
//
/// Invoke an `unsafe` function, but isolate `unsafe {...}` only for the function invocation itself.
/// - If `$fn`, that is, the function itself, is NOT given as an identifier/qualified path, but it's
///   given as an expression, then this expression is treated as if evaluated **outside** `unsafe
///   {...}`.
/// - Any arguments passed in as expressions are treated as if evaluated **outside** `unsafe {...}`.
///
/// There is **no** extra enclosing pair of parenthesis `(...)` around the list of arguments (if
/// any). If there was such a pair, it could be confused for a tuple. It would also be less readable
/// when some parameters were tuples/complex expressions.
///
/// This does NOT accept closures, since, closures cannot be `unsafe`.
///
/// # Possible violations
///
/// Zero arguments. The given expression (which evaluates to the function to be called) is `unsafe.`
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
/// Some arguments. The given expression (which evaluates to the function to be called) is `unsafe.`
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
/// A passed parameter (expression that evaluates to a value passed to the target `unsafe` function as an argument) itself is `unsafe.`
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
/// The target function is safe, hence no need for `unsafe_fn`. Zero args.
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
/// The target function is safe, hence no need for `unsafe_fn`. Some args.
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
/// Use the result of `unsafe_fn!` immediately as an array/slice.
/// ```
/// # use prudent::unsafe_fn;
/// unsafe fn return_array() -> [bool; 1] { [true] }
///
/// let _ = unsafe_fn!( return_array)[0];
/// ```
/// Use the result of `unsafe_fn!` immediately as a mutable array/slice (assign/modify its slot(s)).
/// ```
/// # use prudent::unsafe_fn;
/// // NOT running under MIRI, because of the intentional leak.
/// if !cfg!(miri) {
///     unsafe fn return_mut_ref_array() -> &'static mut [bool; 1] {
///         let boxed = Box::new([true]);
///         Box::leak(boxed)
///     }
///
///     unsafe_fn!( return_mut_ref_array)[0] = true;
/// }
/// ```
/// The same, but without a leak:
/// ```
/// # use prudent::unsafe_fn;
/// unsafe fn return_same_mut_ref<T>(mref: &mut T) -> &mut T {
///    mref
/// }
///
/// let mut marray = [true];
/// unsafe_fn!( return_same_mut_ref, &mut marray )[0] = true;
/// ```
/// TODO docs about tuple tree
#[macro_export]
macro_rules! unsafe_fn {
    /*( $fn:expr $(, $arg:expr)* ) => {
        (
            if false {
                #[deny(unused_unsafe)]
                let _ = $fn;
                $(
                    #[deny(unused_unsafe)]
                    let _ = $arg;
                )*
                unreachable!()
            } else {
                #[allow(unsafe_code)]
                unsafe {
                    ( $fn )( $( $arg ),* )
                }
            }
        )
    };*/
    ( $fn:expr $(, $arg:expr)+ ) => {
        // Enclosed in (...) and NOT in {...}. Why? Because the later does NOT work if the result is
        // an array/slice and then it's indexed with array access suffix [usize_idx].
        (
            // Enclosed in a block, so that
            // 1. the result can be used as a value in an outer expression, and
            // 2. local variables don't conflict with the outer scope
            {
                // Ensure that $fn (the expression itself, one that yields a function to call) and
                // any arguments (expressions that yield values passed to the function to call)
                // don't include any unnecessary `unsafe{...}` block(s):
                #[deny(unused_unsafe)]
                // Ensure that $fn (the expression itself) and any arguments (expressions) don't
                // include any unsafe code/calls/casts on their  own without their own `unsafe{...}`
                // block(s):
                let (tuple_tree, fun) = ($crate::unsafe_fn_internal_build_tuple_tree!{ $($arg),+ }, $fn);

                if false {
                    // Ensure that $fn is not safe, but `unsafe`. Using
                    // https://doc.rust-lang.org/reference/types/function-item.html#r-type.fn-item.coercion
                    let _ = if false {
                        $crate::expecting_unsafe_fn_path!( $( $arg ),+ )
                        //unreachable!()
                    } else {
                        fun
                    };
                    unreachable!()
                }
                $crate::unsafe_fn_internal_build_accessors_and_call! {
                    fun,
                    tuple_tree,
                    ( $( $arg ),* ),
                    (0)
                }
            }
        )
    };
    ($fn:expr) => {
        ({
            // Ensure that $fn (the expression itself, one that yields a function to call)
            // doesn't include an unnecessary `unsafe{...}` block:
            #[deny(unused_unsafe)]
            // Ensure that $fn (the expression itself) doesn't include any unsafe code/calls/casts
            // on its own without its own `unsafe{...}` block(s):
            let fun = $fn;
            if false {
                // Ensure that $fn is not safe, but `unsafe`. Using
                // https://doc.rust-lang.org/reference/types/function-item.html#r-type.fn-item.coercion
                let _ = if false {
                    $crate::expecting_unsafe_fn::fun
                } else {
                    fun
                };
                unreachable!()
            }
            // `#[deny(unused_unsafe)]` does NOT work here. Why? Because when we assigned `let
            // fun = $fn` above, that then happily coerces/infers to an unsafe function, even
            // though it's safe. That's why we have `unsafe_fun` module.
            #[allow(unsafe_code)]
            let result = unsafe {
                fun()
            };
            result
        })
    };
}

// Same `compile_fail` tests as listed above for `unsafe_fn`, but here we validate the error
// numbers.
//
// Error numbers are validated with `cargo +nightly test`, but NOT with
// - `cargo +stable test` nor
// - RUSTDOCFLAGS="..." cargo +nightly doc ...
//
// Even though the following constant is "pub", it will **not** be a part of the public API, neither
// a part of the documentation - it's used for doctest only.
/// ```compile_fail,E0133
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0308
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_fn_internal_build_tuple_tree {
    // Construct the tuple_tree. Recursive:
    ( $first:expr, $($rest:expr),+ ) => {
        (
            $first, $crate::unsafe_fn_internal_build_tuple_tree!{ $($rest),+ }
        )
    };
    ( $last:expr) => {
        ($last,)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_fn_internal_build_accessors_and_call {
    // Access tuple_tree parts and get ready to call the function:
    ( $fn:expr, $tuple_tree:ident,
     ( $_first_arg:expr, $($other_arg:expr),+ ),
     $( ( $($accessor_part:tt),+
        )
     ),*
    ) => {
        $crate::unsafe_fn_internal_build_accessors_and_call!{
            $fn, $tuple_tree, ( $($other_arg),+ ),
            // Insert a new accessor to front (left): 0.
            (0),
            $(  // Prepend 1 to each supplied/existing accessor
                 ( 1, $($accessor_part),+ )
            ),*
        }
    };
    // All accessors are ready, so call the function:
    ( $fn:expr, $tuple_tree:ident,
      ( $_last_or_only_arg:expr ),
      $( ( $($accessor_part:tt),+
         )
      ),*
    ) => {
        #[allow(unsafe_code)]
        #[deny(unused_unsafe)]
        unsafe {
            $fn( $(
                    $crate::unsafe_fn_internal_access_tuple_tree_field!{ $tuple_tree, $($accessor_part),+ }
                ),*
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
/// INTERNAL. Do NOT use directly - subject to change.
///
/// Expand an accessor group/list to access a field in the tuple_tree.
macro_rules! unsafe_fn_internal_access_tuple_tree_field {
    ( $tuple_tree:ident, $($accessor_part:tt),* ) => {
        $tuple_tree $(. $accessor_part )*
    };
}
//-------------

/// NOT a part of public API. Pretend to get a mutable reference from a shared reference. For
/// internal/generated **compile-time** checks only.
#[doc(hidden)]
pub const fn shared_to_mut<T>(_: &T) -> &mut T {
    unreachable!()
}

/// Invoke an `unsafe` method. Like [unsafe_fn], but
/// - This accepts a receiver `&self`, `&mut self` and `self`. TODO Box/Rc/Arc, dyn?
/// - This treats `self` as if it were evaluated **outside** the `unsafe {...}` block.
/// - $fn can **NOT** be an expression or a qualified path (which doesn't work in standard methods
///   calls anyways), but only an identifier.
///
/// Do NOT use parameters/input parts matched by
/// - `$expect_unsafe_empty_indicator` or
/// - `$allow_unsafe_empty_indicator`
///
/// as they are internal.
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
/// ```compile_fail
///  #![allow(clippy::needless_doctest_main)]
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
#[macro_export]
macro_rules! unsafe_method {
    (
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $self:expr, $fn:ident $(, $arg:expr )*
     ) => {
        $crate::unsafe_method_internal_check_self_etc!(
            $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
            $self, $fn $(, $arg )*
        )
     };
    (
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $self:expr, $fn:ident $(, $arg:expr )*
     ) => {
        $crate::unsafe_method_internal_check_self_etc!(
            $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
            $self, $fn $(, $arg )*
        )
     };
}

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_method_internal_check_self_etc {
    (
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $self:expr, $fn:ident $(, $arg:expr )*
     ) => {
        // See unsafe_fn for why here we enclose in (...) and not in {...}.
        (
            if false {
                if false {
                    // This block "makes" owned_receiver, an instance/owned value of the same type
                    // as $self. (Of course, the instance is invalid - this is for compile-time
                    // checks only, hence `if false {...}`.)
                    //
                    // Then we simulate invocation of the given method inside `unsafe {...}``, BUT
                    // without evaluating the given $self expression inside that same `unsafe
                    // {...}`` block, so that we isolate/catch any `unsafe`` code in $self.
                    //
                    // We **cannot** just move/take/assign $self by value, in case it's a non-Copy
                    // **static** variable.
                    let rref = {
                        #[rustfmt::skip]
                        #[deny(unused_unsafe)]
                        // @TODO simplify once https://github.com/rust-lang/rust/issues/15701
                        // (attributes on expressions)
                        #[deny(unsafe_code)]
                        $(
                            $( { $allow_unsafe_empty_indicator } )?
                            #[allow(unsafe_code)]
                        )?
                        $(
                            $( { $expect_unsafe_empty_indicator } )?
                            #[expect(unsafe_code)]
                        )?
                        let rref = &( $self );
                        rref
                    };
                    //
                    let mref = $crate::shared_to_mut(rref);
                    let mut owned_receiver = ::core::mem::replace(mref, unsafe{ ::core::mem::zeroed() });
                    // Detect code where unsafe_fn! or unsafe_method! is not needed at all. That is,
                    // where a function/method used to be `unsafe`, but it stopped being so.
                    #[deny(unused_unsafe)]
                    let _ = unsafe { owned_receiver. $fn( $( $arg ),* ) };
                } else {
                    // @TODO eliminate
                    $(
                        #[deny(unused_unsafe)]
                        let _ = $arg;
                    )*
                }
                unreachable!()
            } else {
                //compile_error!("TODO move to unsafe_method_internal_check_args_etc");
                $crate::unsafe_method_internal_check_args_etc!(
                    $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
                    $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
                    $self, $fn $(, $arg )*
                )
                /*#[allow(unsafe_code)]
                // Notify if $self includes `unsafe {...}`, but no ~allow_unsafe or ~expect_unsafe:
                #[deny(unused_unsafe)]
                $(
                    $( { $allow_unsafe_empty_indicator } )?
                    #[allow(unused_unsafe)]
                )?
                $(
                    $( { $expect_unsafe_empty_indicator } )?
                    #[expect(unused_unsafe)]
                )?
                unsafe { $self. $fn ( $( $arg ),* ) }*/
            }
        )
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_method_internal_check_args_etc {
    (
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $self:expr, $fn:ident $(, $arg:expr )+
     ) => {({
                let tuple_tree =
                    $crate::unsafe_fn_internal_build_tuple_tree!{ $($arg),+ };

                $crate::unsafe_method_internal_build_accessors_check_args_call! {
                    $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
                    $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
                    $self,
                    $fn,
                    tuple_tree,
                    ( $( $arg ),* ),
                    (0)
                }
    })};
    (
        $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
        $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
        $self:expr, $fn:ident
     ) => {({
                #[allow(unsafe_code)]
                // Notify if $self includes `unsafe {...}`, but no ~allow_unsafe or ~expect_unsafe:
                #[deny(unused_unsafe)]
                $(
                    $( { $allow_unsafe_empty_indicator } )?
                    #[allow(unused_unsafe)]
                )?
                $(
                    $( { $expect_unsafe_empty_indicator } )?
                    #[expect(unused_unsafe)]
                )?
                let result = unsafe { $self. $fn () };
                result
    })};
}

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_method_internal_build_accessors_check_args_call {
    // Access tuple_tree parts and get ready to call the method:
    (
     $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
     $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
     $self:expr, $fn:ident, $tuple_tree:ident,
     ( $_first_arg:expr, $($other_arg:expr),+ ),
     $( ( $($accessor_part:tt),+
        )
     ),*
    ) => {
        $crate::unsafe_method_internal_build_accessors_check_args_call!{
            $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator  } )? )?
            $( ~expect_unsafe  $( { $expect_unsafe_empty_indicator  } )? )?
            $self, $fn, $tuple_tree, ( $($other_arg),+ ),
            // Insert a new accessor to front (left): 0.
            (0),
            $(  // Prepend 1 to each supplied/existing accessor
                 ( 1, $($accessor_part),+ )
            ),*
        }
    };
    // All accessors are ready. Call the function:
    (
     $( ~expect_unsafe $( { $expect_unsafe_empty_indicator:tt } )? )?
     $( ~allow_unsafe  $( { $allow_unsafe_empty_indicator:tt  } )? )?
     $self:expr, $fn:ident, $tuple_tree:ident,
      ( $_last_or_only_arg:expr ),
      $( ( $($accessor_part:tt),+
         )
      ),*
    ) => {({
        #[allow(unsafe_code)]
        #[deny(unused_unsafe)]
        $(
            $( { $allow_unsafe_empty_indicator } )?
            #[allow(unused_unsafe)]
        )?
        $(
            $( { $expect_unsafe_empty_indicator } )?
            #[expect(unused_unsafe)]
        )?
        let result = unsafe {
            $self. $fn( $(
                    $crate::unsafe_fn_internal_access_tuple_tree_field!{ $tuple_tree, $($accessor_part),+ }
                ),*
            )
        };
        result
    })};
}
//-------------

/// Set a value of a `static mut` variable or its (sub...-)field, but isolate `unsafe {...}` only to
/// that assignment.
///
/// To minimize unintended `unsafe`, calculate any indexes etc. beforehand, store them in local
/// variables and pass them in.
///
/// We do **not** have a similar macro to get a value of a `static mut`. For that, simply enclose it
/// in `unsafe{...}`.
///
/// TODO:
///
/// NOT for `static` variables (or their fields/components) of `union` types.
/// ```
/// {
///     static mut S: (bool,) = (true,);
///
///     let mptr = &raw mut S;
///     unsafe { *mptr = (false,); }
///
///     let _mref = unsafe {&mut *mptr};
///     
///     // The following IS accepted:
///     //
///     //{unsafe {&mut *mptr}}.0 = true;
///     //
///     // BUT, because the outer curly brackets {...} are **refused** just left of
///     // [index_here] when indexing arrays (see below), we use oval parenthesis (...)
///     // which work for both: the tuple access .usize_literal and for array access
///     // [usize_expression].
/// }
/// {
///     static mut ARR: [bool; 1] = [true];
///     let mptr = &raw mut ARR;
///     unsafe { *mptr = [false]; }
///
///     let _mref = unsafe {&mut *mptr};
///     *_mref = [false];
///     _mref[ 0 ] = true;
///     
///     // Read access OK:
///     let _b: bool = { unsafe {&mut *mptr} }[ 0 ];
///     // Mut access - bad: The following refused:
///     //
///     //{ unsafe {&mut *mptr} }[ 0 ] = true;
///     //
///     // Have to use oval parenthesis:
///     ( unsafe {&mut *mptr} )[ 0 ] = true;
/// }
/// ```
#[macro_export]
macro_rules! unsafe_static_set {
    ($static:path, $val:expr) => {{
        if false {
            let _ = $val;
            unreachable!()
        } else {
            #[allow(unsafe_code)]
            unsafe {
                $static = $val;
            }
        }
    }};
    // @TODO implement + rename, so it's for union fields, too:
    //
    // @TODO similar to read union fields
    ($static:ident { $( $suffix:tt )* } $val:expr) => {{}};
    ($static:path { $( $suffix:tt )* } $val:expr) => {{
        if false {
            let mptr = &raw mut $static;
            let mref = unsafe { &mut *mptr };
            unreachable!()
        } else {
        }
    }};
}

/// Deref a pointer (either `const` or `mut`) and yield a read-only reference.
///
/// If `$type` is given, it's expected to be the referenced type (NOT the given pointer, NOT a
/// reference based on the given pointer), and the given pointer is cast to `* const $type`. `$type`
/// may start with `dyn`. `$type` may be a slice `[...]`.
#[macro_export]
macro_rules! unsafe_ref {
    ($ptr:expr) => {{
        let ptr: *const _ = $ptr;
        unsafe { &*ptr }
    }};
    ($ptr:expr, $lifetime:lifetime) => {{
        let ptr: *const _ = $ptr;
        unsafe { &*ptr as &$lifetime _ }
    }};
    ($ptr:expr, $type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $type;
        unsafe { &*ptr }
    }};
    ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $ptr_type;
        unsafe { &*ptr as &$lifetime _ }
    }};
}

/// Deref a `mut` pointer and yield a `mut` reference.
///
/// Like for [unsafe_ref]: If `$type` is given, it's expected to be the referenced type (NOT the
/// given pointer, NOT the target reference type) and the given pointer is cast to `* const $type`.
/// `$type` may start with `dyn`. `$type` may be a slice `[...]`.
#[macro_export]
macro_rules! unsafe_mut {
    ($ptr:expr) => {{
        let ptr: *mut _ = $ptr;
        unsafe { &mut *ptr }
    }};
    ($ptr:expr, $lifetime:lifetime) => {{
        let ptr: *mut _ = $ptr;
        unsafe { &mut *ptr as &$lifetime mut _}
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *mut $ptr_type;
        unsafe { &mut *ptr}
    }};
    ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {{
        let ptr = $ptr;
        let ptr = ptr as *mut $ptr_type;
        unsafe { &mut *ptr as &$lifetime mut _}
    }};
}

/// This is an "early" type check for [unsafe_val], so that the user knows to use [unsafe_val] with [core::marker::Copy] types only.
///
/// NOT a part of public API!
#[doc(hidden)]
pub const fn expect_copy_ptr<T: Copy>(_: *const T) {}

/// Get a (copy of) value from where the pointer points. For [core::marker::Copy] types only.
#[macro_export]
macro_rules! unsafe_val {
    ($ptr:expr) => {{
        let ptr: *const _ = $ptr;
        $crate::expect_copy_ptr(ptr);
        unsafe { *ptr }
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $ptr_type;
        $crate::expect_copy_ptr(ptr);
        unsafe { *ptr }
    }};
}

/*
-nightly version only
https://doc.rust-lang.org/std/keyword.use.html#ergonomic-clones
https://doc.rust-lang.org/std/clone/trait.UseCloned.html


#[macro_export]
macro_rules! unsafe_use {
    ($ptr:expr) => {{
        let ptr = $ptr;
        unsafe { ( *ptr ).use }
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr as $ptr_type;
        unsafe { ( *ptr ).use }
    }};
}*/

/// Assign the given value to the location given in the pointer.
///
/// Needed, because we can't isolate:
///
/// `unsafe { *ptr } = value;`
///
/// Also, we can't have a macro invocation on the left side (target) of an assignment operator `=`,
/// so nothing like:
///
/// `unsafe_set!( pt ) = false;`
#[macro_export]
macro_rules! unsafe_set {
    ($ptr:expr, $value:expr) => {{
        if false {
            let _: *mut _ = $ptr;
            let _ = $value;
            unreachable!()
        } else {
            #[allow(unsafe_code)]
            unsafe {
                *$ptr = $value;
            }
        }
    }};
}
