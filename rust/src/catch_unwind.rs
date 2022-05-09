use std::panic::{catch_unwind, UnwindSafe};
use crate::error::Error;

pub fn try_with_catch_unwind<F, R>(f: F) -> Result<R, Error>
    where
        F: FnOnce() -> Result<R, Error> + UnwindSafe
{
    let result = catch_unwind(f);
    match result {
        Ok(x) => x,
        Err(e) => Err(match e.downcast::<String>() {
            Ok(s) => Error::Panic(*s) ,
            Err(e) => match e.downcast::<&str>() {
                Ok(m1) => Error::Panic(m1.to_string()),
                Err(_) => Error::Panic("unknown cause".to_string()),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_try_with_catch_unwind() {
        let x0: Result<String, Error> = try_with_catch_unwind(|| Ok("ok".to_string()));
        assert!(matches!(x0, Ok(msg) if msg == "ok"));
        let x1: Result<String, Error> = try_with_catch_unwind(|| Err(Error::Error("err".to_string())));
        assert!(matches!(x1, Err(Error::Error(msg)) if msg == "err"));
        let x2: Result<String, Error> = try_with_catch_unwind(|| panic!("oops"));
        assert!(matches!(x2, Err(Error::Panic(msg)) if msg == "oops"));
        let x3: Result<String, Error> = try_with_catch_unwind(|| panic!("oops{}", "ie"));
        assert!(matches!(x3, Err(Error::Panic(msg)) if msg == "oopsie"));
    }

    // #[tokio::test]
    // async fn test_try_with_catch_unwind_async() {
    //     let x0 = try_with_catch_unwind(|| async { panic!("oops") }.await);
    //     assert!(matches!(x0, Err(Error::Panic(msg)) if msg == "oops"));
    // }
}
