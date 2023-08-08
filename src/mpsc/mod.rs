use crate::io_source::IoSource;
use crate::{event, sys, Interest, Registry, Token};
use std::{
    io,
    sync::{mpsc, Arc, Mutex},
};

/// Create a pair of the [`Sender`] and the [`Receiver`].
///
/// The [`Receiver`] implements the [`event::Source`] so that it can be registered
/// with the [`mio::poll::Poll`], while the [`Sender`] doesn't.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel();

    (Sender { inner: tx }, Receiver { inner: rx })
}

/// Create a pair of the [`SyncSender`] and the [`Receiver`].
///
/// The [`Receiver`] implements the [`event::Source`] so that it can be registered
/// with the [`mio::poll::Poll`], while the [`Sender`] doesn't.
pub fn sync_channel<T>(bound: usize) -> (SyncSender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::sync_channel(bound);

    (SyncSender { inner: tx }, Receiver { inner: rx })
}

pub struct Receiver<T> {
    inner: IoSource<mpsc::Receiver<T>>,
}

impl<T> Receiver<T> {
    /// Try to receive a value. It works just like [`mpsc::Receiver::try_recv`].
    pub fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        self.inner.do_io(|inner| inner.try_recv())
    }
}

impl event::Source for Receiver<T> {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        self.inner.register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        self.inner.reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        self.inner.deregister(registry)
    }
}

pub struct Sender<T> {
    inner: IoSource<mpsc::Sender<T>>,
}

impl<T> SyncSender<T> {
    /// Try to send a value. It works just like [`mpsc::SyncSender::send`].
    /// After sending it, it's waking upthe [`mio::poll::Poll`].
    ///
    /// Note that it does not return any I/O error even if it occurs
    /// when waking up the [`mio::poll::Poll`].
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        self.inner.do_io(|inner| inner.send())
    }
}

pub struct SyncSender<T> {
    inner: IoSource<mpsc::SyncSender<T>>,
}

impl<T> SyncSender<T> {
    /// Try to send a value. It works just like [`mpsc::SyncSender::send`].
    /// After sending it, it's waking upthe [`mio::poll::Poll`].
    ///
    /// Note that it does not return any I/O error even if it occurs
    /// when waking up the [`mio::poll::Poll`].
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        self.inner.do_io(|inner| inner.send())
    }

    /// Try to send a value. It works just like [`mpsc::SyncSender::send`].
    /// After sending it, it's waking upthe [`mio::poll::Poll`].
    ///
    /// Note that it does not return any I/O error even if it occurs
    /// when waking up the [`mio::poll::Poll`].
    pub fn try_send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        self.inner.do_io(|inner| inner.try_send())
    }
}
