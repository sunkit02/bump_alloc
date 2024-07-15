use std::{
    alloc::{alloc, Layout},
    cell::Cell,
    mem, ptr,
};

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

pub struct Arena {
    start: *mut u8,
    size: usize,
    top: Cell<usize>,
}

impl Arena {
    pub fn new() -> Self {
        let layout = Layout::array::<u8>(DEFAULT_SIZE).unwrap();
        let start = unsafe { alloc(layout) };

        Self {
            start,
            size: DEFAULT_SIZE,
            top: Cell::new(0),
        }
    }

    pub fn alloc<T: IntoBytes>(&self, value: T) -> &mut T {
        if self.top.get() >= self.size {
            panic!("Nope");
        }

        let alignment = mem::align_of::<T>();

        // Align the memory layout for type T
        if self.top.get() % alignment != 0 {
            self.top
                .replace(self.top.get() + self.top.get() % alignment);
        }

        unsafe {
            let position = self.start.add(self.top.get());
            self.top.replace(self.top.get() + mem::size_of::<T>());

            let bytes = value.into_bytes();
            ptr::copy_nonoverlapping(bytes.as_ptr(), position, bytes.len());

            (position as *mut T).as_mut().unwrap()
        }
    }

    pub fn reset(&mut self) {
        self.top.replace(0);
    }
}

const DEFAULT_SIZE: usize = 4096;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: [u8; 12],
        age: u32,
    }

    impl IntoBytes for Person {
        fn into_bytes(self) -> Vec<u8> {
            self.name
                .into_iter()
                .chain(self.age.to_le_bytes())
                .collect()
        }
    }

    impl IntoBytes for f64 {
        fn into_bytes(self) -> Vec<u8> {
            self.to_le_bytes().to_vec()
        }
    }

    #[test]
    fn can_allocate() {
        let arena = Arena::new();
        let person1 = arena.alloc(Person {
            name: *b"Sun Kit Tsui",
            age: 22,
        });

        let pi = arena.alloc(3.14159f64);

        let person2 = arena.alloc(Person {
            name: *b"Sun Kit Tsui",
            age: 24,
        });

        assert_eq!(
            *person1,
            Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            }
        );

        assert_eq!(*pi, 3.14159);

        assert_eq!(
            *person2,
            Person {
                name: *b"Sun Kit Tsui",
                age: 24,
            }
        );
    }
}
