use std::sync::atomic::{AtomicUsize, fence, Ordering};

pub struct LockFreeRingBuffer<T: Clone>
{
    buffer: Vec<T>,
    read_index: AtomicUsize,
    write_index: AtomicUsize,
    capacity: usize,
}

impl<T: Clone> LockFreeRingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize(capacity, unsafe { std::mem::MaybeUninit::uninit().assume_init() });

        Self {
            buffer,
            read_index: AtomicUsize::new(0),
            write_index: AtomicUsize::new(0),
            capacity
        }
    }

    fn push(&self, value: T) -> Result<(), T> {
        let current_write = self.write_index.load(Ordering::Relaxed);
        let next_write = (current_write + 1) % self.buffer.len();

        if next_write == self.read_index.load(Ordering::Relaxed) {
            Err(value) // Buffer is full
        } else {
            unsafe {
                std::ptr::write(self.buffer.as_ptr().add(current_write), value);
            }
            self.write_index.store(next_write, Ordering::Release);
            Ok(())
        }
    }

    fn pop(&self) -> Option<T> {
        let current_read = self.read_index.load(Ordering::Relaxed);

        if current_read == self.write_index.load(Ordering::Acquire) {
            None // Buffer is empty
        } else {
            let value = unsafe { std::ptr::read(self.buffer.as_ptr().add(current_read)) };
            self.read_index.store((current_read + 1) % self.buffer.len(), Ordering::Relaxed);
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::fence;
    use std::thread;

    use super::*;

    #[test]
    fn test_lock_free_ring_buffer() {
        let buffer = Arc::new(LockFreeRingBuffer::new(10));

        let producer = {
            let buffer = Arc::clone(&buffer);
            thread::spawn(move || {
                for i in 0..10 {
                    while let Err(_) = buffer.push(i) {
                        fence(Ordering::SeqCst);
                    }
                }
            })
        };

        let consumer = {
            let buffer = Arc::clone(&buffer);
            thread::spawn(move || {
                let mut values = Vec::new();
                for _ in 0..10 {
                    while let None = buffer.pop() {
                        fence(Ordering::SeqCst);
                    }
                }
                values
            })
        };

        producer.join().unwrap();
        let values = consumer.join().unwrap();
        assert_eq!(values, (0..10).collect::<Vec<_>>());
    }
}
