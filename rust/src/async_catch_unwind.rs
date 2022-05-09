use futures::future::FutureExt;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use crate::error::Error;

pub async fn async_try_with_catch_unwind<F, R>(future: F) -> Result<R, Error>
    where
        F: Future<Output = Result<R, Error>>,
{
    let result = AssertUnwindSafe(future).catch_unwind().await;
    match result {
        Ok(x) => x,
        Err(e) => Err(match e.downcast::<String>() {
            Ok(s) => Error::Panic(*s),
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

    #[tokio::test]
    async fn test_async_try_with_catch_unwind() {
        let x0: Result<String, Error> = async_try_with_catch_unwind(async { Ok("ok".to_string()) }).await;
        assert!(matches!(x0, Ok(msg) if msg == "ok"));
        let x1: Result<String, Error> = async_try_with_catch_unwind(async { Err(Error::Error("err".to_string())) }).await;
        assert!(matches!(x1, Err(Error::Error(msg)) if msg == "err"));
        let x2: Result<String, Error> = async_try_with_catch_unwind(async { panic!("oops") }).await;
        assert!(matches!(x2, Err(Error::Panic(msg)) if msg == "oops"));
        let x3: Result<String, Error> = async_try_with_catch_unwind(async { panic!("oops{}", "ie") }).await;
        assert!(matches!(x3, Err(Error::Panic(msg)) if msg == "oopsie"));
    }
}
