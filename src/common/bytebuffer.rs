pub struct ByteBuffer<const N: usize> {
    head: u8,
    tail: u8,
    buffer: [u8; N],
}

impl<const N: usize> ByteBuffer<N> {
    pub const fn new() -> Self {
        Self {
            head: 0,
            tail: 0,
            buffer: [0; N],
        }
    }

    #[inline]
    fn next_head(&self) -> u8 {
        (self.head + 1) % N as u8
    }

    #[inline]
    fn next_tail(&self) -> u8 {
        (self.tail + 1) % N as u8
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.next_head() == self.tail
    }

    #[inline]
    pub fn add(&mut self, byte: u8) -> bool {
        if self.is_full() {
            return false;
        }

        self.buffer[self.head as usize] = byte;
        self.head = self.next_head();

        true
    }

    #[inline]
    pub fn count_empty(&self) -> u8 {
        if self.head >= self.tail {
            N as u8 - 1 - self.head + self.tail
        } else {
            self.tail - self.head - 1
        }
    }

    #[inline]
    pub fn count_filled(&self) -> u8 {
        (N as u8 + self.head - self.tail) % N as u8
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(self.buffer[self.tail as usize])
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        if let Some(byte) = self.peek() {
            self.tail = self.next_tail();

            Some(byte)
        } else {
            None
        }
    }
}
