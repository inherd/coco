fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_failure() {
        assert_eq!(1, 1);
    }
}
