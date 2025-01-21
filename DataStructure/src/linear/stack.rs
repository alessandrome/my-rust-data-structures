use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ptr::NonNull;

#[cfg(test)]
mod tests;

pub struct Stack<T> {
    size: usize,
    length: usize,
    buffer: NonNull<T>,
    layout: Layout,
}

pub const STACK_START_SIZE: usize = 10;
pub const STACK_SIZE_INCREMENT: usize = 10;

impl<T: Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        let layout = Layout::array::<T>(STACK_START_SIZE).unwrap();
        let ptr = NonNull::new(unsafe { alloc(layout) } as *mut T).expect("Allocation failed");
        Stack {
            size: STACK_START_SIZE,
            length: 0,
            buffer: ptr,
            layout,
        }
    }

    fn _recreate_buffer(&mut self) {
        let layout = Layout::array::<T>(self.size).unwrap();
        let ptr = NonNull::new(unsafe { alloc(layout) } as *mut T).expect("Allocation failed");
        self.buffer = ptr;
        self.layout = layout;
    }

    fn _increment_size(&mut self, increment: usize) {
        self.size += increment;
        let old_array = self.buffer;
        let old_layout = self.layout;
        self._recreate_buffer();
        for i in 0..self.length {
            unsafe { *self.buffer.as_ptr().add(i) = ptr::read(old_array.as_ptr().add(i)) };
        }
        unsafe { dealloc(old_array.as_ptr() as *mut u8, old_layout) };
    }

    pub fn length(&self) -> usize { self.length }
    pub fn is_empty(&self) -> bool { self.length == 0 }
    pub fn buffer_size(&self) -> usize { self.size }
    pub fn buffer_is_full(&self) -> bool { self.size == self.length }

    pub fn push(&mut self, item: T) {
        if self.buffer_is_full() {
            self._increment_size(STACK_SIZE_INCREMENT);
        }
        // TODO
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;
            return Some(unsafe { ptr::read(self.buffer.as_ptr().add(self.length)) });
        }
        None
    }
    pub fn top(&mut self) -> Option<&T> {
        if self.length > 0 {
            return Some(unsafe { &ptr::read(self.buffer.as_ptr().add(self.length)) });
        }
        None
    }
    pub fn top_mut(&mut self) -> Option<&mut T> {
        if self.length > 0 {
            return Some(unsafe { &mut ptr::read(self.buffer.as_ptr().add(self.length)) });
        }
        None
    }
}

impl<T> Display for Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Top -> [")?;
        for i in 0..self.length {
            write!(f, "{}", i)?;
            if i != self.length - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe { dealloc(self.buffer.as_ptr() as _, self.layout); }
    }
}