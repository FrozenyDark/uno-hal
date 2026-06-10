macro_rules! add_number_writable {
    (($name_u:ident: $type_u:ty, $name_i:ident: $type_i:ty) = $digits:literal) => {
        #[inline(never)]
        fn $name_u(&mut self, mut n: $type_u) -> usize {
            let mut arr = [0u8; $digits];

            for i in arr.iter_mut() {
                *i = (n % 10) as u8 + b'0';
                n /= 10;

                if n == 0 {
                    break;
                }
            }

            self.write_iter(arr.iter().rev())
        }

        #[inline]
        fn $name_i(&mut self, n: $type_i) -> usize {
            if n < 0 {
                self.write_c(b'-') + self.$name_u(n.unsigned_abs())
            } else {
                self.$name_u(n as $type_u)
            }
        }
    };
}

pub trait Writable {
    fn write_c(&mut self, c: u8) -> usize;

    #[inline]
    fn write_iter<'a, T: Iterator<Item = &'a u8>>(&mut self, iter: T) -> usize {
        iter.fold(0, |acc, &x| acc + self.write_c(x))
    }

    #[inline]
    fn write(&mut self, s: &str) -> usize {
        self.write_iter(s.as_bytes().iter())
    }

    #[inline]
    fn writeln(&mut self, data: &str) -> usize {
        self.write_iter(data.as_bytes().iter().chain(b"\n"))
    }

    add_number_writable!((write_u8: u8, write_i8: i8) = 3);
    add_number_writable!((write_u16: u16, write_i16: i16) = 5);
    add_number_writable!((write_u32: u32, write_i32: i32) = 10);

    #[inline(never)]
    fn write_f32(&mut self, mut n: f32, digits: u8) -> usize {
        let mut sum = 0;

        if n < 0.0 {
            sum += self.write_c(b'-');
            n = -n;
        }

        n += 0.5 / (10.0 * digits as f32);

        let int_part = n as u32;
        sum += self.write_u32(int_part);

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
