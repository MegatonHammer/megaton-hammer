use byteorder::ByteOrder;

// Minimal cursor implementation
pub struct CursorWrite<'a> {
    data: &'a mut [u8],
    pos: usize
}

#[allow(dead_code)]
impl<'a> CursorWrite<'a> {
    pub fn new(data: &mut [u8]) -> CursorWrite {
        CursorWrite {
            data: data,
            pos: 0
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    // Dissociate the lifetimes
    pub fn skip_write(&mut self, bytelen: usize) -> &mut [u8] {
        let ret = &mut self.data[self.pos..self.pos + bytelen];
        self.pos += bytelen;
        ret
    }
    pub fn write_u8<BO: ByteOrder>(&mut self, v: u8) {
        self.data[self.pos] = v;
        self.pos += 1;
    }
    pub fn write_u16<BO: ByteOrder>(&mut self, v: u16) {
        BO::write_u16(&mut self.data[self.pos..], v);
        self.pos += 2;
    }
    pub fn write_u32<BO: ByteOrder>(&mut self, v: u32) {
        BO::write_u32(&mut self.data[self.pos..], v);
        self.pos += 4;
    }
    pub fn write_u64<BO: ByteOrder>(&mut self, v: u64) {
        BO::write_u64(&mut self.data[self.pos..], v);
        self.pos += 8;
    }

    pub fn write(&mut self, v: &[u8]) {
        self.data[self.pos..self.pos + v.len()].copy_from_slice(v);
        self.pos += v.len();
    }
}

pub struct CursorRead<'a> {
    data: &'a [u8],
    // Let's cheat
    pos: ::core::cell::Cell<usize>
}

#[allow(dead_code)]
impl<'a> CursorRead<'a> {
    pub fn new(data: &[u8]) -> CursorRead {
        CursorRead {
            data: data,
            pos: 0.into()
        }
    }

    pub fn pos(&self) -> usize {
        self.pos.get()
    }

    // Dissociate the lifetimes
    pub fn read_u8<BO: ByteOrder>(&self) -> u8 {
        let ret = self.data[self.pos.get()];
        self.pos.set(self.pos.get() + 1);
        ret
    }
    pub fn read_u16<BO: ByteOrder>(&self) -> u16 {
        let ret = BO::read_u16(&self.data[self.pos.get()..]);
        self.pos.set(self.pos.get() + 2);
        ret
    }
    pub fn read_u32<BO: ByteOrder>(&self) -> u32 {
        let ret = BO::read_u32(&self.data[self.pos.get()..]);
        self.pos.set(self.pos.get() + 4);
        ret
    }
    pub fn read_u64<BO: ByteOrder>(&self) -> u64 {
        let ret = BO::read_u64(&self.data[self.pos.get()..]);
        self.pos.set(self.pos.get() + 8);
        ret
    }

    pub fn assert(&self, v: &[u8]) {
        assert_eq!(&self.data[self.pos.get()..self.pos.get() + v.len()], v);
        self.pos.set(self.pos.get() + v.len());
    }

    pub fn skip_read(&self, bytelen: usize) -> &[u8] {
        let ret = &self.data[self.pos.get()..self.pos.get() + bytelen];
        self.pos.set(self.pos.get() + bytelen);
        ret
    }
}

pub fn hex_print<T: ::core::fmt::Write>(arr: &[u8], f: &mut T) {
    for (i, chunk) in arr.chunks(16).enumerate() {
        // Print the current offset (do some padding if necessary so it all
        // aligns correctly).
        let log2 = 64 - arr.len().leading_zeros();
        let log2 = if log2 % 4 == 0 { log2 / 4 } else { (log2 / 4) + 1 };
        let _ = write!(f, "{:01$x}:", i * 16, log2 as usize);

        // Print the bytes one by one. Put an extra space in the middle
        for (i, b) in chunk.iter().enumerate() {
            if i % 2 == 0 {
                let _ = write!(f, " ");
            }
            let _ = write!(f, "{:02x}", b);
        }
        // Fill missing with spaces.
        for _ in 0..16 - chunk.len() {
            let _ = write!(f, "{}", "   ");
        }

        // And now show the ASCII representation. Replace unprintable
        // characters with  a '.'
        let _ = write!(f, "  ");
        for b in chunk {
            let _ = write!(f, "{}", if (*b as char).is_ascii_graphic() { *b as char } else { '.' });
        }
        let _ = writeln!(f, "");
    }
}

pub fn div_ceil(a: u64, b: u64) -> u64 {
    1 + ((a - 1) / b)
}
