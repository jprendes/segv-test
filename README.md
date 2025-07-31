# segv-test

A small library providing an `assert_segv!` macro to test that some code triggers a segmentation fault.

## Example

```rust
use segv_test::assert_segv;

const INVALID_PTR: *mut i32 = 0x08 as *mut i32;

#[test]
fn test_a_segv() {
    assert_segv!(unsafe {
        INVALID_PTR.write_volatile(1);
    });
}
```
