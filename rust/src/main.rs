use std::any::{Any};
use std::panic;
// unstable
// use std::backtrace::Backtrace;

// https://stackoverflow.com/a/68558313

fn convert_unwind_error<R>(r: Result<R, Box<dyn Any + Send>>) -> Result<R, String> {
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
    println!("ok {:?}", convert_unwind_error(r0));
    let r1 = panic::catch_unwind(|| {
        panic!("not good");
    });
    println!("err {:?}", convert_unwind_error(r1));
    let r2 = panic::catch_unwind(|| {
        panic!("not good {}", "apple");
    });
    println!("err {:?}", convert_unwind_error(r2));
}

fn main() {
    unwind()
}
