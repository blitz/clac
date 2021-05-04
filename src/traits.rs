use std::str::FromStr;
use std::string::ToString;

/// A trait for any kind of numbers we care about
pub trait Number: Clone + Copy + FromStr + ToString + PartialEq + Eq {
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
}

impl Number for i64 {
    fn checked_add(self, rhs: Self) -> std::option::Option<Self> {
        self.checked_add(rhs)
    }

    fn checked_sub(self, rhs: Self) -> std::option::Option<Self> {
        self.checked_sub(rhs)
    }

    fn checked_mul(self, rhs: Self) -> std::option::Option<Self> {
        self.checked_sub(rhs)
    }

    fn checked_div(self, rhs: Self) -> std::option::Option<Self> {
        self.checked_div(rhs)
    }
}
