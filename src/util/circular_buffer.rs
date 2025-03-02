use core::mem::MaybeUninit;

use thiserror_no_std::Error;

#[derive(Error, Debug)]
#[error("Failed to push item into buffer")]
pub struct PushError;

pub struct CircularBuffer<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> CircularBuffer<T, N> {
    pub const fn new() -> Self {
        Self {
            array: MaybeUninit::uninit_array(),
            head: 0,
            tail: 0,
        }
    }

    pub fn push_back(&mut self, item: T) -> Result<(), PushError> {
        let insertion_index = self.tail;

        match self.increment_tail() {
            Some(tail) => self.tail = tail,

            None => return Err(PushError),
        }

        self.array[insertion_index] = MaybeUninit::new(item);

        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let index = self.head;

        if self.head == self.tail {
            return None;
        }

        wrapping_increment(&mut self.head, N);

        let mut ret = MaybeUninit::uninit();

        core::mem::swap(&mut ret, &mut self.array[index]);

        unsafe { Some(ret.assume_init()) }
    }

    fn increment_tail(&mut self) -> Option<usize> {
        if self.tail == N + 1 {
            return None;
        }

        wrapping_increment(&mut self.tail, N);

        if self.tail == self.head {
            self.tail = N + 1;
        }

        Some(self.tail)
    }
}

fn wrapping_increment(i: &mut usize, container_size: usize) {
    *i = (*i + 1) % container_size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_case]
    fn test_pop_empty() {
        print_serial!("[TEST] Assert that safe pop from empty buffer ... ");

        let mut buffer: CircularBuffer<i32, 4> = CircularBuffer::new();
        assert_eq!(buffer.pop_front(), None);

        println_serial!("✓");
    }

    #[test_case]
    fn test_buffer_full() {
        print_serial!("[TEST] Assert that buffer is full ... ");

        let mut buffer: CircularBuffer<i32, 4> = CircularBuffer::new();
        buffer.push_back(1).unwrap();
        buffer.push_back(2).unwrap();
        buffer.push_back(3).unwrap();
        buffer.push_back(4).unwrap();

        assert_eq!(buffer.push_back(5).is_err(), true);

        println_serial!("✓");
    }

    #[test_case]
    fn test_buffer() {
        print_serial!("[TEST] Assert buffer functionality ... ");

        let mut buffer: CircularBuffer<i32, 4> = CircularBuffer::new();

        buffer.push_back(1).unwrap();
        buffer.push_back(2).unwrap();
        assert_eq!(buffer.pop_front(), Some(1));

        buffer.push_back(3).unwrap();
        buffer.push_back(4).unwrap();
        assert_eq!(buffer.pop_front(), Some(2));
        assert_eq!(buffer.pop_front(), Some(3));
        assert_eq!(buffer.pop_front(), Some(4));

        println_serial!("✓");
    }
}
