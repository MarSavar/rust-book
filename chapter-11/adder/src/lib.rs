pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg(test)]
mod tests {

    fn panic() {
        panic!("Panic message");

    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4, "{} should be different from 4 but it is not!", result);
    }

    #[test]
    #[should_panic]
    fn check_panic() {
        panic();
    }

}
