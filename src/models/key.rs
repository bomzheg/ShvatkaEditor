use std::fmt::{Display, Formatter};

pub struct Key<'a> {
    pub type_: &'a str,
    pub value: &'a str,
}

impl Display for Key<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key={}", self.value)
    }
}
