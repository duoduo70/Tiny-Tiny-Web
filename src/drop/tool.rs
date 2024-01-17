pub trait ShouldResult<T> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T;
}
impl<T> ShouldResult<T> for Option<T> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T{
        match self {
            Some(a) => a,
            _ => {func(); std::process::exit(ret_code);}
        }
    }
}
impl<T, E> ShouldResult<T> for Result<T, E> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T{
        match self {
            Ok(a) => a,
            _ => {func(); std::process::exit(ret_code);}
        }
    }
}