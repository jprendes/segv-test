use std::any::Any;
use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Once;

mod ffi {
    pub(crate) use core::ffi::{c_int, c_void};
    unsafe extern "C" {
        pub(crate) fn segv_init();
        pub(crate) fn check_segv(f: extern "C" fn(*mut c_void), arg: *mut c_void) -> c_int;
    }
}

extern "C" fn call_func(func: *mut ffi::c_void) {
    let func = func as *mut Box<dyn FnMut()>;
    let res = catch_unwind(AssertUnwindSafe(|| {
        unsafe { (*func)() };
    }));
    if let Err(payload) = res {
        SEGV_CAUGHT_PANIC.replace(Some(payload));
    }
}

thread_local! {
    static SEGV_CAUGHT_PANIC: RefCell<Option<Box<dyn Any + Send>>> = RefCell::new(None);
}

#[doc(hidden)]
pub fn check_segv(f: impl FnOnce()) -> bool {
    static SEGV_INIT: Once = Once::new();
    SEGV_INIT.call_once(|| unsafe { ffi::segv_init() });
    let mut f = Some(f);
    let f_box: Box<dyn FnMut()> = Box::new(move || {
        f.take().unwrap()();
    });
    let f_ptr = &f_box as *const _;
    let res = unsafe { ffi::check_segv(call_func, f_ptr as *mut _) };
    match res {
        0 => {
            // No segmentation fault, check if we caught a panic
            if let Some(payload) = SEGV_CAUGHT_PANIC.take() {
                std::panic::resume_unwind(payload);
            }
            false
        }
        1 => true, // Segmentation fault occurred
        2 => panic!("Nested segmentation fault assertion is not supported."),
        _ => unreachable!(),
    }
}

/// Macro to assert that a segmentation fault occurs when executing
/// the given code block.
///
/// If the code block does not cause a segmentation fault, it will panic.
///
/// # Examples
///
/// Reading or writing to an **invalid** pointer will trigger a segmentation fault:
///
/// ```
/// # use segv_test::assert_segv;
/// #
/// const INVALID_PTR: *mut i32 = 0x08 as *mut i32;
///
/// assert_segv!(
///     unsafe { INVALID_PTR.write_volatile(1); },
///     "write should trigger a segv"
/// );
///
/// assert_segv!(
///     unsafe { INVALID_PTR.read_volatile(); },
///     "read should trigger a segv"
/// );
/// ```
///
/// Reading or writing to an **valid** pointer will trigger a segmentation fault:
///
/// ```should_panic
/// # use segv_test::assert_segv;
/// #
/// let mut val = 0;
/// let valid_ptr = &raw mut val;
///
/// assert_segv!(
///     unsafe { valid_ptr.write_volatile(1); },
/// );
/// ```
///
/// # Notes
/// After the segmentation fault happens, the stack is not unwound.
/// The control flow will return directly to the point where the macro
/// was called.
///
/// In particular, this means that drop will not be called after the
/// segmentation fault occurs, and no cleanup will be performed.
///
/// It is recommended to keep the code inside the macro as simple as
/// possible to avoid leaking resources.
///
#[macro_export]
macro_rules! assert_segv {
    ($body:expr $(,)?) => {
        assert_segv!($body, "Expected a segmentation fault.");
    };
    ($body:expr, $($rest:tt)+) => {{
        if !$crate::check_segv(|| { { $body }; }) {
            ::std::panic!($($rest)+);
        }
    }};
}
