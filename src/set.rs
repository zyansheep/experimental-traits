/// A Set is a collection that can be queried for a contained value and is iterable.
pub trait Set: Collection + Contains + Iterable {
    /// Returns true if `self` has no elements in common with `other`
    fn is_disjoint(&self, other: &Self) -> bool;

    /// Returns true if `self` is a subset of another.
    fn is_subset(&self, other: &Self) -> bool;

    /// Returns true if `self` is a superset of another.
    fn is_superset(&self, other: &Self) -> bool;

    /// Removes and returns the value in `self`, if any, that is equal to the given one.
    fn take(&mut self, value: &Self::ElemType) -> Option<Self::ElemType>
    where
        Self: Owned;

    /// Adds a value to the set.
    fn insert(&mut self, value: Self::ElemType) -> bool
    where
        Self: Owned;

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    fn get(&self, value: &Self::ElemType) -> Option<&Self::ElemType>;

    /// Removes a value from `self`. Returns whether the value was present in `self`.
    fn remove(&mut self, value: &Self::ElemType) -> bool
    where
        Self: Owned;

    /// Adds a value to `self`, replacing the existing value, if any, that is equal to the given one. Returns the replaced value.
    fn replace(&mut self, value: Self::ElemType) -> Option<Self::ElemType>
    where
        Self: Owned;
}