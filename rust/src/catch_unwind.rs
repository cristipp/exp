use std::panic::{catch_unwind, UnwindSafe};

pub enum SomeError {
    Error(String),
    Panic(String),
}

pub fn try_with_catch_unwind<F, R>(f: F) -> Result<R, SomeError>
    where
        F: FnOnce() -> Result<R, SomeError> + UnwindSafe
{
    let result = catch_unwind(f);
    match result {
        Ok(x) => x,
        Err(e) => Err(match e.downcast::<String>() {
            Ok(s) => SomeError::Panic(*s) ,
            Err(e) => match e.downcast::<&str>() {
                Ok(m1) => SomeError::Panic(m1.to_string()),
                Err(_) => SomeError::Panic("unknown cause".to_string()),
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[tokio::test]
    async fn test_try_with_catch_unwind() {
        let x0: Result<String, SomeError> = try_with_catch_unwind(|| Ok("ok".to_string()));
        assert!(matches!(x0, Ok(msg) if msg == "ok"));
        let x1: Result<String, SomeError> = try_with_catch_unwind(|| Err(SomeError::Error("err".to_string())));
        assert!(matches!(x1, Err(SomeError::Error(msg)) if msg == "err"));
        let x2: Result<String, SomeError> = try_with_catch_unwind(|| panic!("oops"));
        assert!(matches!(x2, Err(SomeError::Panic(msg)) if msg == "oops"));
        let x3: Result<String, SomeError> = try_with_catch_unwind(|| panic!("oops{}", "ie"));
        assert!(matches!(x3, Err(SomeError::Panic(msg)) if msg == "oopsie"));
    }
}
