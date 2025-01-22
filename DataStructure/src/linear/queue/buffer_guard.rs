use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct BufferGuard<T> {
    buffer: NonNull<T>,
    layout: Layout,
}

impl<T> BufferGuard<T> {
    pub fn new(size: usize) -> BufferGuard<T> {
        let layout = Layout::array::<T>(size).unwrap();
        let buffer = NonNull::new(unsafe { alloc(layout) } as _).expect("Failed to allocate buffer");
        BufferGuard { buffer, layout }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.buffer.as_ptr()
    }

    pub fn into_inner(mut self) -> (NonNull<T>, Layout) {
        let buffer = self.buffer;
        let layout = self.layout;
        mem::forget(self);
        (buffer, layout)
    }
}

impl<T> Drop for BufferGuard<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.buffer.as_ptr() as _, self.layout);
        }
    }
}
