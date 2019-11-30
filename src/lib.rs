pub type BoxError = Box<dyn std::error::Error>;
pub type StdResult<T> = Result<T, BoxError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
