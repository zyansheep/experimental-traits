

/// Re
pub trait CountableCollection {
    /// Size unit for indexing & length
    type SizeType: std::cmp::PartialEq = usize;

    /// Checks if `Self` is  empty
    fn is_empty(&self) -> bool;

    /// Returns the length of `Self`
    fn len(&self) -> Self::SizeType;
}

pub use impls::*;
mod impls {
    use super::CountableCollection;

    macro_rules! countable_collection_impl {
        () => {};
        ([$($args: ident $(: $bound: path $(| $others:path )*)?),* => $t: ty]; $($tail:tt)*) => {
            impl<$($args $(: $bound $(+ $others)*)?),*> CountableCollection for $t {
                type SizeType = usize;

                fn is_empty(&self) -> bool {
                    <$t>::is_empty(self)
                }

                fn len(&self) -> usize {
                    <$t>::len(self)
                }
            }
            countable_collection_impl!($($tail)*);
        }
    }

    impl<T> CountableCollection for &[T] {
        fn is_empty(&self) -> bool {
            <[T]>::is_empty(self)
        }

        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T> CountableCollection for &mut [T] {
        fn is_empty(&self) -> bool {
            <[T]>::is_empty(self)
        }

        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T, const N: usize> CountableCollection for [T; N] {
        fn is_empty(&self) -> bool {
            N == 0
        }

        fn len(&self) -> usize {
            N
        }
    }

    countable_collection_impl!(
        [T, A: std::alloc::Allocator => Vec<T, A>];
        [T => std::collections::VecDeque<T>];
        [T => std::collections::LinkedList<T>];
        [T => std::collections::BTreeSet<T>];
        [T, S: std::hash::BuildHasher => std::collections::HashSet<T, S>];
        [T => std::collections::BinaryHeap<T>];
        [K, V => std::collections::BTreeMap<K, V>];
        [K, V, S: std::hash::BuildHasher => std::collections::HashMap<K, V, S>];
    );
}