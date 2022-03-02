
type ClearFn<T> = fn(*mut T) -> *mut T;

/**
 * a generic static memory pool implementation
 * 
 * generic type must implement Default for buffer
 * initialization
 * 
 */

pub struct MemPool<T: Default, const SZ: usize> {
    // buffer of all alocated elements
    buffer: [T; SZ],

    // pointers to all free elements
    free: [*mut T; SZ],

    // an index to the next free element
    index: usize,

    // clears element values on acquisition
    clear: ClearFn<T>,
}

impl<T: Default, const SZ: usize> MemPool<T, SZ> {
    pub fn new(clear: ClearFn<T>) -> MemPool<T, SZ> {

        let mut buffer = array_init::array_init(|_| {
            T::default()
        });

        MemPool {
            free: array_init::array_init(|i| {
                &mut buffer[i] as *mut T
            }),

            index: 0,

            buffer: buffer,
               
            clear,
        }
    }

    pub fn acquire(&mut self) -> *mut T {
        assert_ne!(self.index + 1, SZ);

        self.index += 1;
        (self.clear)(self.free[self.index - 1])
    }

    pub fn release(&mut self, elem: *mut T) {
        assert_ne!(self.index, 0);

        self.index -= 1;
        self.free[self.index] = elem;
    }
}