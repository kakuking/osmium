use std::ops::BitAnd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    signature: u32
}

impl Signature {
    pub fn new() -> Self {
        Self {
            signature: 0
        }
    }

    pub fn reset(&mut self) {
        self.signature = 0;
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.signature |= 1 << index;
        } else {
            self.signature &= 0 << index;
        }
    }

    pub fn has(&self, index: usize) -> bool {
        (self.signature & (1 << index)) != 0
    }
}

impl BitAnd for Signature {
    type Output = Signature;

    fn bitand(self, rhs: Self) -> Self::Output {
        Signature {
            signature: self.signature & rhs.signature,
        }
    }
}