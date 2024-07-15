use std::{
    alloc::{alloc, Layout},
    cell::Cell,
    mem,
    ptr::{self, NonNull},
};

pub struct Arena {
    start: NonNull<u8>,
    size: usize,
    top: Cell<usize>,
}

impl Arena {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_SIZE)
    }

    pub fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<u8>(cap).unwrap();
        let start = unsafe { NonNull::new_unchecked(alloc(layout)) };

        Self {
            start,
            size: cap,
            top: Cell::new(0),
        }
    }

    #[cfg(debug_assertions)]
    pub fn top(&self) -> usize {
        self.top.get()
    }

    pub fn alloc<T>(&self, value: T) -> &mut T {
        if self.top.get() >= self.size {
            panic!("Out of memory");
        }

        let alignment = mem::align_of::<T>();

        let heap_top = (self.start.as_ptr() as usize + self.top.get()) as *mut T;

        // Align the memory layout for type T
        let remainder = heap_top as usize % alignment;
        let aligned_ptr = if dbg!(remainder) == 0 {
            heap_top
        } else {
            (heap_top as usize - remainder + alignment) as *mut T
        };

        println!("{}", aligned_ptr as usize - self.start.as_ptr() as usize);

        unsafe {
            self.top
                .set(aligned_ptr.add(1) as usize - self.start.as_ptr() as usize);

            ptr::write(aligned_ptr, value);
            &mut *aligned_ptr
        }
    }

    /// NOTE: It is undefined behavior to access allocations acquired before calling reset.
    pub fn reset(&mut self) {
        self.top.set(0);
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.size).unwrap();
        unsafe { std::alloc::dealloc(self.start.as_ptr(), layout) }
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

    #[test]
    fn can_allocate() {
        let cap = mem::size_of::<Person>() * 2;
        let mut arena = Arena::with_capacity(cap * 1000);

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

        dbg!(&arena.top);
        arena.reset();
    }

    #[test]
    fn can_run_in_loop() {
        let mut arena = Arena::with_capacity(std::mem::size_of::<Person>() * 22);

        for _ in 0..1 {
            let _person1 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person2 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person3 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person4 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person5 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person6 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person7 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person8 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person9 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person10 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person11 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person12 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person13 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person14 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person15 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person16 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person17 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person18 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person19 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person20 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person21 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);
            let _person22 = arena.alloc(Person {
                name: *b"Sun Kit Tsui",
                age: 22,
            });
            dbg!(&arena.top);

            arena.reset();
        }
    }
}
