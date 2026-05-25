use core::str::from_utf8;

use crate::get_ms;

pub trait Readable {
    fn peek_c(&self) -> Option<u8>;
    fn read_c(&self) -> Option<u8>;

    fn read_timed(&self, timeout: u32) -> Option<u8> {
        let mut c = self.read_c();
        let start = get_ms();

        while c.is_none() && (get_ms() - start) < timeout {
            c = self.read_c()
        }

        c
    }

    fn read_bytes_until<const N: usize>(
        &self,
        buffer: &mut [u8; N],
        terminator: u8,
        timeout: u32,
    ) -> usize {
        let mut count = 0;

        while count < buffer.len() {
            let Some(c) = self.read_timed(timeout) else {
                break;
            };
            if c == terminator {
                break;
            }

            buffer[count] = c;
            count += 1;
        }

        count
    }

    fn read_bytes<const N: usize>(&self, buffer: &mut [u8; N], timeout: u32) -> usize {
        self.read_bytes_until(buffer, 0, timeout)
    }

    fn find_multiple<const N: usize>(&self, targets: [&str; N], timeout: u32) -> Option<usize> {
        let mut targets = targets.map(|x| (x.as_bytes(), 0usize));

        loop {
            let c = self.read_timed(timeout)?;

            for (i, (target, count)) in targets.iter_mut().enumerate() {
                if c == target[*count] {
                    *count += 1;
                } else {
                    *count = 0;
                }

                if *count == target.len() {
                    return Some(i);
                }
            }
        }
    }

    fn find_until(&self, target: &str, terminator: u8, timeout: u32) -> bool {
        let terminator = [terminator];
        let terminator = from_utf8(&terminator).unwrap();
        self.find_multiple([target, terminator], timeout) == Some(0)
    }

    fn find(&self, target: &str, timeout: u32) -> bool {
        self.find_until(target, 0, timeout)
    }

    fn available(&self) -> usize {
        0
    }
}
