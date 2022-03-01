
type ClearFn<T> = fn(*mut T) -> *mut T;
type Constructor<T> = fn() -> *mut T;

pub struct MemPool<T: Default, const Sz: usize> {
    buffer: [T; Sz],
    elements: [*mut T; Sz],
    index: usize,

    clear_fn: ClearFn<T>,
    constructor: Constructor<T>,

    tag: String,
}

impl<T: Default, const Sz: usize> MemPool<T, Sz> {
    pub fn new(constructor: Constructor<T>, clear_fn: ClearFn<T>, tag: &String) -> MemPool<T, Sz> {

        let mut buffer = array_init::array_init(|_| {
            T::default()
        });

        let mut elements = [std::ptr::null_mut(); Sz];

        for (i, elem) in buffer.iter_mut().enumerate() {
            elements[i] = elem as *mut T;
        }

        MemPool {
            buffer: buffer,
            elements: elements,
            index: 0,
            
            clear_fn: clear_fn,
            constructor: constructor,
            tag: tag.clone(),
        }
    }

    pub fn acquire(&mut self) -> *mut T {
        assert_ne!(self.index, 0);

        let ptr = self.elements[self.index];
        self.elements[self.index] = std::ptr::null_mut();

        self.index += 1;

        (self.clear_fn)(ptr)
    }

    pub fn release(&mut self, elem: *mut T) {
        assert_ne!(self.index, 0);

        self.index -= 1;

        self.elements[self.index] = elem;
    }
}