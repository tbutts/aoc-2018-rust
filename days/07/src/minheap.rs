use std::fmt;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RChar(pub char);

impl Ord for RChar {
    fn cmp(&self, other: &RChar) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for RChar {
    fn partial_cmp(&self, other: &RChar) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for RChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl fmt::Debug for RChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RUint32(pub u32);

impl Ord for RUint32 {
    fn cmp(&self, other: &RUint32) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for RUint32 {
    fn partial_cmp(&self, other: &RUint32) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
