fn unsafe_mem() -> (i32, i32) {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("const unsafe ref = {}", *r1);
        println!("mut ref = {}", *r2);
        return (*r1, *r2);
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
}
