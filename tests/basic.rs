use segv_test::assert_segv;

// do not use null_ptr or 0x00 as write_volatile
// checks that in debug mode.
const INVALID_PTR: *mut i32 = 0x08 as *mut i32;

#[test]
fn test_basic_success_write() {
    assert_segv!(unsafe {
        INVALID_PTR.write_volatile(1);
    });
}

#[test]
fn test_basic_success_read() {
    assert_segv!(unsafe {
        INVALID_PTR.read_volatile();
    });
}

#[test]
fn test_basic_success_twice() {
    // on unix, this test that we are unblocking the SIGSEGV
    // signal before longjmp-ing out of the signal handler

    assert_segv!(unsafe {
        INVALID_PTR.write_volatile(1);
    });

    assert_segv!(unsafe {
        INVALID_PTR.write_volatile(1);
    });
}

#[test]
#[should_panic(expected = "Expected a segmentation fault.")]
fn test_basic_failure() {
    let mut val = 0;
    let valid_ptr = &raw mut val;
    assert_segv!(unsafe {
        valid_ptr.write_volatile(1);
    });
}

#[test]
#[should_panic(expected = "Nested segmentation fault assertion is not supported.")]
fn test_nested_failure() {
    assert_segv!({
        assert_segv!(unsafe {
            INVALID_PTR.write_volatile(1);
        })
    });
}

#[test]
#[should_panic(expected = "Hello world, 1234!")]
fn test_nested_failure_custom_message() {
    assert_segv!({}, "Hello world, {}!", 1234);
}
