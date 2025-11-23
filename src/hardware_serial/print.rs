#[allow(dead_code)]
pub trait Print {
    fn write_c(&mut self, c: u8) -> usize;

    #[inline]
    fn write_buffer(&mut self, buffer: &[u8]) -> usize {
        buffer.iter().fold(0, |acc, &x| acc + self.write_c(x))
    }

    #[inline]
    fn write(&mut self, s: &str) -> usize {
        self.write_buffer(s.as_bytes())
    }

    #[inline]
    fn writeln(&mut self, data: &str) -> usize {
        self.write(data) + self.write_c(b'\n')
    }

    #[inline(never)]
    fn write_number(&mut self, mut n: u32) -> usize {
        let mut arr = [0u8; 8];

        for i in arr.iter_mut().rev() {
            *i = (n % 10) as u8 + b'0';
            n /= 10;

            if n == 0 {
                break;
            }
        }

        self.write_buffer(&arr)
    }

    #[inline(never)]
    fn write_float(&mut self, mut n: f32, digits: u8) -> usize {
        let mut sum = 0;

        if n < 0.0 {
            sum += self.write_c(b'-');
            n = -n;
        }

        n += 0.5 / (10.0 * digits as f32);

        let int_part = n as u32;
        sum += self.write_number(int_part);

        if digits > 0 {
            sum += self.write_c(b'.');
        }

        let mut remainder = n - int_part as f32;

        for _ in 0..digits {
            remainder *= 10.0;
            let to_print = remainder as u8 + b'0';
            sum += self.write_c(to_print);
            remainder -= to_print as f32;
        }

        sum
    }

    #[inline]
    fn flush(&mut self) {}

    #[inline]
    fn available_for_write(&self) -> usize {
        0
    }
}
