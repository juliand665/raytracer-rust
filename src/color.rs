use super::Component;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: Component,
    pub green: Component,
    pub blue: Component,
    pub alpha: Component,
}

impl Color {
    pub fn clear() -> Self {
        Self::new_gray(0.0, 0.0)
    }

    pub fn black() -> Self {
        Self::new_gray(0.0, 1.0)
    }

    pub fn white() -> Self {
        Self::new_gray(1.0, 1.0)
    }

    pub fn new(red: Component, green: Component, blue: Component, alpha: Component) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn new_gray(brightness: Component, alpha: Component) -> Self {
        Self {
            red: brightness,
            green: brightness,
            blue: brightness,
            alpha,
        }
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha, // TODO is this appropriate?
        }
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
            alpha: self.alpha * rhs.alpha, // TODO is this appropriate?
        }
    }
}
