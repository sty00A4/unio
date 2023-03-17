#![allow(dead_code, unused_macros)]
use std::{
    fmt::{Display, Debug},
    cmp::Ordering, ops::{Add, Sub, Mul, Div}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeUnit {
    Meter,
    Liter,
    Gramm,
    Second, Minute, Hour, Day, Week, Year
}
impl Display for NativeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Meter => write!(f, "m"),
            Self::Liter => write!(f, "l"),
            Self::Gramm => write!(f, "g"),
            Self::Second => write!(f, "s"),
            Self::Minute => write!(f, "min"),
            Self::Hour => write!(f, "h"),
            Self::Day => write!(f, "d"),
            Self::Week => write!(f, "w"),
            Self::Year => write!(f, "y"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitKind {
    Pro(Box<Self>, Box<Self>), // km / h
    Per(Box<Self>, Box<Self>), // m * s
    Pow(Box<Self>, usize), // m ^ 2
    Native(NativeUnit),
    Custom(String),
    None
}
impl Display for UnitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(native) => write!(f, "{native}"),
            Self::Custom(unit) => write!(f, "{unit}"),
            Self::Pro(u1, u2) => write!(f, "{u1}/{u2}"),
            Self::Per(u1, u2) => write!(f, "{u1}*{u2}"),
            Self::Pow(unit, pow) => write!(f, "{unit}^{pow}"),
            Self::None => Ok(()),
        }
    }
}
macro_rules! unit_per {
    ($v1:expr, $v2:expr) => {
        self::UnitKind::Per(Box::new($v1), Box::new($v2))
    };
}
macro_rules! unit_pro {
    ($v1:expr, $v2:expr) => {
        self::UnitKind::Pro(Box::new($v1), Box::new($v2))
    };
}
macro_rules! unit_pow {
    ($v1:expr, $v2:expr) => {
        self::UnitKind::Pow(Box::new($v1), $v2)
    };
}
macro_rules! native {
    ($id:ident) => {
        self::UnitKind::Native(NativeUnit::$id)
    };
}

