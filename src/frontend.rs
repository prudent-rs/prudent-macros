/// Invoke an `unsafe` function, but isolate `unsafe {...}` only for the function invocation itself.
/// - If `$fn` (the function itself) is NOT given as an identifier/qualified path, but it's given as
///   an expression, then this expression is treated as if evaluated **outside** `unsafe {...}`.
/// - Any arguments passed in as expressions are treated as if evaluated **outside** `unsafe {...}`.
///
/// There is **no** extra enclosing pair of parenthesis `(...)` around the list of arguments (if
/// any). If there was such a pair, it could be confused for a tuple. It would also be less readable
/// when some parameters were tuples/complex expressions.
///
/// This does NOT accept closures, since closures cannot be `unsafe`.
///
/// # Possible violations
/// - Zero arguments. The given expression (which evaluates to the function to be called) is
///   `unsafe.`
/// - Some arguments. The given expression (which evaluates to the function to be called) is
///   `unsafe.`
/// ```compile_fail
/// // @TODO Docs: at your crate's top level, use either self::prudent, or crate:;prudent (but NOT
/// // just prudent, which will fail, fortunately).
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
///
/// ## Some arguments
/// The given expression (which evaluates to the function to be called) is `unsafe.`
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
///
/// A passed parameter (expression that evaluates to a value passed to the target `unsafe` function as an argument) itself is `unsafe.`
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
///
/// The target function is safe, hence no need for `unsafe_fn`. Zero args.
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
///
/// The target function is safe, hence no need for `unsafe_fn`. Some args.
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
/// test cfg test:
/// ```test_harness
/// // test_harness -as per https://github.com/rust-lang/rust/issues/148942#issuecomment-3565011334
/// #[cfg(not(test))]
/// compile_error!("NOT DOCTEST!");
/// ```
/// Use the result of `unsafe_fn!` immediately as an array/slice:
/// ```test_harness
/// //TODO? failing??
/// use prudent::*;
/// const unsafe fn return_array() -> [bool; 1] { [true] }
///
/// const _: bool = unsafe_fn!( return_array)[0];
/// ```
/// Use the result of `unsafe_fn!` immediately as a mutable array/slice (assign/modify its slot(s)):
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// use prudent::*;
/// fn _test_unsafe_fn_returning_mut_ref() {
///     // NOT running under MIRI, because of an intentional leak.
///     if !cfg!(miri) {
///         unsafe fn return_mut_ref_array() -> &'static mut [bool; 1] {
///             let boxed = Box::new([true]);
///              Box::leak(boxed)
///         }
///
///     unsafe_fn!( return_mut_ref_array)[0] = true;
///     }
/// }
/// fn main() {}
/// ```
/// The same, but the function takes an argument (and no leak):
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// use prudent::*;
/// unsafe fn return_same_mut_ref<T>(mref: &mut T) -> &mut T {
///    mref
/// }
///
/// fn main() {
///     let mut marray = [true];
///     unsafe_fn!( return_same_mut_ref => &mut marray )[0] = true;
/// }
/// ```
#[macro_export]
macro_rules! unsafe_fn {
    ( $fn:expr => $( $arg:expr ),+ ) => {

        // Enclosed in (...) and NOT in {...}. Why? Because the later does NOT work if the result is
        // an array/slice and then it's indexed with array access suffix [usize_idx].
        (
            // Enclosed in a block, so that
            // 1. the result can be used as a value in an outer expression, and
            // 2. local variables don't conflict with the outer scope
            {
                // Ensure that
                // - $fn (the expression itself, one that yields the function to call) and
                // - any arguments (expressions that yield values passed to the function to call)
                //
                // don't include any unnecessary `unsafe{...}` block(s):
                //
                // @TODO remove this #[deny(unused_unsafe)] ??? $fn or any of $arg could be a rexult
                // of unsafe_method!(...) that itself MAY have "unused_unsafe" in $self!!!
                #[deny(unused_unsafe)]
                // Ensure that $fn (the expression itself) and any arguments (expressions) don't
                // include any unsafe code/calls/casts on their  own without their own `unsafe{...}`
                // block(s).
                let (tuple_tree, fun) = ($crate::unsafe_fn_internal_build_tuple_tree!{ $($arg),+ }, $fn);

                if false {
                    // Detect code where `unsafe_fn!` is not needed at all. Maybe the function used
                    // to be `unsafe`, but not anymore.
                    //
                    // Ensure that $fn is not safe, but `unsafe`. Using
                    // https://doc.rust-lang.org/reference/types/function-item.html#r-type.fn-item.coercion
                    //
                    // We can't just use
                    // ```
                    // let _: unsafe fn(_, _,... ) -> _ = fun;
                    // ```
                    // (with the appropriate number of _ for arguments), because that would coerce a
                    // safe function into unsafe, and we would lose the ability to verify that it's
                    // indeed unsafe!
                    let _ = if false {
                        $crate::expecting_unsafe_fn_path!( $( $arg ),+ )
                    } else {
                        fun
                    };
                    ::core::unreachable!()
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
            // Ensure that $fn (the expression itself, one that yields a function to call) doesn't
            // include an unnecessary `unsafe{...}` block:
            //
            // @TODO remove this #[deny(unused_unsafe)]
            #[deny(unused_unsafe)]
            // Ensure that $fn (the expression itself) doesn't include any unsafe code/calls/casts
            // on its own without its own `unsafe{...}` block(s):
            let fun = $fn;
            if false {
                // Ensure that $fn is not safe, but `unsafe`. Using
                // https://doc.rust-lang.org/reference/types/function-item.html#r-type.fn-item.coercion
                let _ = if false {
                    $crate::backend::expecting_unsafe_fn::fun
                } else {
                    fun
                };
                ::core::unreachable!()
            }
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
// Error numbers are validated with `cargo +nightly test`, ([The Rustdoc book > Unstable features >
// Error numbers for compile-fail
// doctests](https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests))
// but NOT with
// - `cargo +stable test` nor
// - RUSTDOCFLAGS="..." cargo +nightly doc ...
//
// Even though the following constant is "pub", it will **not** be a part of the public API, neither
// a part of the documentation - it's used for doctest only.
/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/fn_expr_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_fn/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0308
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0308
#[doc = include_str!("../violations_coverage/unsafe_fn/fn_unused_unsafe/some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};
//----------------------

/// INTERNAL. Do NOT use directly - subject to change.
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

/// INTERNAL. Do NOT use directly - subject to change.
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
            // Insert a new accessor to the front (left): 0.
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
        // @TODO remove this #[deny(unused_unsafe)]
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

/// Invoke an `unsafe` method. For methods that have a receiver parameter (`&self`, `&mut self`,
/// `self`). For associated functions (implemented for a type but with no receiver) use `unsafe_fn`,
/// and pass the qualified name of the associated function to it.
///
/// Like [unsafe_fn], but
/// - This accepts a receiver `&self`, `&mut self` and `self`. TODO Box/Rc/Arc, dyn?
/// - This treats `self` as if it were evaluated **outside** the `unsafe {...}` block.
/// - $fn can **NOT** be an expression or a qualified path (which doesn't work in standard methods
///   calls anyways), but only an identifier.
///
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
///
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
///
/// ```compile_fail
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
// TODO refactor for new checks:
// ```compile_fail
//#[doc = include_str!("../violations_coverage/unsafe_method/fn_unused_unsafe/zero_args.rs")]
// ```
//
//#[allow(clippy::useless_attribute)]
//#[allow(clippy::needless_doctest_main)]
// ```compile_fail
//#[doc = include_str!("../violations_coverage/unsafe_method/fn_unused_unsafe/some_args.rs")]
// ```
#[macro_export]
macro_rules! unsafe_method {
    (
        $self:expr =>@ $method:ident
     ) => {
        $crate::unsafe_method_check_cfg!(
            $self =>@ $method =>
        )
     };
    (
        $self:expr =>@ $method:ident => $( $arg:expr ),*
     ) => {
        $crate::unsafe_method_check_cfg!(
            $self =>@ $method => $( $arg ),*
        )
    }
}

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/arg.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_zero_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};

/// ```compile_fail,E0133
#[doc = include_str!("../violations_coverage/unsafe_method/sneaked_unsafe/self_some_args.rs")]
/// ```
#[cfg(doctest)]
pub const _: () = {};
//----------------------

#[cfg(not(feature = "assert_unsafe_methods"))]
#[macro_export]
#[doc(hidden)]
macro_rules! unsafe_method_check_cfg {
    (
        $self:expr =>@ $method:ident => $( $arg:expr ),*
     ) => {
        $crate::unsafe_method_assert_unsafe_methods!(
            {}
            $self =>@ $method => $( $arg ),*
        )
     }
}
#[cfg(feature = "assert_unsafe_methods")]
#[macro_export]
#[doc(hidden)]
macro_rules! unsafe_method_check_cfg {
    (
        $self:expr =>@ $method:ident => $( $arg:expr ),*
     ) => {
        $crate::unsafe_method_assert_unsafe_methods!(
            {
                type OwnedReceiver = impl Sized;
                //let _ = move || -> OwnedReceiver { owned_receiver };
                let owned_receiver: OwnedReceiver = owned_receiver;

                // Detect code where `unsafe_method!` is not needed at all. Maybe the method used
                // to be `unsafe`, but not anymore.
                //
                // See unsafe_fn for why we can't just use simple coercion like:
                // ```
                // let _: unsafe fn(_, _,... ) -> _ = OwnedReceiver::$method;
                // ```

                let _ = OwnedReceiver::$method;
                /*let _ = if false {
                    $crate::expecting_unsafe_fn_path!( first_goes_receiver $(, $arg )* )
                } else {
                    OwnedReceiver::$method
                };*/
                ::core::unreachable!();
            }
            $self =>@ $method => $( $arg ),*
        )
     }
}

#[macro_export]
#[doc(hidden)]
macro_rules! unsafe_method_assert_unsafe_methods {
    (
        { $( $code_assert_unsafe_methods:tt )* }
        $self:expr =>@ $method:ident => $( $arg:expr ),*
     ) => {
        // See unsafe_fn for why here we enclose in (...) and not in {...}.
        (
            if false {
                // "Make" an owned_receiver, an instance/owned value of the same type as $self. (Of
                // course, the instance is invalid - this is for compile-time checks only, hence `if
                // false {...}`.)
                //
                // Then we simulate invocation of the given method inside `unsafe {...}`, BUT
                // without evaluating the given $self expression inside that same `unsafe {...}`
                // block, so that we isolate/catch any `unsafe` code in $self.
                //
                // We **cannot** just move/take/assign $self by value, in case it's a non-`Copy`
                // `static` variable (or a deref of a non-`Copy` raw pointer). See also comments in
                // unsafe_method_internal_build_accessors_check_args_call.
                let mref = {
                    let rref = &( $self );
                    $crate::backend::shared_to_mut( rref )
                };
                #[allow(unused_mut)] // in case the method takes &mut self.
                #[allow(invalid_value)] // for &str and other types where zeroed() issues invalid_value warning.
                let mut owned_receiver = ::core::mem::replace(mref, unsafe{ ::core::mem::zeroed() });

                if false {
                    $( $code_assert_unsafe_methods )*
                }
                // @TODO double check and remove:
                //
                // Detect code where `unsafe_method!` is not needed at all. Maybe the method used
                // to be `unsafe`, but not anymore.
                #[deny(unused_unsafe)]
                let _ = unsafe { owned_receiver. $method( $( $arg ),* ) };
                ::core::unreachable!()
            } else {
                $crate::unsafe_method_internal_check_args_etc!(
                    $self, $method $(, $arg )*
                )
            }
        )
     }
}

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_method_internal_check_args_etc {
    (
        $self:expr, $fn:ident $(, $arg:expr )+
     ) => {({
                let tuple_tree =
                    $crate::unsafe_fn_internal_build_tuple_tree!{ $($arg),+ };

                $crate::unsafe_method_internal_build_accessors_check_args_call! {
                    $self,
                    $fn,
                    tuple_tree,
                    ( $( $arg ),* ),
                    (0)
                }
    })};
    (
        $self:expr, $fn:ident
     ) => {({
                #[allow(unsafe_code)]
                let result = unsafe { $self. $fn () };
                result
    })};
}

#[doc(hidden)]
#[macro_export]
macro_rules! unsafe_method_internal_build_accessors_check_args_call {
    // Access tuple_tree parts and get ready to call the method:
    (
     $self:expr, $fn:ident, $tuple_tree:ident,
     ( $_first_arg:expr, $($other_arg:expr),+ ),
     $( ( $($accessor_part:tt),+
        )
     ),*
    ) => {
        $crate::unsafe_method_internal_build_accessors_check_args_call!{
            $self, $fn, $tuple_tree, ( $($other_arg),+ ),
            // Insert a new accessor to the front (left): 0.
            (0),
            $(  // Prepend 1 to each supplied/existing accessor
                 ( 1, $($accessor_part),+ )
            ),*
        }
    };
    // All accessors are ready. Call the function:
    (
     $self:expr, $fn:ident, $tuple_tree:ident,
      ( $_last_or_only_arg:expr ),
      $( ( $($accessor_part:tt),+
         )
      ),*
    ) => {({
        #[allow(unsafe_code)]
        let result = unsafe {
            // Unlike arguments, we can NOT store result of $self expression in a variable, because
            // - it would be moved, but a method with receiver by reference `&self` or `&mut self`
            // does NOT move the instance it's called on. Also,
            // - if Self were `Copy`, then `&self` or `&mut self` reference would not point to the
            //   original instance! (Plus extra stack used, plus lifetimes issues.)
            // - it could be a non-Copy **static** variable, which cannot be moved.
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
/// in `unsafe{...}`. TODO reconsider.
///
/// NOT for `static` variables (or their fields/components) of `union` types.
///
/// ```
/// // @TODO MOVE OUT TO coverage_positive/
/// //use prudent::*;
/// fn main() {
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
/// }
/// ```
#[macro_export]
macro_rules! unsafe_static_set {
    ($static:path, $val:expr) => {{
        if false {
            let _ = $val;
            ::core::unreachable!()
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
            ::core::unreachable!()
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
/// Like in [unsafe_ref]: If `$type` is given, it's expected to be the referenced
/// type (NOT the given pointer, NOT the target reference type) and the given pointer is cast to `*
/// const $type`. `$type` may start with `dyn`. `$type` may be a slice `[...]`.
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

/// Get a (copy of) value from where the pointer points. For [core::marker::Copy] types only.
#[macro_export]
macro_rules! unsafe_val {
    ($ptr:expr) => {{
        let ptr: *const _ = $ptr;
        $crate::backend::expect_copy_ptr(ptr);
        unsafe { *ptr }
    }};
    ($ptr:expr, $ptr_type:ty) => {{
        let ptr = $ptr;
        let ptr = ptr as *const $ptr_type;
        $crate::backend::expect_copy_ptr(ptr);
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
            ::core::unreachable!()
        } else {
            #[allow(unsafe_code)]
            unsafe {
                *$ptr = $value;
            }
        }
    }};
}
