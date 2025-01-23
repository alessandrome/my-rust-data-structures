use std::alloc::{alloc, dealloc, Layout};
use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::NonNull;

use crate::linear::buffer_guard::BufferGuard;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Queue<T> {
    size: usize,
    length: usize,
    buffer: NonNull<T>,
    layout: Layout,
}

pub const QUEUE_START_SIZE: usize = 10;
pub const QUEUE_SIZE_INCREMENT: usize = 10;

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        let layout = Layout::array::<T>(QUEUE_START_SIZE).expect("Layout error");
        let ptr = NonNull::new(unsafe { alloc(layout) } as *mut T).expect("Allocation failed");
        Queue {
            size: QUEUE_START_SIZE,
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
            self._increment_size(QUEUE_SIZE_INCREMENT);
        }
        unsafe {
            *self.buffer.as_ptr().add(self.length) = item;
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;
            let to_return = unsafe { ptr::read(self.buffer.as_ptr()) };
            // Move all next elements by one position - O(N) Complexity with this simple implementation
            for i in 0..self.length {
                unsafe {
                    ptr::write(
                        self.buffer.as_ptr().add(i),
                        ptr::read(self.buffer.as_ptr().add(i + 1)),
                    );
                }
            }
            return Some(to_return);
        }
        None
    }
    pub fn head(&mut self) -> Option<&T> {
        if self.length > 0 {
            return Some(unsafe { &*self.buffer.as_ptr() });
        }
        None
    }
    pub fn head_mut(&mut self) -> Option<&mut T> {
        if self.length > 0 {
            return Some(unsafe { &mut *self.buffer.as_ptr() });
        }
        None
    }
}

impl<T: Display> Display for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Head -> [")?;
        for i in 0..self.length {
            write!(f, "{}", unsafe { self.buffer.as_ptr().add(i).read() })?;
            if i != self.length - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "] <- Tail")
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            // Drop every element to ensure custom and deep drops are executed
            for i in 0..self.length {
                ptr::drop_in_place(self.buffer.as_ptr().add(i));
            }
            // Drop the buffer array
            dealloc(self.buffer.as_ptr() as _, self.layout);
        }
    }
}
