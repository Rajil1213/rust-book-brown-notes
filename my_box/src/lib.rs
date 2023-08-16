#[cfg(test)]
mod test {
    #[test]
    fn test_ref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(x, *y);
    }
}
