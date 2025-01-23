use std::alloc::{alloc, dealloc, Layout};
use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::NonNull;

use super::buffer_guard::BufferGuard;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Deque<T> {
    size: usize,
    length: usize,
    buffer: NonNull<T>,
    layout: Layout,
    // Indexes for circular array (and reduce most ops to O(1))
    head_index: usize,
    tail_index: usize,
}

pub const DEQUE_START_SIZE: usize = 10;
pub const DEQUE_SIZE_INCREMENT: usize = 10;

impl<T: Clone> Deque<T> {
    pub fn new() -> Deque<T> {
        let layout = Layout::array::<T>(DEQUE_START_SIZE).expect("Layout error");
        let ptr = NonNull::new(unsafe { alloc(layout) } as *mut T).expect("Allocation failed");
        Deque {
            size: DEQUE_START_SIZE,
            length: 0,
            buffer: ptr,
            layout,
            head_index: 0,
            tail_index: 0,
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
                    ptr::read(self.buffer.as_ptr().add((self.head_index + i) % self.length)),
                )
            };
        }
        // Deallocate the old buffer and replace it with the new in the buffer guard
        unsafe { dealloc(self.buffer.as_ptr() as *mut u8, self.layout) };
        (self.buffer, self.layout) = buffer_guard.into_inner();
        self.size = new_size;
        self.head_index = 0;
        if self.length > 0 { self.length - 1 } else { 0 };
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

    pub fn push_front(&mut self, item: T) {
        if self.buffer_is_full() {
            self._increment_size(DEQUE_SIZE_INCREMENT);
        }
        unsafe {
            let mut new_head_index = self.head_index;
            if self.length != 0 {
                new_head_index = if self.head_index == 0 { self.size - 1 } else { self.head_index - 1 };
            }
            *self.buffer.as_ptr().add(new_head_index) = item;
            self.head_index = new_head_index;
        }
        self.length += 1;
    }
    pub fn push_back(&mut self, item: T) {
        if self.buffer_is_full() {
            self._increment_size(DEQUE_SIZE_INCREMENT);
        }
        unsafe {
            let mut new_tail_index = self.tail_index;
            if self.length != 0 {
                new_tail_index = (self.tail_index + 1) % self.size;
            }
            *self.buffer.as_ptr().add(new_tail_index) = item;
            self.tail_index = new_tail_index;
        }
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.length > 0 {
            let to_return = unsafe { ptr::read(self.buffer.as_ptr().add(self.head_index)) };
            self.length -= 1;
            if self.length != 0 {
                self.head_index = (self.head_index + 1) % self.size;
            }
            return Some(to_return);
        }
        None
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.length > 0 {
            let to_return = unsafe { ptr::read(self.buffer.as_ptr().add(self.tail_index)) };
            self.length -= 1;
            if self.length != 0 {
                self.tail_index = if self.tail_index == 0 { self.size - 1 } else { self.tail_index - 1 };
            }
            return Some(to_return);
        }
        None
    }

    pub fn head(&mut self) -> Option<&T> {
        if self.length > 0 {
            return Some(unsafe { &*self.buffer.as_ptr().add(self.head_index) });
        }
        None
    }
    pub fn head_mut(&mut self) -> Option<&mut T> {
        if self.length > 0 {
            return Some(unsafe { &mut *self.buffer.as_ptr().add(self.head_index) });
        }
        None
    }
    pub fn tail(&mut self) -> Option<&T> {
        if self.length > 0 {
            return Some(unsafe { &*self.buffer.as_ptr().add(self.tail_index) });
        }
        None
    }
    pub fn tail_mut(&mut self) -> Option<&mut T> {
        if self.length > 0 {
            return Some(unsafe { &mut *self.buffer.as_ptr().add(self.tail_index) });
        }
        None
    }
}

impl<T: Display> Display for Deque<T> {
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

impl<T> Drop for Deque<T> {
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
