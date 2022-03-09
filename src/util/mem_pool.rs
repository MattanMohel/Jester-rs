
type ClearFn<T> = fn(&mut T) -> &mut T;

enum Elem<T> {
    Used(T),
    Free(*mut T),
}

impl<T> Elem<T> {
    fn unwrap(&mut self) -> &mut T {
        if let Elem::Used(elem) = self {
            return elem
        }

        panic!("unwrapped 'Elem' value wasn't 'Used'");
    }
}

/**
 * a generic static memory pool implementation
 * 
 * generic type must implement Default for buffer
 * initialization
 * 
 */

pub struct MemPool<T: Default, const SZ: usize> {  
    // buffer of all alocated elements
    buffer: [Elem<T>; SZ],

    index: *mut T,

    // clears element values on acquisition
    clear: ClearFn<T>,
}

impl<T: Default, const SZ: usize> MemPool<T, SZ> {
    pub fn new(clear: ClearFn<T>) -> MemPool<T, SZ> {

        assert!(std::mem::size_of::<T>() >= std::mem::size_of::<*mut T>());
        
        let buffer: array_init::array_init(|_| {
            Elem::Used(T::default())
        });

        MemPool { 
            index: buffer[0].unwrap() as *mut T,
            
            buffer: buffer,
               
            clear,
        }
    }

    pub fn acquire(&mut self) -> *mut T {
        assert_ne!(self.index + 1, SZ);

        self.index += 1;

        unsafe {
            (self.clear)(&mut (*self.free[self.index - 1])) as *mut T
        }
    }

    pub fn release(&mut self, elem: *mut T) {
        let index = self.index_of(elem);
        assert!(index > 0 && index < SZ as isize);

        self.index -= 1;
        self.free[self.index] = elem;
    }

    fn index_of(&self, elem: *mut T) -> isize {
        unsafe {
            elem.offset_from(self.buffer[0].unwrap())
        }
    }
}