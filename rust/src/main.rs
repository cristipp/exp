use std::any::{Any};
use std::future::Future;
use futures::future::FutureExt;
use std::panic;
use std::pin::Pin;
// unstable
// use std::backtrace::Backtrace;

mod async_catch_unwind;
mod catch_unwind;
mod error;
mod stream;

// https://stackoverflow.com/a/68558313

fn unwind_error_to_string<R>(r: Result<R, Box<dyn Any + Send>>) -> Result<R, String> {
    return match r {
        Ok(x) => Ok(x),
        Err(e) => Err(
            match e.downcast::<String>() {
                Ok(s) => *s,
                Err(e) => match e.downcast::<&str>() {
                    Ok(m1) => m1.to_string(),
                    Err(_) => "unknown panic cause".to_string(),
                }
            }
        )
    }
}

fn unwind() {
    let r0 = panic::catch_unwind(|| {
        return "nothing new";
    });
    println!("ok {:?}", unwind_error_to_string(r0));
    let r1 = panic::catch_unwind(|| {
        panic!("not good");
    });
    println!("err {:?}", unwind_error_to_string(r1));
    let r2 = panic::catch_unwind(|| {
        panic!("not good {}", "apple");
    });
    println!("err {:?}", unwind_error_to_string(r2));
}

async fn async_try_with_catch_unwind<F, R>(future: F) -> Result<R, String>
    where F: Future<Output = Result<R, String>>
{
    let result = panic::AssertUnwindSafe(future).catch_unwind().await;
    return match result {
        Ok(x) => x,
        Err(e) => Err(
            match e.downcast::<String>() {
                Ok(s) => *s,
                Err(e) => match e.downcast::<&str>() {
                    Ok(m1) => m1.to_string(),
                    Err(_) => "unknown panic cause".to_string(),
                }
            }
        )
    }
}

async fn async_unwind(x: i32) {
    let f0: Pin<Box<dyn Future<Output = Result<i32, String>>>> = Box::pin(async { Ok(x) });
    let x0 = async_try_with_catch_unwind(f0).await;
    println!("x0 {:?}", x0);
    let f1: Pin<Box<dyn Future<Output = Result<i32, String>>>> = Box::pin(async { Err("err".to_string()) });
    let x1 = async_try_with_catch_unwind(f1).await;
    println!("x1 {:?}", x1);
    let f2: Pin<Box<dyn Future<Output = Result<i32, String>>>> = Box::pin(async { panic!("oops") });
    let x2 = async_try_with_catch_unwind(f2).await;
    println!("x2 {:?}", x2);
    let f3: Pin<Box<dyn Future<Output = Result<i32, String>>>> = Box::pin(async { panic!("oops{}", "ie") });
    let x3 = async_try_with_catch_unwind(f3).await;
    println!("x3 {:?}", x3);
}

#[tokio::main]
async fn main() {
    unwind();
    async_unwind(42).await
}
