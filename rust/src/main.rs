use std::any::{Any};
use std::future::Future;
use futures::future::FutureExt;
use std::panic;
// unstable
// use std::backtrace::Backtrace;

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

async fn try_with_catch_unwind<F, R>(future: F) -> Result<R, String>
    where F: Future<Output = Result<R, String>>
{
    let result = future.catch_unwind().await;
    result.unwrap()
}

async fn async_unwind(x: i32) {
    let x = try_with_catch_unwind(async { Ok(x) }).await;
    println!("x {:?}", x);
    // let x2 = try_with_catch_unwind(async { Err("err".to_string()) }).await;
    // println!("x2 {:?}", x2);
    // let x3 = try_with_catch_unwind(async { panic!("not good") }).await;
    // println!("x3 {:?}", x3);
}

#[tokio::main]
async fn main() {
    unwind();
    async_unwind(42).await
}
