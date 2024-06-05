use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct CircularBuffer<T> {
    buffer: *mut T,
    capacity: usize,
    read_index: AtomicUsize,
    write_index: AtomicUsize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer = unsafe {
            let mut vec = Vec::with_capacity(capacity);
            vec.set_len(capacity);
            vec.as_mut_ptr()
        };

        CircularBuffer {
            buffer,
            capacity,
            read_index: AtomicUsize::new(0),
            write_index: AtomicUsize::new(0),
        }
    }

    pub fn write(&self, item: T) {
        let write_idx = self.write_index.load(Ordering::Relaxed);

        println!("write_idx = {write_idx}");

        unsafe {
            ptr::write(self.buffer.add(write_idx), item);
        }

        self.write_index
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                Some((n + 1) % self.capacity)
            })
            .unwrap();
    }

    pub fn read(&self) -> &T {
        unsafe {
            let item = self
                .buffer
                .add(self.read_index.load(Ordering::Relaxed))
                .as_ref()
                .unwrap();

            self.read_index
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                    Some((n + 1) % self.capacity)
                })
                .unwrap();

            item
        }
    }
}

impl<T> Drop for CircularBuffer<T> {
    fn drop(&mut self) {
        for i in 0..self.capacity {
            unsafe {
                ptr::drop_in_place(self.buffer.add(i));
            }
        }

        unsafe {
            Vec::from_raw_parts(self.buffer, 0, self.capacity);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn basic() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(2);

        buffer.write(1);
        buffer.write(2);

        assert_eq!(*buffer.read(), 1);
        assert_eq!(*buffer.read(), 2);
        assert_eq!(*buffer.read(), 1);
    }

    #[test]
    fn wrap_around() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(2);

        buffer.write(1);
        buffer.write(2);
        buffer.write(3);

        assert_eq!(*buffer.read(), 3);
        assert_eq!(*buffer.read(), 2);
        assert_eq!(*buffer.read(), 3);
    }

    #[test]
    fn complex() {
        let buffer: CircularBuffer<Arc<i32>> = CircularBuffer::new(2);

        let v = Arc::new(123);

        buffer.write(v.clone());

        assert_eq!(**buffer.read(), 123);

        drop(buffer);

        assert_eq!(*v, 123);
    }
}
