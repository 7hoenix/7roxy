pub fn hello_world() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert!(hello_world());
    }
}
