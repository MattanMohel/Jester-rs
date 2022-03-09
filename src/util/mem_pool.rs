use std::ptr;
use std::mem;

type Init<T> = fn() -> T;

enum Elem<T> {
    Used(T),
    Free(usize),
}

impl<T> Elem<T> {
    pub fn unwrap_used(&mut self) -> &mut T {
        if let Elem::Used(value) = self {
            value
        } else {
            panic!("unwrapperd 'Elem' was not 'Free'")
        }
    }

    pub fn unwrap_free(&self) -> usize {
        if let Elem::Free(next) = *self {
            next
        } else {
            panic!("unwrapperd 'Elem' was not 'Free'")
        }
    }
}

// a generic static memory pool implementation

pub struct MemPool<T: Default, const SZ: usize> {  
    // buffer of all alocated elements
    buffer: [Elem<T>; SZ],

    // current index to the buffer
    head: usize,

    // clears element values on acquisition
    clear: Init<T>,
}

impl<T: Default, const SZ: usize> MemPool<T, SZ> {
    pub fn new(clear: Init<T>) -> MemPool<T, SZ> {

        // assert that size of 'T' is greater than or equal to size
        // of 'usize' to ensure correctness of later pointer arithmetic
        assert!(mem::size_of::<T>() >= mem::size_of::<usize>());
        
        MemPool { 
            buffer: array_init::array_init(|i| {
                Elem::Free(i + 1)
            }),

            head: 0,
                     
            clear: clear,
        }
    }

    pub fn acquire(&mut self) -> &mut T {
        assert_ne!(self.head, SZ);

        self.head = self.buffer[self.head].unwrap_free();
        self.buffer[self.head] = Elem::Used((self.clear)());
        self.buffer[self.head].unwrap_used()
    }

    pub fn release(&mut self, elem: &mut T) {
        let index = self.index_of(elem);

        self.buffer[index] = Elem::Free(self.head);
        self.head = index;
    }

    fn index_of(&self, elem: &T) -> usize {
        let index = unsafe {
            (elem as *const T).offset_from(ptr::addr_of!(self.buffer[0]) as *const T)
        };

        assert!(index > 0 && index < SZ as isize);

        index as usize
    }
}