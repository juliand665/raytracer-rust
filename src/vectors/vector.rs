use super::*;
use std::fmt;
use std::ops;

pub type Component = f32;

// TODO: try using `Into<Self>` for `Add` etc. rather than implementing twice

pub trait Vector:
    'static
    + Send
    + Sync
    + Sized
    + Copy
    + Clone
    + PartialEq
    + fmt::Display
    + From<Normalized<Self>>
    + ops::Neg<Output = Self>
    + ops::Add<Output = Self>
    + ops::Add<Normalized<Self>, Output = Self>
    + ops::AddAssign
    + ops::AddAssign<Normalized<Self>>
    + ops::Sub<Output = Self>
    + ops::Sub<Normalized<Self>, Output = Self>
    + ops::SubAssign
    + ops::SubAssign<Normalized<Self>>
    + ops::Mul<Component, Output = Self>
    + ops::MulAssign<Component>
    + ops::Div<Component, Output = Self>
    + ops::DivAssign<Component>
{
    fn zero() -> Self;

    fn squared_sum(self) -> Component;
    fn norm(self) -> Component;

    fn dot<R: Into<Self>>(self, rhs: R) -> Component;

    fn normalized(self) -> Normalized<Self>;
}

macro_rules! vec_type {
    ($type:ident($($component:ident)*)) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $type {
            $(pub $component: Component),*
        }

        impl $type {
            pub fn new($($component: Component,)*) -> Self {
                Self { $($component),* }
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({})", [$(self.$component.to_string()),*].join(", "))
            }
        }

        impl Vector for $type {
            fn zero() -> Self {
                Self { $($component: 0.0),* }
            }

            fn squared_sum(self) -> Component {
                self.dot(self)
            }

            fn norm(self) -> Component {
                self.squared_sum().sqrt()
            }

            fn dot<R: Into<Self>>(self, rhs: R) -> Component {
                let rhs: Self = rhs.into();
                0.0 $(+ self.$component * rhs.$component)*
            }

            fn normalized(self) -> Normalized<Self> {
                let length = self.norm();
                if length > 0.0 && length != 1.0 {
                    Normalized(self / length)
                } else {
                    Normalized(self)
                }
            }
        }

        impl From<Normalized<Self>> for $type {
            fn from(normalized: Normalized<Self>) -> Self {
                normalized.0
            }
        }

        impl ops::Neg for $type {
            type Output = Self;

            fn neg(self) -> Self {
                Self { $($component: -self.$component),* }
            }
        }

        macro_rules! bin_op {
            (cwise $rhs:ty, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<$rhs> for $type {
                    type Output = Self;

                    fn $fn(self, rhs: $rhs) -> Self {
                        Self { $($component: self.$component $op rhs.$component),* }
                    }
                }
            };
            (cwise, $op_name:ident, $fn:ident, $op:tt) => {
                bin_op!(cwise Self, $op_name, $fn, $op);
                bin_op!(cwise Normalized<Self>, $op_name, $fn, $op);
            };
            (linear, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<Component> for $type {
                    type Output = Self;

                    fn $fn(self, rhs: Component) -> Self {
                        Self { $($component: self.$component $op rhs),* }
                    }
                }
            };
        }

        bin_op!(cwise, Add, add, +);
        bin_op!(cwise, Sub, sub, -);
        bin_op!(linear, Mul, mul, *);
        bin_op!(linear, Div, div, /);

        macro_rules! bin_op_assign {
            (cwise $rhs:ty, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<$rhs> for $type {
                    fn $fn(&mut self, rhs: $rhs) {
                        $(self.$component $op rhs.$component;)*
                    }
                }
            };
            (cwise, $op_name:ident, $fn:ident, $op:tt) => {
                bin_op_assign!(cwise Self, $op_name, $fn, $op);
                bin_op_assign!(cwise Normalized<Self>, $op_name, $fn, $op);
            };
            (linear, $op_name:ident, $fn:ident, $op:tt) => {
                impl ops::$op_name<Component> for $type {
                    fn $fn(&mut self, rhs: Component) {
                        $(self.$component $op rhs;)*
                    }
                }
            };
        }

        bin_op_assign!(cwise, AddAssign, add_assign, +=);
        bin_op_assign!(cwise, SubAssign, sub_assign, -=);
        bin_op_assign!(linear, MulAssign, mul_assign, *=);
        bin_op_assign!(linear, DivAssign, div_assign, /=);
    };
}

vec_type!(Vec2(x y));
vec_type!(Vec3(x y z));
vec_type!(Vec4(x y z w));

impl Vec3 {
    pub fn positive_x() -> Normalized<Self> {
        Normalized(Self::new(1.0, 0.0, 0.0))
    }

    pub fn negative_x() -> Normalized<Self> {
        Normalized(Self::new(-1.0, 0.0, 0.0))
    }

    pub fn positive_y() -> Normalized<Self> {
        Normalized(Self::new(0.0, 1.0, 0.0))
    }

    pub fn negative_y() -> Normalized<Self> {
        Normalized(Self::new(0.0, -1.0, 0.0))
    }

    pub fn positive_z() -> Normalized<Self> {
        Normalized(Self::new(0.0, 0.0, 1.0))
    }

    pub fn negative_z() -> Normalized<Self> {
        Normalized(Self::new(0.0, 0.0, -1.0))
    }

    pub fn cross<R: Into<Self>>(self, rhs: R) -> Self {
        let rhs: Self = rhs.into();
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}
