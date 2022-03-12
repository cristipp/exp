use std::pin::Pin;
use std::task::{Context, Poll};
use async_stream::stream;
use futures::stream::{ Stream, StreamExt };
use pin_project_lite::pin_project;

pin_project! {
    struct StreamWrapper {
        #[pin]
        stream: Pin<Box<dyn Stream<Item = Result<i32, String>>>>,
        finished: bool,
    }
}

impl StreamWrapper {
    pub fn new(count: i32) -> Self {
        let stream = stream! {
            for i in 0..count {
                yield Ok(i);
            }
        };

        StreamWrapper {
            stream: stream.boxed(),
            finished: false,
        }
    }
}

impl Stream for StreamWrapper {
    type Item = Result<i32, String>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        return Pin::new(&mut this.stream).poll_next(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stream() {
        let w0 = StreamWrapper::new(3);
        let xs = w0.stream.collect::<Vec<_>>().await;
        assert_eq!(xs, vec![Ok(0), Ok(1), Ok(2)]);
        let w1 = StreamWrapper::new(3);
        let ws = w1.collect::<Vec<_>>().await;
        assert_eq!(ws, vec![Ok(0), Ok(1), Ok(2)]);
    }
}