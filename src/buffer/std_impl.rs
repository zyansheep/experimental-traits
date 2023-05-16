use super::*;

impl Buffer for Vec<u8> {
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn len(&self) {
        self.len()
    }

    fn capacity(&self) {
        self.capacity()
    }
}

