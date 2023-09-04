#[cfg(test)]
mod test {
    #[test]
    fn match_arms() {
        let x: Option<i32> = Some(1);
        let matched = match x {
            None => 0,
            Some(i) => i + 1,
        };

        assert!(matched > 1);
    }
}
