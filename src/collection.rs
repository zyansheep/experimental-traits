

mod countable;
mod aligned;
mod iterable;
pub use countable::*;
pub use aligned::*;
pub use iterable::*;

/// Representing collection of items all of the same type.
pub trait TypedCollection {
	type Item;
}