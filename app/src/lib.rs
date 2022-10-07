#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    fn exploration() {
        let result = 2 + 2;
        assert!(result == 5, "Was expecting 4, got {}", result);
    }

    #[test]
    fn advanced_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("not verified"))
        }
    }
}
