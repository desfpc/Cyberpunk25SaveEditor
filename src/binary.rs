use anyhow::{ensure, Context, Result};
pub struct Reader<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}
impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }
    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(n).context("Offset overflow")?;
        let x = self.data.get(self.pos..end).context("Truncated save")?;
        self.pos = end;
        Ok(x)
    }
    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }
    pub fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.bytes(2)?.try_into()?))
    }
    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into()?))
    }
    pub fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.bytes(8)?.try_into()?))
    }
    pub fn vlq(&mut self) -> Result<i32> {
        let b = self.u8()?;
        let neg = b & 128 != 0;
        let mut v = (b & 63) as u64;
        let mut more = b & 64 != 0;
        let mut shift = 6;
        while more {
            ensure!(shift <= 27, "Invalid VLQ");
            let b = self.u8()?;
            v |= ((b & 127) as u64) << shift;
            shift += 7;
            more = b & 128 != 0;
        }
        ensure!(v <= i32::MAX as u64, "VLQ overflow");
        Ok(if neg { -(v as i32) } else { v as i32 })
    }
    pub fn string(&mut self) -> Result<String> {
        let n = self.vlq()?;
        ensure!(n.unsigned_abs() < 1_000_000, "String too long");
        if n <= 0 {
            Ok(std::str::from_utf8(self.bytes(n.unsigned_abs() as usize)?)?.to_owned())
        } else {
            let b = self.bytes(n as usize * 2)?;
            Ok(String::from_utf16(
                &b.as_chunks::<2>()
                    .0
                    .iter()
                    .map(|x| u16::from_le_bytes([x[0], x[1]]))
                    .collect::<Vec<_>>(),
            )?)
        }
    }
}
pub fn u32_at(b: &[u8], p: usize) -> Result<u32> {
    Reader::new(b, p).u32()
}
pub fn put32(b: &mut [u8], p: usize, v: u32) -> Result<()> {
    b.get_mut(p..p.checked_add(4).context("Offset overflow")?)
        .context("Patch outside save")?
        .copy_from_slice(&v.to_le_bytes());
    Ok(())
}
pub fn vlq(v: i32) -> Vec<u8> {
    let mut n = v.unsigned_abs();
    let mut b = (n & 63) as u8 | if v < 0 { 128 } else { 0 };
    n >>= 6;
    if n > 0 {
        b |= 64;
    }
    let mut out = vec![b];
    while n > 0 {
        let mut b = (n & 127) as u8;
        n >>= 7;
        if n > 0 {
            b |= 128;
        }
        out.push(b);
    }
    out
}
pub fn string(s: &str) -> Vec<u8> {
    let mut b = vlq(-(s.len() as i32));
    b.extend(s.as_bytes());
    b
}
