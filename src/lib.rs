#![no_std]

use core::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{Sink, Stream};

pub struct SinkStream<I, T: Sink<I>, U: Stream> {
    sink: T,
    stream: U,
    item: PhantomData<I>,
}

pub trait ItemSplit<I, O>: Sink<I> + Stream<Item = O> {
    type Sink: Sink<I>;
    type Stream: Stream<Item = O>;

    fn split(self) -> (Self::Sink, Self::Stream);
}

impl<I: Unpin, T: Sink<I> + Unpin, U: Stream + Unpin> Stream for SinkStream<I, T, U> {
    type Item = U::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }
}

impl<I: Unpin, T: Sink<I> + Unpin, U: Stream + Unpin> Sink<I> for SinkStream<I, T, U> {
    type Error = T::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sink).poll_ready(cx)
    }
    fn start_send(mut self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        Pin::new(&mut self.sink).start_send(item)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sink).poll_flush(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sink).poll_close(cx)
    }
}

impl<I: Unpin, T: Sink<I> + Unpin, U: Stream + Unpin> ItemSplit<I, U::Item>
    for SinkStream<I, T, U>
{
    type Sink = T;
    type Stream = U;

    fn split(self) -> (Self::Sink, Self::Stream) {
        (self.sink, self.stream)
    }
}

impl<I: Unpin, T: Sink<I> + Unpin, U: Stream + Unpin> SinkStream<I, T, U> {
    pub fn new(sink: T, stream: U) -> Self {
        SinkStream {
            sink,
            stream,
            item: PhantomData,
        }
    }
}
