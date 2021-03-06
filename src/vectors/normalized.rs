use super::*;
use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Normalized<V: Vector>(pub(super) V);

impl<V: Vector> fmt::Display for Normalized<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<V: Vector> ops::Deref for Normalized<V> {
    type Target = V;

    fn deref(&self) -> &V {
        &self.0
    }
}

impl<V: Vector> ops::Neg for Normalized<V> {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

macro_rules! normalized_op {
    ($op_name:ident, $fn:ident, $rhs:ty) => {
        impl<V: Vector> ops::$op_name<$rhs> for Normalized<V> {
            type Output = V;

            fn $fn(self, rhs: $rhs) -> V {
                (self.0).$fn(rhs)
            }
        }
    };
}

normalized_op!(Add, add, V);
normalized_op!(Add, add, Normalized<V>);
normalized_op!(Sub, sub, V);
normalized_op!(Sub, sub, Normalized<V>);
normalized_op!(Mul, mul, Component);
normalized_op!(Div, div, Component);

impl Normalized<Vec3> {
    pub fn cross(self, rhs: Self) -> Self {
        Self(self.0.cross(rhs))
    }
}
