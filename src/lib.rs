pub fn run() {
    println!("hello, world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity() {
        assert_eq!(1+1, 2);
    }
}