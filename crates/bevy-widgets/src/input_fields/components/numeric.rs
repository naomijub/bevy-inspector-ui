use bevy::prelude::*;
use num_traits::{clamp, Bounded, CheckedAdd, CheckedSub, NumCast};
use std::cmp::PartialOrd;
use std::ops::{Add, Bound, RangeBounds, Sub};
use std::str::FromStr;

use crate::input_fields::builder::NumericFieldBuilder;

use super::AllowedCharSet;

const SIGNED_CHAR_SET: &[char] = &['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const UNSIGNED_CHAR_SET: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const FLOAT_CHAR_SET: &[char] = &['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

/// Represents a numeric field with optional constraints
#[derive(Component, Reflect)]
pub struct NumericField<T: NumericFieldValue> {
    /// Current value
    pub(crate) value: T,
    /// Minimum allowed value
    pub(crate) min: Option<T>,
    /// Maximum allowed value
    pub(crate) max: Option<T>,
    /// End inclusive range
    pub(crate) end_inclusive: bool,
    /// Value change per logical pixel during mouse drag
    pub(crate) drag_step: Option<T>,
}

impl<T: NumericFieldValue> From<NumericFieldBuilder<T>> for NumericField<T> {
    fn from(value: NumericFieldBuilder<T>) -> Self {
        Self {
            value: value.value.unwrap_or_default(),
            min: value.min,
            max: value.max,
            end_inclusive: value.end_inclusive,
            drag_step: value.drag_step.or_else(|| Some(T::default_drag_step())),
        }
    }
}

/// Trait defining requirements for numeric field values
pub trait NumericFieldValue:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + NumCast
    + PartialEq
    + Send
    + Sync
    + 'static
    + ToString
    + FromStr
    + Bounded
    + Default
{
    /// Default change per logical pixel during dragging
    fn default_drag_step() -> Self;

    /// Chars allowed in text field for this type
    fn allowed_chars() -> AllowedCharSet;

    /// Checked addition
    fn checked_add(&self, rhs: &Self) -> Option<Self>;
    /// Checked subtraction
    fn checked_sub(&self, rhs: &Self) -> Option<Self>;
}

impl<T> NumericField<T>
where
    T: NumericFieldValue,
{
    /// Returns the current value of the numeric field.
    pub const fn get_value(&self) -> T {
        self.value
    }

    /// Sets the current value of the numeric field, clamping it to the bounds
    /// specified by `min` and `max` if present. If `end_inclusive` is true, the
    /// value is inclusive of the bounds, otherwise it is exclusive. If no bounds
    /// are specified, the value is used as is.
    pub fn set_value(&mut self, value: T) {
        let value = match (self.min, self.max, self.end_inclusive) {
            (Some(min), Some(max), true) => clamp(value, min, max),
            (Some(min), None, true) => {
                if value >= min {
                    value
                } else {
                    min
                }
            }
            (Some(min), None, false) => {
                if value > min {
                    value
                } else {
                    min
                }
            }
            (None, Some(max), true) => {
                if value <= max {
                    value
                } else {
                    max
                }
            }
            (None, Some(max), false) => {
                if value < max {
                    value
                } else {
                    max
                }
            }
            (None, None, _) => value,
            (Some(min), Some(max), false) => {
                if value < min {
                    min
                } else if value >= max {
                    max
                } else {
                    value
                }
            }
        };

        self.value = value;
    }

    /// Sets the bounds of the numeric field, allowing you to specify a range of values which can be input into the field.
    ///
    /// The `range` parameter should be a type which implements the `RangeBounds` trait, such as `std::ops::Range<T>`
    /// or `std::ops::RangeInclusive<T>`.
    pub fn set_bounds(&mut self, range: impl RangeBounds<T>) {
        self.min = match range.start_bound() {
            Bound::Excluded(min) | Bound::Included(min) => Some(*min),
            Bound::Unbounded => None,
        };
        self.max = match range.end_bound() {
            Bound::Excluded(max) | Bound::Included(max) => Some(*max),
            Bound::Unbounded => None,
        };
        self.end_inclusive = match range.end_bound() {
            Bound::Excluded(_) => false,
            Bound::Included(_) | Bound::Unbounded => true,
        };
    }
}

// Macro to implement NumericFieldValue for signed integer types
macro_rules! impl_signed_numeric_field_value {
    ($($t:ty),*) => {
        $(
            impl NumericFieldValue for $t {
                fn default_drag_step() -> $t { 1 }

                fn allowed_chars() -> AllowedCharSet {
                    AllowedCharSet::new(SIGNED_CHAR_SET.to_vec())
                }

                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedAdd>::checked_add(self, rhs)
                }
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedSub>::checked_sub(self, rhs)
                }
            }
        )*
    }
}

// Macro to implement NumericFieldValue for unsigned integer types
macro_rules! impl_unsigned_numeric_field_value {
    ($($t:ty),*) => {
        $(
            impl NumericFieldValue for $t {
                fn default_drag_step() -> $t { 1 }

                fn allowed_chars() -> AllowedCharSet {
                    AllowedCharSet::new(UNSIGNED_CHAR_SET.to_vec())
                }

                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedAdd>::checked_add(self, rhs)
                }
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedSub>::checked_sub(self, rhs)
                }
            }
        )*
    }
}

// Implement NumericFieldValue for signed and unsigned integer types
impl_signed_numeric_field_value!(i8, i16, i32, i64, i128);
impl_unsigned_numeric_field_value!(u8, u16, u32, u64, u128);

// Implement NumericFieldValue for f32
impl NumericFieldValue for f32 {
    fn default_drag_step() -> Self {
        0.1
    }

    fn allowed_chars() -> AllowedCharSet {
        AllowedCharSet::new(FLOAT_CHAR_SET.to_vec())
    }

    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        Some(*self + *rhs)
    }

    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(*self - *rhs)
    }
}

// Implement NumericFieldValue for f64
impl NumericFieldValue for f64 {
    fn default_drag_step() -> Self {
        0.1
    }

    fn allowed_chars() -> AllowedCharSet {
        AllowedCharSet::new(FLOAT_CHAR_SET.to_vec())
    }

    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        Some(*self + *rhs)
    }

    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(*self - *rhs)
    }
}
