use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn unsafe_mem() -> (i32, i32) {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("const unsafe ref = {}", *r1);
        println!("mut ref = {}", *r2);
        (*r1, *r2)
    }
}

/// Does something unsafe.
///
/// # Safety
///
/// Use at your own risk. 😈
pub unsafe fn dangerous() -> Result<(), String> {
    Ok(())
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr: *mut i32 = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_mem() {
        assert_eq!(5, unsafe_mem().0);
        assert_eq!(5, unsafe_mem().1);
    }

    #[test]
    fn call_dangerous() {
        unsafe {
            assert!(dangerous().is_ok());
        }
    }

    #[test]
    fn safe_over_unsafe() {
        let mut a = [1, 2, 3, 4, 5];
        let parts = split_at_mut(&mut a[..], 2);

        assert_eq!(parts.0, &[1, 2]);
        assert_eq!(parts.1, &[3, 4, 5]);
    }

    #[test]
    fn invoke_ffi() {
        unsafe {
            assert_eq!(3, abs(-3));
        }
    }
}
