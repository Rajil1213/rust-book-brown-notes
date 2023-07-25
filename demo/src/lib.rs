pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn another() {
        panic!("this test fails!");
    }
}
