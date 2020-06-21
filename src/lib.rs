fn hello_world() -> bool {
    1 == 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert!(hello_world());
    }
}
