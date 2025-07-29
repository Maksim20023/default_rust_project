pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test{
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}