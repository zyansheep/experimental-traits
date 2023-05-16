mod std_impl;

use std::{mem::MaybeUninit, io::{self, Read, Write}, task::{Context, Poll}, pin::Pin};

use futures::{AsyncRead, AsyncWrite};

use crate::TypedCollection;

// A buffer is any object that can be coerced into a contiguous slice of bytes
pub trait Buffer: TypedCollection<Item = u8> {
	fn with_capacity(capacity: usize) -> Self;
	fn len(&self) -> usize;
	fn capacity(&self) -> usize;
}

pub trait ContiguousBuffer: AsRef<[u8]> + AsMut<[u8]> {

}
trait WritableBuffer: Buffer {
	/// Fill buffer from reader
	fn fill(&mut self, reader: &mut impl Read) -> io::Result<usize>;

	/// Fill buffer from async reader
	fn poll_fill<R: AsyncRead + Unpin>(&mut self, reader: Pin<&mut R>, cx: &mut Context<'_>) -> Poll<io::Result<usize>>;

	/// Fill buffer with function if buffer has at least `min_size` bytes left to fill.
	fn fill_with<R>(&mut self, min_size: usize, f: impl FnOnce(&mut [u8]) -> R) -> Option<R>;
}
trait ReadableBuffer: Buffer {
	/// Empty buffer to writer
	fn empty(&mut self, writer: &mut impl Write) -> io::Result<usize>;
	/// Consume amount from start of buffer
	fn consume(&mut self, amount: usize);

	/// Empty to async writer
	fn poll_empty<W: AsyncWrite + Unpin>(&mut self, writer: Pin<&mut W>, cx: &mut Context<'_>) -> Poll<io::Result<usize>>;
}

// Buffer that allows for writing into uninitialized part of its memory
pub trait UninitBuffer: Buffer {
	/// Get number of bytes initialized in buffer
	fn init(&self) -> usize;
	/// Get initialized part of buffer as u8 slice
	fn get_init(&mut self) -> &mut [u8];

	/// Get number of bytes uninitialized in buffer
	fn uninit(&self) -> usize;
	/// Get uninitialized part of buffer as MaybeUninit<u8> slice.
	fn get_uninit(&mut self) -> &mut [MaybeUninit<u8>];
}

/// Abstraction over Buffer that can work as a ring
pub trait RingBuffer: Buffer {
	/// Get number of bytes in buffer
	fn len(&self) -> usize;
	

}