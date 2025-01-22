use crate::linear::stack::buffer_guard::BufferGuard;
use std::alloc::{alloc, dealloc, Layout};
use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::NonNull;

mod buffer_guard;
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

    fn _increment_size(&mut self, increment: usize) {
        let new_size = self.size + increment;
        // Create a temporary new buffer that will be correctly dropped if something goes wrong
        let buffer_guard = BufferGuard::<T>::new(new_size);
        for i in 0..self.length {
            unsafe {
                ptr::write(
                    buffer_guard.as_ptr().add(i),
                    ptr::read(self.buffer.as_ptr().add(i)),
                )
            };
        }
        // Deallocate the old buffer and replace it with the new in the buffer guard
        unsafe { dealloc(self.buffer.as_ptr() as *mut u8, self.layout) };
        (self.buffer, self.layout) = buffer_guard.into_inner();
        self.size = new_size;
    }

    pub fn length(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn buffer_size(&self) -> usize {
        self.size
    }
    pub fn buffer_is_full(&self) -> bool {
        self.size == self.length
    }

    pub fn push(&mut self, item: T) {
        if self.buffer_is_full() {
            self._increment_size(STACK_SIZE_INCREMENT);
        }
        unsafe {
            *self.buffer.as_ptr().add(self.length) = item;
        }
        self.length += 1;
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
            return Some(unsafe { &*self.buffer.as_ptr().add(self.length - 1) });
        }
        None
    }
    pub fn top_mut(&mut self) -> Option<&mut T> {
        if self.length > 0 {
            return Some(unsafe { &mut *self.buffer.as_ptr().add(self.length - 1) });
        }
        None
    }
}

impl<T: std::fmt::Display> Display for Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Top -> [")?;
        for i in (0..self.length).rev() {
            write!(f, "{}", unsafe { self.buffer.as_ptr().add(i).read() })?;
            if i != 0 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            // Drop every element to ensure custom and deep drops are executed
            for i in 0..self.size {
                ptr::drop_in_place(self.buffer.as_ptr().add(i));
            }
            // Drop the buffer array
            dealloc(self.buffer.as_ptr() as _, self.layout);
        }
    }
}
