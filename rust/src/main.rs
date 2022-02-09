use std::any::{Any, TypeId};
use std::panic;
// unstable
// use std::backtrace::Backtrace;

fn unwind() {
    let r0 = panic::catch_unwind(|| {
        return "nothing new";
    });
    println!("ok {:?}", r0);
    let r1 = panic::catch_unwind(|| {
        // panic!("not good")
        panic!("not good {}", 2)
    });
    let r2 = match r1 {
        Ok(r) => Ok(r),
        Err(e) => Err(
            match e.downcast::<String>() {
                Ok(s) => *s,
                Err(_) => match e.downcast::<&str>() {
                    Ok(m1) => m1.to_string(),
                    Err(_) => "unknown cause".to_string(),
                }
            }
        ),
    };
    println!("err {:?}", r2);
}

fn main() {
    unwind()
}