pub struct Unit<T> {
    value: T,
    unit: UnitKind
}
impl<T> Unit<T> {
    pub fn new(value: T, unit: UnitKind) -> Self {
        Self { value, unit }
    }
    pub fn value(self) -> T {
        self.value
    }
    pub fn unit(self) -> UnitKind {
        self.unit
    }
    pub fn value_ref(&self) -> &T {
        &self.value
    }
    pub fn unit_ref(&self) -> &UnitKind {
        &self.unit
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    pub fn unit_mut(&mut self) -> &mut UnitKind {
        &mut self.unit
    }
}
impl Unit<f64> {
    pub fn powf(self, n: f64) -> Self {
        Self::new(self.value.powf(n), self.unit)
    }
    pub fn powi(self, n: i32) -> Self {
        Self::new(self.value.powi(n), self.unit)
    }
}
impl Unit<f32> {
    pub fn powf(self, n: f32) -> Self {
        Self::new(self.value.powf(n), self.unit)
    }
    pub fn powi(self, n: i32) -> Self {
        Self::new(self.value.powi(n), self.unit)
    }
}
impl Unit<i64> {
    pub fn powf(self, n: u32) -> Self {
        Self::new(self.value.pow(n), self.unit)
    }
}
impl Unit<i32> {
    pub fn powf(self, n: u32) -> Self {
        Self::new(self.value.pow(n), self.unit)
    }
}
impl<T: Display> Display for Unit<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}
impl<T: Debug> Debug for Unit<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unit({:?}, {:?})", self.value, self.unit)
    }
}
impl<T: PartialEq> PartialEq for Unit<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.unit == other.unit
    }
}
impl<T: Clone> Clone for Unit<T> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone(), self.unit.clone())
    }
}
impl<T: PartialOrd> PartialOrd for Unit<T> {
    fn gt(&self, other: &Self) -> bool {
        self.value > other.value && self.unit == other.unit
    }
    fn lt(&self, other: &Self) -> bool {
        self.value < other.value && self.unit == other.unit
    }
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value && self.unit == other.unit
    }
    fn le(&self, other: &Self) -> bool {
        self.value <= other.value && self.unit == other.unit
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(other) {
            Some(Ordering::Less)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}
impl<T: PartialOrd> Unit<T> {
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }
}
impl<T: Add<Output = T>> Add for Unit<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            panic!("cannot add {} with {}", self.unit, rhs.unit)
        }
        Self::new(self.value + rhs.value, self.unit)
    }
}
impl<T: Sub<Output = T>> Sub for Unit<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            panic!("cannot subtract {} with {}", self.unit, rhs.unit)
        }
        Self::new(self.value - rhs.value, self.unit)
    }
}
impl<T: Mul<Output = T>> Mul for Unit<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self.unit, rhs.unit) {
            (UnitKind::Pow(unit1, pow), unit2) if *unit1 == unit2 =>
                Self::new(self.value * rhs.value, UnitKind::Pow(unit1, pow + 1)),
            (UnitKind::Pro(unit1, unit2), unit3) if *unit2 == unit3 =>
                Self::new(self.value * rhs.value, *unit1),
            (unit1, unit2) if unit1 == unit2 => Self::new(self.value * rhs.value, UnitKind::Pow(Box::new(unit1), 2)),
            (unit1, unit2) => Self::new(self.value * rhs.value, UnitKind::Per(Box::new(unit1), Box::new(unit2))),
        }
    }
}
impl<T: Div<Output = T>> Div for Unit<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self.unit, rhs.unit) {
            (UnitKind::Per(unit1, unit2), unit3) if *unit2 == unit3 =>
                Self::new(self.value / rhs.value, *unit1),
            (unit1, unit2) if unit1 == unit2 => Self::new(self.value / rhs.value, UnitKind::None),
            (unit1, unit2) => Self::new(self.value / rhs.value, UnitKind::Pro(Box::new(unit1), Box::new(unit2))),
        }
    }
}
impl<T: Mul<isize, Output = T>> Mul<isize> for Unit<T> {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.value * rhs, self.unit)
    }
}
impl<T: Mul<f64, Output = T>> Mul<f64> for Unit<T> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.value * rhs, self.unit)
    }
}
impl<T: Mul<f32, Output = T>> Mul<f32> for Unit<T> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.value * rhs, self.unit)
    }
}
impl<T: Div<isize, Output = T>> Div<isize> for Unit<T> {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self::new(self.value / rhs, self.unit)
    }
}
impl<T: Div<f64, Output = T>> Div<f64> for Unit<T> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.value / rhs, self.unit)
    }
}
impl<T: Div<f32, Output = T>> Div<f32> for Unit<T> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.value / rhs, self.unit)
    }
}
impl<T: Neg<Output = T>> Neg for Unit<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.value, self.unit)
    }
}

#[macro_export]
macro_rules! unit {
    ($v:expr, $unit:expr) => {
        self::Unit::new($v, $unit)
    };
}
#[macro_export]
macro_rules! meter {
    ($v:expr) => {
        self::Unit::new($v, UnitKind::Native(NativeUnit::Meter))
    };
}
#[macro_export]
macro_rules! liter {
    ($v:expr) => {
        ::Unit::new($v, UnitKind::Native(NativeUnit::Liter))
    };
}
#[macro_export]
macro_rules! gramm {
    ($v:expr) => {
        ::Unit::new($v, UnitKind::Native(NativeUnit::Gramm))
    };
}
#[macro_export]
macro_rules! second {
    ($v:expr) => {
        self::Unit::new($v, UnitKind::Native(NativeUnit::Second))
    };
}
#[macro_export]
macro_rules! minute {
    ($v:expr) => {
        ::Unit::new($v, UnitKind::Native(NativeUnit::Minute))
    };
}
#[macro_export]
macro_rules! hour {
    ($v:expr) => {
        ::Unit::new($v, UnitKind::Native(NativeUnit::Hour))
    };
}
#[macro_export]
macro_rules! day {
    ($v:expr) => {
        ::Unit::new($v, UnitKind::Native(NativeUnit::Day))
    };
}
#[macro_export]
macro_rules! year {
    ($v:expr) => {
        ::Unit::new($v, UnitKind::Native(NativeUnit::Year))
    };
}
#[macro_export]
macro_rules! m_pro_s {
    ($v:expr) => {
        unit!($v, unit_pro!(native!(Meter), native!(Second)))
    };
}
#[macro_export]
macro_rules! area {
    ($v:expr) => {
        unit!($v, unit_pow!(native!(Meter), 2))
    };
}
#[macro_export]
macro_rules! volume {
    ($v:expr) => {
        unit!($v, unit_pow!(native!(Meter), 3))
    };
}

#[cfg(test)]
mod tests;