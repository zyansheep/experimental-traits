#![feature(array_methods)]
#![feature(allocator_api)]
#![feature(associated_type_defaults)]
#![feature(trusted_random_access)]
#![feature(type_alias_impl_trait)]
#![feature(slice_range)]
#![feature(btree_drain_filter)]
#![feature(drain_filter)]
#![feature(hash_drain_filter)]
#![feature(linked_list_remove)]
#![feature(map_try_insert)]
#![feature(extend_one)]
#![feature(return_position_impl_trait_in_trait)]

mod buffer;
mod collection;

pub use buffer::*;
pub use collection::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
