use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter::repeat,
    mem::{size_of, take},
};

use ecow::EcoVec;

use crate::{
    algorithm::{pervade::*, FillContext},
    array::*,
    boxed::Boxed,
    cowslice::CowSlice,
    grid_fmt::GridFmt,
    Complex, Uiua, UiuaResult,
};

/// A generic array value
///
/// This enum is used to represent all possible array types.
#[derive(Clone)]
#[repr(C)]
pub enum Value {
    /// Common number array
    Num(Array<f64>),
    /// Byte array used for some boolean operations and for I/O
    #[cfg(feature = "bytes")]
    Byte(Array<u8>),
    /// Complex number array
    Complex(Array<Complex>),
    /// Common character array
    Char(Array<char>),
    /// Common box array
    Box(Array<Boxed>),
}

impl Default for Value {
    fn default() -> Self {
        Array::<u8>::default().into()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(array) => array.fmt(f),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => array.fmt(f),
            Self::Complex(array) => array.fmt(f),
            Self::Char(array) => array.fmt(f),
            Self::Box(array) => array.fmt(f),
        }
    }
}

/// A combination of [`ExactSizeIterator`] and [`DoubleEndedIterator`]
pub trait ExactDoubleIterator: ExactSizeIterator + DoubleEndedIterator {}
impl<T: ExactSizeIterator + DoubleEndedIterator> ExactDoubleIterator for T {}

impl Value {
    pub(crate) fn builder(capacity: usize) -> ValueBuilder {
        ValueBuilder::with_capacity(capacity)
    }
    pub(crate) fn type_id(&self) -> u8 {
        match self {
            Self::Num(_) => 0,
            #[cfg(feature = "bytes")]
            Self::Byte(_) => 0,
            Self::Complex(_) => 1,
            Self::Char(_) => 2,
            Self::Box(_) => 3,
        }
    }
    /// Get a reference to a possible number array
    pub fn as_num_array(&self) -> Option<&Array<f64>> {
        match self {
            Self::Num(array) => Some(array),
            _ => None,
        }
    }
    /// Get a reference to a possible byte array
    pub fn as_byte_array(&self) -> Option<&Array<u8>> {
        match self {
            #[cfg(feature = "bytes")]
            Self::Byte(array) => Some(array),
            _ => None,
        }
    }
    /// Get a reference to a possible character array
    pub fn as_char_array(&self) -> Option<&Array<char>> {
        match self {
            Self::Char(array) => Some(array),
            _ => None,
        }
    }
    /// Get a reference to a possible box array
    pub fn as_box_array(&self) -> Option<&Array<Boxed>> {
        match self {
            Self::Box(array) => Some(array),
            _ => None,
        }
    }
    /// Get a reference to a possible scalar box
    pub fn as_box(&self) -> Option<&Boxed> {
        self.as_box_array().and_then(Array::as_scalar)
    }
    /// Get an iterator over the rows of the value
    pub fn rows(&self) -> Box<dyn ExactSizeIterator<Item = Self> + '_> {
        match self {
            Self::Num(array) => Box::new(array.rows().map(Value::from)),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => Box::new(array.rows().map(Value::from)),
            Self::Complex(array) => Box::new(array.rows().map(Value::from)),
            Self::Char(array) => Box::new(array.rows().map(Value::from)),
            Self::Box(array) => Box::new(array.rows().map(Value::from)),
        }
    }
    /// Get an iterator over the rows of the value that have the given shape
    pub fn row_shaped_slices(
        &self,
        row_shape: Shape,
    ) -> Box<dyn ExactSizeIterator<Item = Self> + '_> {
        match self {
            Self::Num(array) => Box::new(array.row_shaped_slices(row_shape).map(Value::from)),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => Box::new(array.row_shaped_slices(row_shape).map(Value::from)),
            Self::Complex(array) => Box::new(array.row_shaped_slices(row_shape).map(Value::from)),
            Self::Char(array) => Box::new(array.row_shaped_slices(row_shape).map(Value::from)),
            Self::Box(array) => Box::new(array.row_shaped_slices(row_shape).map(Value::from)),
        }
    }
    /// Get an iterator over the rows of the value that have the given shape
    pub fn into_row_shaped_slices(
        self,
        row_shape: Shape,
    ) -> Box<dyn ExactSizeIterator<Item = Self>> {
        match self {
            Self::Num(array) => Box::new(array.into_row_shaped_slices(row_shape).map(Value::from)),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => Box::new(array.into_row_shaped_slices(row_shape).map(Value::from)),
            Self::Complex(array) => {
                Box::new(array.into_row_shaped_slices(row_shape).map(Value::from))
            }
            Self::Char(array) => Box::new(array.into_row_shaped_slices(row_shape).map(Value::from)),
            Self::Box(array) => Box::new(array.into_row_shaped_slices(row_shape).map(Value::from)),
        }
    }
    /// Consume the value and get an iterator over its rows
    pub fn into_rows(self) -> Box<dyn ExactDoubleIterator<Item = Self>> {
        match self {
            Self::Num(array) => Box::new(array.into_rows().map(Value::from)),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => Box::new(array.into_rows().map(Value::from)),
            Self::Complex(array) => Box::new(array.into_rows().map(Value::from)),
            Self::Char(array) => Box::new(array.into_rows().map(Value::from)),
            Self::Box(array) => Box::new(array.into_rows().map(Value::from)),
        }
    }
    /// Cosume the value and get an iterator over its elements
    pub fn into_elements(self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Self::Num(array) => Box::new(array.data.into_iter().map(Value::from)),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => Box::new(array.data.into_iter().map(Value::from)),
            Self::Complex(array) => Box::new(array.data.into_iter().map(Value::from)),
            Self::Char(array) => Box::new(array.data.into_iter().map(Value::from)),
            Self::Box(array) => Box::new(array.data.into_iter().map(Value::from)),
        }
    }
    /// Get the value's type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Num(_) => "number",
            #[cfg(feature = "bytes")]
            Self::Byte(_) => "number",
            Self::Complex(_) => "complex",
            Self::Char(_) => "character",
            Self::Box(_) => "box",
        }
    }
    /// Get a plural form of the value's type name
    pub fn type_name_plural(&self) -> &'static str {
        match self {
            Self::Num(_) => "numbers",
            #[cfg(feature = "bytes")]
            Self::Byte(_) => "number",
            Self::Complex(_) => "complexes",
            Self::Char(_) => "characters",
            Self::Box(_) => "boxes",
        }
    }
    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.shape().first().copied().unwrap_or(1)
    }
    /// Get the number of element in each row
    pub fn row_len(&self) -> usize {
        self.shape().iter().skip(1).product()
    }
    pub(crate) fn prototype_scalar(&self, env: &Uiua) -> Self {
        match self {
            Self::Num(_) => env.num_fill().unwrap_or_else(|_| f64::prototype()).into(),
            #[cfg(feature = "bytes")]
            Self::Byte(_) => env.byte_fill().unwrap_or_else(|_| u8::prototype()).into(),
            Self::Complex(_) => env
                .complex_fill()
                .unwrap_or_else(|_| Complex::prototype())
                .into(),
            Self::Char(_) => env.char_fill().unwrap_or_else(|_| char::prototype()).into(),
            Self::Box(_) => env.box_fill().unwrap_or_else(|_| Boxed::prototype()).into(),
        }
    }
    pub(crate) fn prototype_row(&self, env: &Uiua) -> Self {
        let shape: Shape = self.shape()[1..].into();
        let elem_count = shape.iter().product();
        match self {
            Self::Num(_) => Array::new(
                shape,
                repeat(env.num_fill().unwrap_or_else(|_| f64::prototype()))
                    .take(elem_count)
                    .collect::<CowSlice<_>>(),
            )
            .into(),
            #[cfg(feature = "bytes")]
            Self::Byte(_) => Array::new(
                shape,
                repeat(env.byte_fill().unwrap_or_else(|_| u8::prototype()))
                    .take(elem_count)
                    .collect::<CowSlice<_>>(),
            )
            .into(),
            Self::Complex(_) => Array::new(
                shape,
                repeat(env.complex_fill().unwrap_or_else(|_| Complex::prototype()))
                    .take(elem_count)
                    .collect::<CowSlice<_>>(),
            )
            .into(),
            Self::Char(_) => Array::new(
                shape,
                repeat(env.char_fill().unwrap_or_else(|_| char::prototype()))
                    .take(elem_count)
                    .collect::<CowSlice<_>>(),
            )
            .into(),
            Self::Box(_) => Array::new(
                shape,
                repeat(env.box_fill().unwrap_or_else(|_| Boxed::prototype()))
                    .take(elem_count)
                    .collect::<CowSlice<_>>(),
            )
            .into(),
        }
    }
    pub(crate) fn first_dim_zero(&self) -> Self {
        match self {
            Self::Num(array) => array.first_dim_zero().into(),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => array.first_dim_zero().into(),
            Self::Complex(array) => array.first_dim_zero().into(),
            Self::Char(array) => array.first_dim_zero().into(),
            Self::Box(array) => array.first_dim_zero().into(),
        }
    }
    /// Get a formattable representation of the shape
    pub fn format_shape(&self) -> FormatShape {
        FormatShape(self.shape())
    }
    /// Get the rank
    pub fn rank(&self) -> usize {
        self.shape().len()
    }
    pub(crate) fn pop_row(&mut self) -> Option<Self> {
        match self {
            Self::Num(array) => array.pop_row().map(Value::from),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => array.pop_row().map(Value::from),
            Self::Complex(array) => array.pop_row().map(Value::from),
            Self::Char(array) => array.pop_row().map(Value::from),
            Self::Box(array) => array.pop_row().map(Value::from),
        }
    }
    pub(crate) fn elem_size(&self) -> usize {
        match self {
            Self::Num(_) => size_of::<f64>(),
            #[cfg(feature = "bytes")]
            Self::Byte(_) => size_of::<u8>(),
            Self::Complex(_) => size_of::<Complex>(),
            Self::Char(_) => size_of::<char>(),
            Self::Box(_) => size_of::<Boxed>(),
        }
    }
}

#[repr(C)]
struct Repr {
    discriminant: u8,
    arr: Array<f64>,
}

impl Value {
    /// Get the shape of the value
    pub fn shape(&self) -> &[usize] {
        let repr = unsafe { &*(self as *const Self as *const Repr) };
        &repr.arr.shape
    }
    /// Get a mutable reference to the shape
    pub fn shape_mut(&mut self) -> &mut Shape {
        let repr = unsafe { &mut *(self as *mut Self as *mut Repr) };
        &mut repr.arr.shape
    }
    /// Get the number of elements
    pub fn element_count(&self) -> usize {
        let repr = unsafe { &*(self as *const Self as *const Repr) };
        repr.arr.element_count()
    }
    pub(crate) fn validate_shape(&self) {
        self.generic_ref_shallow(
            Array::validate_shape,
            Array::validate_shape,
            Array::validate_shape,
            Array::validate_shape,
            Array::validate_shape,
        )
    }
    /// Get the row at the given index
    pub fn row(&self, i: usize) -> Self {
        self.generic_ref_shallow(
            |arr| arr.row(i).into(),
            |arr| arr.row(i).into(),
            |arr| arr.row(i).into(),
            |arr| arr.row(i).into(),
            |arr| arr.row(i).into(),
        )
    }
    pub(crate) fn generic_into_deep<T>(
        self,
        n: impl FnOnce(Array<f64>) -> T,
        _b: impl FnOnce(Array<u8>) -> T,
        _co: impl FnOnce(Array<Complex>) -> T,
        ch: impl FnOnce(Array<char>) -> T,
        f: impl FnOnce(Array<Boxed>) -> T,
    ) -> T {
        match self {
            Self::Num(array) => n(array),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => _b(array),
            Self::Complex(array) => _co(array),
            Self::Char(array) => ch(array),
            Self::Box(array) => match array.into_unboxed() {
                Ok(value) => value.generic_into_deep(n, _b, _co, ch, f),
                Err(array) => f(array),
            },
        }
    }
    pub(crate) fn generic_ref_shallow<'a, T: 'a>(
        &'a self,
        n: impl FnOnce(&'a Array<f64>) -> T,
        _b: impl FnOnce(&'a Array<u8>) -> T,
        _co: impl FnOnce(&'a Array<Complex>) -> T,
        ch: impl FnOnce(&'a Array<char>) -> T,
        f: impl FnOnce(&'a Array<Boxed>) -> T,
    ) -> T {
        match self {
            Self::Num(array) => n(array),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => _b(array),
            Self::Complex(array) => _co(array),
            Self::Char(array) => ch(array),
            Self::Box(array) => f(array),
        }
    }
    pub(crate) fn generic_ref_deep<'a, T: 'a>(
        &'a self,
        n: impl FnOnce(&'a Array<f64>) -> T,
        _b: impl FnOnce(&'a Array<u8>) -> T,
        _co: impl FnOnce(&'a Array<Complex>) -> T,
        ch: impl FnOnce(&'a Array<char>) -> T,
        f: impl FnOnce(&'a Array<Boxed>) -> T,
    ) -> T {
        match self {
            Self::Num(array) => n(array),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => _b(array),
            Self::Complex(array) => _co(array),
            Self::Char(array) => ch(array),
            Self::Box(array) => {
                if let Some(bx) = array.as_scalar() {
                    bx.as_value().generic_ref_deep(n, _b, _co, ch, f)
                } else {
                    f(array)
                }
            }
        }
    }
    pub(crate) fn generic_ref_env_deep<'a, T: 'a>(
        &'a self,
        n: impl FnOnce(&'a Array<f64>, &Uiua) -> UiuaResult<T>,
        b: impl FnOnce(&'a Array<u8>, &Uiua) -> UiuaResult<T>,
        co: impl FnOnce(&'a Array<Complex>, &Uiua) -> UiuaResult<T>,
        ch: impl FnOnce(&'a Array<char>, &Uiua) -> UiuaResult<T>,
        f: impl FnOnce(&'a Array<Boxed>, &Uiua) -> UiuaResult<T>,
        env: &Uiua,
    ) -> UiuaResult<T> {
        self.generic_ref_deep(
            |a| n(a, env),
            |a| b(a, env),
            |a| co(a, env),
            |a| ch(a, env),
            |a| f(a, env),
        )
    }
    pub(crate) fn generic_mut_deep<T>(
        &mut self,
        n: impl FnOnce(&mut Array<f64>) -> T,
        _b: impl FnOnce(&mut Array<u8>) -> T,
        _co: impl FnOnce(&mut Array<Complex>) -> T,
        ch: impl FnOnce(&mut Array<char>) -> T,
        f: impl FnOnce(&mut Array<Boxed>) -> T,
    ) -> T {
        match self {
            Self::Num(array) => n(array),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => _b(array),
            Self::Complex(array) => _co(array),
            Self::Char(array) => ch(array),
            Self::Box(array) => {
                if let Some(bx) = array.as_scalar_mut() {
                    bx.as_value_mut().generic_mut_deep(n, _b, _co, ch, f)
                } else {
                    f(array)
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generic_bin_into<T, E>(
        self,
        other: Self,
        n: impl FnOnce(Array<f64>, Array<f64>) -> Result<T, E>,
        _b: impl FnOnce(Array<u8>, Array<u8>) -> Result<T, E>,
        _co: impl FnOnce(Array<Complex>, Array<Complex>) -> Result<T, E>,
        ch: impl FnOnce(Array<char>, Array<char>) -> Result<T, E>,
        f: impl FnOnce(Array<Boxed>, Array<Boxed>) -> Result<T, E>,
        err: impl FnOnce(Self, Self) -> E,
    ) -> Result<T, E> {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => n(a, b),
            #[cfg(feature = "bytes")]
            (Self::Byte(a), Self::Byte(b)) => _b(a, b),
            #[cfg(feature = "bytes")]
            (Self::Byte(a), Self::Num(b)) => n(a.convert(), b),
            #[cfg(feature = "bytes")]
            (Self::Num(a), Self::Byte(b)) => n(a, b.convert()),
            (Self::Complex(a), Self::Complex(b)) => _co(a, b),
            (Self::Complex(a), Self::Num(b)) => _co(a, b.convert()),
            (Self::Num(a), Self::Complex(b)) => _co(a.convert(), b),
            #[cfg(feature = "bytes")]
            (Self::Complex(a), Self::Byte(b)) => _co(a, b.convert()),
            #[cfg(feature = "bytes")]
            (Self::Byte(a), Self::Complex(b)) => _co(a.convert(), b),
            (Self::Char(a), Self::Char(b)) => ch(a, b),
            (Self::Box(a), Self::Box(b)) => f(a, b),
            (Self::Box(a), b) => f(a, b.coerce_to_boxes()),
            (a, Self::Box(b)) => f(a.coerce_to_boxes(), b),
            (a, b) => Err(err(a, b)),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generic_bin_ref<T, E>(
        &self,
        other: &Self,
        n: impl FnOnce(&Array<f64>, &Array<f64>) -> Result<T, E>,
        _b: impl FnOnce(&Array<u8>, &Array<u8>) -> Result<T, E>,
        _co: impl FnOnce(&Array<Complex>, &Array<Complex>) -> Result<T, E>,
        ch: impl FnOnce(&Array<char>, &Array<char>) -> Result<T, E>,
        f: impl FnOnce(&Array<Boxed>, &Array<Boxed>) -> Result<T, E>,
        err: impl FnOnce(&Self, &Self) -> E,
    ) -> Result<T, E> {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => n(a, b),
            #[cfg(feature = "bytes")]
            (Self::Byte(a), Self::Byte(b)) => _b(a, b),
            #[cfg(feature = "bytes")]
            (Self::Byte(a), Self::Num(b)) => n(&a.convert_ref(), b),
            #[cfg(feature = "bytes")]
            (Self::Num(a), Self::Byte(b)) => n(a, &b.convert_ref()),
            (Self::Complex(a), Self::Complex(b)) => _co(a, b),
            (Self::Complex(a), Self::Num(b)) => _co(a, &b.convert_ref()),
            (Self::Num(a), Self::Complex(b)) => _co(&a.convert_ref(), b),
            #[cfg(feature = "bytes")]
            (Self::Complex(a), Self::Byte(b)) => _co(a, &b.convert_ref()),
            #[cfg(feature = "bytes")]
            (Self::Byte(a), Self::Complex(b)) => _co(&a.convert_ref(), b),
            (Self::Char(a), Self::Char(b)) => ch(a, b),
            (Self::Box(a), Self::Box(b)) => f(a, b),
            (Self::Box(a), b) => f(a, &b.coerce_as_boxes()),
            (a, Self::Box(b)) => f(&a.coerce_as_boxes(), b),
            (a, b) => Err(err(a, b)),
        }
    }
    /// Ensure that the capacity is at least `min`
    pub(crate) fn reserve_min(&mut self, min: usize) {
        match self {
            Self::Num(arr) => arr.data.reserve_min(min),
            #[cfg(feature = "bytes")]
            Self::Byte(arr) => arr.data.reserve_min(min),
            Self::Complex(arr) => arr.data.reserve_min(min),
            Self::Char(arr) => arr.data.reserve_min(min),
            Self::Box(arr) => arr.data.reserve_min(min),
        }
    }
    /// Get the pretty-printed string representation of the value
    pub fn show(&self) -> String {
        match self {
            Self::Num(array) => array.grid_string(),
            #[cfg(feature = "bytes")]
            Self::Byte(array) => array.grid_string(),
            Self::Complex(array) => array.grid_string(),
            Self::Char(array) => array.grid_string(),
            Self::Box(array) => array.grid_string(),
        }
    }
    /// Attempt to convert the array to a list of integers
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_ints(&self, env: &Uiua, requirement: &'static str) -> UiuaResult<Vec<isize>> {
        self.as_number_list(env, requirement, |f| f.fract() == 0.0, |f| f as isize)
    }
    /// Attempt to convert the array to a single boolean
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_bool(&self, env: &Uiua, mut requirement: &'static str) -> UiuaResult<bool> {
        if requirement.is_empty() {
            requirement = "Expected value to be boolean"
        }
        Ok(match self {
            Value::Num(nums) => {
                if nums.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", nums.rank()))
                    );
                }
                let num = nums.data[0];
                if num == 0.0 {
                    false
                } else if num == 1.0 {
                    true
                } else {
                    return Err(env.error(format!("{requirement}, but it is {num}")));
                }
            }
            #[cfg(feature = "bytes")]
            Value::Byte(bytes) => {
                if bytes.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", bytes.rank()))
                    );
                }
                bytes.data[0] != 0
            }
            value => {
                return Err(env.error(format!("{requirement}, but it is {}", value.type_name())))
            }
        })
    }
    /// Attempt to convert the array to a single natural number
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_nat(&self, env: &Uiua, mut requirement: &'static str) -> UiuaResult<usize> {
        if requirement.is_empty() {
            requirement = "Expected value to be a natural number";
        }
        Ok(match self {
            Value::Num(nums) => {
                if nums.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", nums.rank()))
                    );
                }
                let num = nums.data[0];
                if num < 0.0 {
                    return Err(env.error(format!("{requirement}, but it is negative")));
                }
                if num.fract() != 0.0 {
                    return Err(env.error(format!("{requirement}, but it has a fractional part")));
                }
                num as usize
            }
            #[cfg(feature = "bytes")]
            Value::Byte(bytes) => {
                if bytes.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", bytes.rank()))
                    );
                }
                bytes.data[0] as usize
            }
            value => {
                return Err(env.error(format!("{requirement}, but it is {}", value.type_name())))
            }
        })
    }
    /// Attempt to convert the array to a single integer
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_int(&self, env: &Uiua, mut requirement: &'static str) -> UiuaResult<isize> {
        if requirement.is_empty() {
            requirement = "Expected value to be an integer";
        }
        Ok(match self {
            Value::Num(nums) => {
                if nums.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", nums.rank()))
                    );
                }
                let num = nums.data[0];
                if num.fract() != 0.0 {
                    return Err(env.error(format!("{requirement}, but it has a fractional part")));
                }
                num as isize
            }
            #[cfg(feature = "bytes")]
            Value::Byte(bytes) => {
                if bytes.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", bytes.rank()))
                    );
                }
                bytes.data[0] as isize
            }
            value => {
                return Err(env.error(format!("{requirement}, but it is {}", value.type_name())))
            }
        })
    }
    /// Attempt to convert the array to a single number
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_num(&self, env: &Uiua, mut requirement: &'static str) -> UiuaResult<f64> {
        if requirement.is_empty() {
            requirement = "Expected value to be a number";
        }
        Ok(match self {
            Value::Num(nums) => {
                if nums.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", nums.rank()))
                    );
                }
                nums.data[0]
            }
            #[cfg(feature = "bytes")]
            Value::Byte(bytes) => {
                if bytes.rank() > 0 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", bytes.rank()))
                    );
                }
                bytes.data[0] as f64
            }
            value => {
                return Err(env.error(format!("{requirement}, but it is {}", value.type_name())))
            }
        })
    }
    /// Attempt to convert the array to a list of numbers
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_nums(&self, env: &Uiua, requirement: &'static str) -> UiuaResult<Vec<f64>> {
        self.as_number_list(env, requirement, |_| true, |f| f)
    }
    /// Attempt to convert the array to a list of natural numbers
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_nats(&self, env: &Uiua, requirement: &'static str) -> UiuaResult<Vec<usize>> {
        self.as_number_list(
            env,
            requirement,
            |f| f.fract() == 0.0 && f >= 0.0,
            |f| f as usize,
        )
    }
    /// Attempt to convert the array to a list of bytes
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_bytes(&self, env: &Uiua, requirement: &'static str) -> UiuaResult<Vec<u8>> {
        self.as_number_list(
            env,
            requirement,
            |f| f.fract() == 0.0 && (0.0..256.0).contains(&f),
            |f| f as u8,
        )
    }
    /// Attempt to convert the array to a list of integers or infinity
    ///
    /// `None` represents infinity.
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_rank_list(
        &self,
        env: &Uiua,
        mut requirement: &'static str,
    ) -> UiuaResult<Vec<Option<isize>>> {
        if requirement.is_empty() {
            requirement = "Elements of rank list must be integers or infinity";
        }
        self.as_number_list(
            env,
            requirement,
            |n| n.fract() == 0.0 || n == f64::INFINITY,
            |n| {
                if n == f64::INFINITY {
                    None
                } else {
                    Some(n as isize)
                }
            },
        )
    }
    pub(crate) fn as_number_list<T>(
        &self,
        env: &Uiua,
        requirement: &'static str,
        test: fn(f64) -> bool,
        convert: fn(f64) -> T,
    ) -> UiuaResult<Vec<T>> {
        Ok(match self {
            Value::Num(nums) => {
                if nums.rank() > 1 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", nums.rank()))
                    );
                }
                let mut result = Vec::with_capacity(nums.row_count());
                for &num in nums.data() {
                    if !test(num) {
                        return Err(env.error(requirement));
                    }
                    result.push(convert(num));
                }
                result
            }
            #[cfg(feature = "bytes")]
            Value::Byte(bytes) => {
                if bytes.rank() > 1 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", bytes.rank()))
                    );
                }
                let mut result = Vec::with_capacity(bytes.row_count());
                for &byte in bytes.data() {
                    let num = byte as f64;
                    if !test(num) {
                        return Err(env.error(requirement));
                    }
                    result.push(convert(num));
                }
                result
            }
            value => {
                return Err(env.error(format!(
                    "{requirement}, but it is {}",
                    value.type_name_plural()
                )))
            }
        })
    }
    pub(crate) fn as_integer_array(
        &self,
        env: &Uiua,
        requirement: &'static str,
    ) -> UiuaResult<Array<isize>> {
        self.as_number_array(
            env,
            requirement,
            |_| true,
            |n| n.fract() == 0.0,
            |n| n as isize,
        )
    }
    pub(crate) fn as_natural_array(
        &self,
        env: &Uiua,
        requirement: &'static str,
    ) -> UiuaResult<Array<usize>> {
        self.as_number_array(
            env,
            requirement,
            |_| true,
            |n| n.fract() == 0.0 && n >= 0.0,
            |n| n as usize,
        )
    }
    pub(crate) fn as_number_array<T: Clone>(
        &self,
        env: &Uiua,
        requirement: &'static str,
        test_shape: fn(&[usize]) -> bool,
        test_num: fn(f64) -> bool,
        convert_num: fn(f64) -> T,
    ) -> UiuaResult<Array<T>> {
        Ok(match self {
            Value::Num(nums) => {
                if !test_shape(self.shape()) {
                    return Err(env.error(format!(
                        "{requirement}, but its shape is {}",
                        nums.format_shape()
                    )));
                }
                let mut result = EcoVec::with_capacity(nums.element_count());
                for &num in nums.data() {
                    if !test_num(num) {
                        return Err(env.error(requirement));
                    }
                    result.push(convert_num(num));
                }
                Array::new(self.shape(), result)
            }
            #[cfg(feature = "bytes")]
            Value::Byte(bytes) => {
                if !test_shape(self.shape()) {
                    return Err(env.error(format!(
                        "{requirement}, but its shape is {}",
                        bytes.format_shape()
                    )));
                }
                let mut result = EcoVec::with_capacity(bytes.element_count());
                for &byte in bytes.data() {
                    let num = byte as f64;
                    if !test_num(num) {
                        return Err(env.error(requirement));
                    }
                    result.push(convert_num(num));
                }
                Array::new(self.shape(), result)
            }
            value => {
                return Err(env.error(format!(
                    "{requirement}, but its type is {}",
                    value.type_name()
                )))
            }
        })
    }
    /// Attempt to convert the array to a string
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn as_string(&self, env: &Uiua, mut requirement: &'static str) -> UiuaResult<String> {
        if requirement.is_empty() {
            requirement = "Expected value to be a string";
        }
        match self {
            Value::Char(chars) => {
                if chars.rank() > 1 {
                    return Err(
                        env.error(format!("{requirement}, but its rank is {}", chars.rank()))
                    );
                }
                return Ok(chars.data().iter().collect());
            }
            Value::Box(boxes) => {
                if let Some(bx) = boxes.as_scalar() {
                    return bx.as_value().as_string(env, requirement);
                }
            }
            _ => {}
        }
        Err(env.error(format!(
            "{requirement}, but its type is {}",
            self.type_name()
        )))
    }
    /// Attempt to convert the array to a list of bytes
    ///
    /// The `requirement` parameter is used in error messages.
    pub fn into_bytes(self, env: &Uiua, mut requirement: &'static str) -> UiuaResult<Vec<u8>> {
        if requirement.is_empty() {
            requirement = "Expected value to be a list of bytes";
        }
        Ok(match self {
            #[cfg(feature = "bytes")]
            Value::Byte(a) => {
                if a.rank() != 1 {
                    return Err(env.error(format!("{requirement}, but its rank is {}", a.rank())));
                }
                a.data.into()
            }
            Value::Num(a) => {
                if a.rank() != 1 {
                    return Err(env.error(format!("{requirement}, but its rank is {}", a.rank())));
                }
                a.data.into_iter().map(|f| f as u8).collect()
            }
            Value::Char(a) => {
                if a.rank() != 1 {
                    return Err(env.error(format!("{requirement}, but its rank is {}", a.rank())));
                }
                a.data.into_iter().collect::<String>().into_bytes()
            }
            value => {
                return Err(env.error(format!(
                    "{requirement}, but its type is {}",
                    value.type_name()
                )))
            }
        })
    }
    /// Remove all top-level layers of boxing
    pub fn unpack(&mut self) {
        if let Value::Box(arr) = self {
            *self = match take(arr).into_unboxed() {
                Ok(value) => value.unpacked(),
                Err(arr) => Value::Box(arr),
            };
        }
    }
    /// Remove all top-level layers of boxing
    pub fn unpacked(self) -> Self {
        match self {
            Self::Box(arr) => match arr.into_unboxed() {
                Ok(value) => value.unpacked(),
                Err(arr) => Self::Box(arr),
            },
            value => value,
        }
    }
    /// Apply a function to the highest-level unboxed value
    pub fn map_boxed(self, f: impl FnOnce(Self) -> Self) -> Self {
        match self {
            Value::Box(boxed) => match boxed.into_scalar() {
                Ok(scalar) => Boxed(scalar.0.map_boxed(f)).into(),
                Err(boxed) => f(Value::Box(boxed)),
            },
            val => f(val),
        }
    }
    /// Apply a function to the highest-level unboxed value
    pub fn try_map_boxed(self, f: impl FnOnce(Self) -> UiuaResult<Self>) -> UiuaResult<Self> {
        match self {
            Value::Box(boxed) => match boxed.into_scalar() {
                Ok(scalar) => scalar.0.try_map_boxed(f).map(Boxed).map(Value::from),
                Err(boxed) => f(Value::Box(boxed)),
            },
            val => f(val),
        }
    }
    /// Remove a single layer of boxing
    pub fn unboxed(self) -> Self {
        match self {
            Value::Box(boxed) => match boxed.into_scalar() {
                Ok(scalar) => scalar.0,
                Err(boxed) => Value::Box(boxed),
            },
            val => val,
        }
    }
    /// Turn the value into a scalar box if it is not one already
    pub fn box_if_not(&mut self) {
        match &mut *self {
            Value::Box(arr) if arr.rank() == 0 => {}
            val => *self = Value::Box(Array::from(Boxed(take(val)))),
        }
    }
    /// Turn the value into a scalar box if it is not one already
    pub fn boxed_if_not(self) -> Boxed {
        match self {
            Value::Box(arr) if arr.rank() == 0 => arr.data.into_iter().next().unwrap(),
            val => Boxed(val),
        }
    }
    /// Turn a number array into a byte array if no information is lost.
    pub fn compress(&mut self) {
        #[cfg(feature = "bytes")]
        if let Value::Num(nums) = self {
            if nums
                .data
                .iter()
                .all(|n| n.fract() == 0.0 && *n <= u8::MAX as f64 && *n >= 0.0)
            {
                let mut bytes = EcoVec::with_capacity(nums.element_count());
                for n in take(&mut nums.data) {
                    bytes.push(n as u8);
                }
                *self = (take(&mut nums.shape), bytes).into();
            }
        }
    }
    /// Convert to a box array by boxing every element
    pub fn coerce_to_boxes(self) -> Array<Boxed> {
        match self {
            Value::Num(arr) => arr.convert_with(|v| Boxed(Value::from(v))),
            #[cfg(feature = "bytes")]
            Value::Byte(arr) => arr.convert_with(|v| Boxed(Value::from(v))),

            Value::Complex(arr) => arr.convert_with(|v| Boxed(Value::from(v))),
            Value::Char(arr) => arr.convert_with(|v| Boxed(Value::from(v))),
            Value::Box(arr) => arr,
        }
    }
    /// Convert to a box array by boxing every element
    pub fn coerce_as_boxes(&self) -> Cow<Array<Boxed>> {
        match self {
            Value::Num(arr) => Cow::Owned(arr.convert_ref_with(|v| Boxed(Value::from(v)))),
            #[cfg(feature = "bytes")]
            Value::Byte(arr) => Cow::Owned(arr.convert_ref_with(|v| Boxed(Value::from(v)))),

            Value::Complex(arr) => Cow::Owned(arr.convert_ref_with(|v| Boxed(Value::from(v)))),
            Value::Char(arr) => Cow::Owned(arr.convert_ref_with(|v| Boxed(Value::from(v)))),
            Value::Box(arr) => Cow::Borrowed(arr),
        }
    }
}

macro_rules! value_from {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for Value {
            fn from(item: $ty) -> Self {
                Self::$variant(Array::from(item))
            }
        }
        impl From<Array<$ty>> for Value {
            fn from(array: Array<$ty>) -> Self {
                Self::$variant(array)
            }
        }
        impl From<EcoVec<$ty>> for Value {
            fn from(vec: EcoVec<$ty>) -> Self {
                Self::$variant(Array::from(vec))
            }
        }
        impl From<CowSlice<$ty>> for Value {
            fn from(vec: CowSlice<$ty>) -> Self {
                Self::$variant(Array::from(vec))
            }
        }
        impl From<(Shape, EcoVec<$ty>)> for Value {
            fn from((shape, data): (Shape, EcoVec<$ty>)) -> Self {
                Self::$variant(Array::new(shape, data))
            }
        }
        impl From<(Shape, CowSlice<$ty>)> for Value {
            fn from((shape, data): (Shape, CowSlice<$ty>)) -> Self {
                Self::$variant(Array::new(shape, data))
            }
        }
        impl FromIterator<$ty> for Value {
            fn from_iter<I: IntoIterator<Item = $ty>>(iter: I) -> Self {
                Self::$variant(Array::from_iter(iter))
            }
        }
    };
}

value_from!(f64, Num);
#[cfg(feature = "bytes")]
value_from!(u8, Byte);
value_from!(char, Char);
value_from!(Boxed, Box);
value_from!(Complex, Complex);

#[cfg(not(feature = "bytes"))]
impl From<u8> for Value {
    fn from(u: u8) -> Self {
        Value::from(u as f64)
    }
}
#[cfg(not(feature = "bytes"))]
impl From<Array<u8>> for Value {
    fn from(array: Array<u8>) -> Self {
        array.convert_with(|b| b as f64).into()
    }
}
#[cfg(not(feature = "bytes"))]
impl FromIterator<u8> for Value {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        iter.into_iter().map(|b| b as f64).collect()
    }
}

impl FromIterator<usize> for Value {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        iter.into_iter().map(|i| i as f64).collect()
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::from(b as u8)
    }
}

impl From<usize> for Value {
    fn from(i: usize) -> Self {
        Value::from(i as f64)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        s.chars().collect()
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        s.chars().collect()
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::from(i as f64)
    }
}

macro_rules! value_un_impl {
    ($name:ident, $(
        $([$($feature1:literal,)* $in_place:ident, $f:ident])?
        $(($($feature2:literal,)* $make_new:ident, $f2:ident))?
    ),* $(,)?) => {
        impl Value {
            pub(crate) fn $name(self, env: &Uiua) -> UiuaResult<Self> {
                Ok(match self {
                    $($($(#[cfg(feature = $feature1)])* Self::$in_place(mut array) => {
                        for val in &mut array.data {
                            *val = $name::$f(*val);
                        }
                        array.into()
                    },)*)*
                    $($($(#[cfg(feature = $feature2)])* Self::$make_new(array) => {
                        let mut new = EcoVec::with_capacity(array.flat_len());
                        for val in array.data {
                            new.push($name::$f2(val));
                        }
                        (array.shape, new).into()
                    },)*)*
                    Value::Box(mut array) => {
                        let mut new_data = EcoVec::with_capacity(array.flat_len());
                        for b in array.data {
                            new_data.push(Boxed(b.0.$name(env)?));
                        }
                        array.data = new_data.into();
                        array.into()
                    }
                    #[allow(unreachable_patterns)]
                    val => return Err($name::error(val.type_name(), env))
                })
            }
        }
    }
}

value_un_impl!(neg, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(not, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(abs, [Num, num], ("bytes", Byte, byte), (Complex, com));
value_un_impl!(sign, [Num, num], ["bytes", Byte, byte], [Complex, com]);
value_un_impl!(sqrt, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(sin, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(cos, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(asin, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(acos, [Num, num], ("bytes", Byte, byte), [Complex, com]);
value_un_impl!(floor, [Num, num], ["bytes", Byte, byte], [Complex, com]);
value_un_impl!(ceil, [Num, num], ["bytes", Byte, byte], [Complex, com]);
value_un_impl!(round, [Num, num], ["bytes", Byte, byte], [Complex, com]);
value_un_impl!(
    complex_re,
    [Num, generic],
    ["bytes", Byte, generic],
    (Complex, com),
    [Char, generic]
);
value_un_impl!(
    complex_im,
    [Num, num],
    ["bytes", Byte, byte],
    (Complex, com)
);

macro_rules! val_retry {
    (Byte, $env:expr) => {
        $env.num_fill().is_ok()
    };
    ($variant:ident, $env:expr) => {
        false
    };
}

macro_rules! value_bin_impl {
    ($name:ident, $(
        $(($($feature1:literal,)* $na:ident, $nb:ident, $f:ident $(, $retry:ident)?))*
        $([$($feature2:literal,)* $ip:ident, $f2:ident $(, $retry2:ident)?])*
    ),* ) => {
        impl Value {
            #[allow(unreachable_patterns, clippy::wrong_self_convention)]
            pub(crate) fn $name(self, other: Self, a_depth: usize, b_depth: usize, env: &Uiua) -> UiuaResult<Self> {
                Ok(match (self, other) {
                    $($($(#[cfg(feature = $feature2)])* (Value::$ip(mut a), Value::$ip(b)) => {
                        if val_retry!($ip, env) {
                            let mut a_clone = a.clone();
                            if let Err(e) = bin_pervade_mut(&mut a_clone, b.clone(), a_depth, b_depth, env, $name::$f2) {
                                if e.is_fill() {
                                    $(
                                        let mut a = a.convert();
                                        let b = b.convert();
                                        bin_pervade_mut(&mut a, b, env, $name::$retry2)?;
                                        return Ok(a.into());
                                    )*
                                }
                                return Err(e);
                            } else {
                                a_clone.into()
                            }
                        } else {
                            bin_pervade_mut(&mut a, b, a_depth, b_depth, env, $name::$f2)?;
                            a.into()
                        }
                    },)*)*
                    $($($(#[cfg(feature = $feature1)])* (Value::$na(a), Value::$nb(b)) => {
                        if val_retry!($na, env) || val_retry!($nb, env) {
                            let res = bin_pervade(a.clone(), b.clone(), a_depth, b_depth, env, InfalliblePervasiveFn::new($name::$f));
                            match res {
                                Ok(arr) => arr.into(),
                                #[allow(unreachable_code, unused_variables)]
                                Err(e) if e.is_fill() => {
                                    $(return bin_pervade(a.convert(), b.convert(), a_depth, b_depth, env, InfalliblePervasiveFn::new($name::$retry)).map(Into::into);)?
                                    return Err(e);
                                }
                                Err(e) => return Err(e),
                            }
                        } else {
                            bin_pervade(a, b, a_depth, b_depth, env, InfalliblePervasiveFn::new($name::$f))?.into()
                        }
                    },)*)*
                    (Value::Box(a), Value::Box(b)) => {
                        let (a, b) = match (a.into_unboxed(), b.into_unboxed()) {
                            (Ok(a), Ok(b)) => return Ok(Boxed(Value::$name(a, b, a_depth, b_depth, env)?).into()),
                            (Ok(a), Err(b)) => (a.coerce_as_boxes().into_owned(), b),
                            (Err(a), Ok(b)) => (a, b.coerce_as_boxes().into_owned()),
                            (Err(a), Err(b)) => (a, b),
                        };
                        bin_pervade(a, b, a_depth, b_depth, env, FalliblePerasiveFn::new(|a: Boxed, b: Boxed, env: &Uiua| {
                            Ok(Boxed(Value::$name(a.0, b.0, a_depth, b_depth, env)?))
                        }))?.into()
                    }
                    (Value::Box(a), b) => {
                        match a.into_unboxed() {
                            Ok(a) => Boxed(Value::$name(a, b, a_depth, b_depth, env)?).into(),
                            Err(a) => {
                                let b = b.coerce_as_boxes().into_owned();
                                bin_pervade(a, b, a_depth, b_depth, env, FalliblePerasiveFn::new(|a: Boxed, b: Boxed, env: &Uiua| {
                                    Ok(Boxed(Value::$name(a.0, b.0, a_depth, b_depth, env)?))
                                }))?.into()
                            }
                        }
                    },
                    (a, Value::Box(b)) => {
                        match b.into_unboxed() {
                            Ok(b) => Boxed(Value::$name(a, b, a_depth, b_depth, env)?).into(),
                            Err(b) => {
                                let a = a.coerce_as_boxes().into_owned();
                                bin_pervade(a, b, a_depth, b_depth, env, FalliblePerasiveFn::new(|a: Boxed, b: Boxed, env: &Uiua| {
                                    Ok(Boxed(Value::$name(a.0, b.0, a_depth, b_depth, env)?))
                                }))?.into()
                            }
                        }
                    },
                    (a, b) => return Err($name::error(a.type_name(), b.type_name(), env)),
                })
            }
        }
    };
}

macro_rules! value_bin_math_impl {
    ($name:ident $(,$($tt:tt)*)?) => {
        value_bin_impl!(
            $name,
            [Num, num_num],
            ("bytes", Byte, Byte, byte_byte, num_num),
            ("bytes", Byte, Num, byte_num, num_num),
            ("bytes", Num, Byte, num_byte, num_num),
            [Complex, com_x],
            (Complex, Num, com_x),
            (Num, Complex, x_com),
            ("byte", Complex, Byte, com_x, com_x),
            ("byte", Byte, Complex, x_com, com_x),
            $($($tt)*)?
        );
    };
}

value_bin_math_impl!(
    add,
    (Num, Char, num_char),
    (Char, Num, char_num),
    ("bytes", Byte, Char, byte_char),
    ("bytes", Char, Byte, char_byte),
);
value_bin_math_impl!(
    sub,
    (Num, Char, num_char),
    (Char, Char, char_char),
    ("bytes", Byte, Char, byte_char),
);
value_bin_math_impl!(mul);
value_bin_math_impl!(div);
value_bin_math_impl!(modulus);
value_bin_math_impl!(pow);
value_bin_math_impl!(log);
value_bin_math_impl!(atan2);
value_bin_math_impl!(min, [Char, char_char]);
value_bin_math_impl!(max, [Char, char_char]);

value_bin_impl!(
    complex,
    (Num, Num, num_num),
    ("bytes", Byte, Byte, byte_byte, num_num),
    ("bytes", Byte, Num, byte_num, num_num),
    ("bytes", Num, Byte, num_byte, num_num),
    [Complex, com_x],
    (Complex, Num, com_x),
    (Num, Complex, x_com),
    ("byte", Complex, Byte, com_x, com_x),
    ("byte", Byte, Complex, x_com, com_x),
);

macro_rules! eq_impls {
    ($($name:ident),*) => {
        $(
            value_bin_impl!(
                $name,
                // Value comparable
                [Num, same_type],
                [Complex, same_type],
                ("bytes", Byte, Byte, same_type, num_num),
                (Char, Char, generic),
                ("bytes", Num, Byte, num_byte, num_num),
                ("bytes", Byte, Num, byte_num, num_num),
                (Complex, Num, com_x),
                (Num, Complex, x_com),
                ("byte", Complex, Byte, com_x),
                ("byte", Byte, Complex, x_com),
                // Type comparable
                (Num, Char, always_less),
                ("bytes", Byte, Char, always_less),
                (Char, Num, always_greater),
                ("bytes", Char, Byte, always_greater),
            );
        )*
    };
}

macro_rules! cmp_impls {
    ($($name:ident),*) => {
        $(
            value_bin_impl!(
                $name,
                // Value comparable
                [Num, same_type],
                [Complex, com_x],
                ("bytes", Byte, Byte, same_type, num_num),
                (Char, Char, generic),
                ("bytes", Num, Byte, num_byte, num_num),
                ("bytes", Byte, Num, byte_num, num_num),
                (Complex, Num, com_x),
                (Num, Complex, x_com),
                ("byte", Complex, Byte, com_x),
                ("byte", Byte, Complex, x_com),
                // Type comparable
                (Num, Char, always_less),
                ("bytes", Byte, Char, always_less),
                (Char, Num, always_greater),
                ("bytes", Char, Byte, always_greater),
            );
        )*
    };
}

eq_impls!(is_eq, is_ne);
cmp_impls!(is_lt, is_le, is_gt, is_ge);

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            #[cfg(feature = "bytes")]
            (Value::Byte(a), Value::Byte(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Complex(a), Value::Complex(b)) => a == b,
            (Value::Box(a), Value::Box(b)) => a == b,
            #[cfg(feature = "bytes")]
            (Value::Num(a), Value::Byte(b)) => a == b,
            #[cfg(feature = "bytes")]
            (Value::Byte(a), Value::Num(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.type_id().cmp(&other.type_id());
        if type_order != Ordering::Equal {
            return type_order;
        }
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a.cmp(b),
            #[cfg(feature = "bytes")]
            (Value::Byte(a), Value::Byte(b)) => a.cmp(b),
            (Value::Complex(a), Value::Complex(b)) => a.cmp(b),
            (Value::Char(a), Value::Char(b)) => a.cmp(b),
            (Value::Box(a), Value::Box(b)) => a.cmp(b),
            #[cfg(feature = "bytes")]
            (Value::Num(a), Value::Byte(b)) => a.partial_cmp(b).unwrap(),
            #[cfg(feature = "bytes")]
            (Value::Byte(a), Value::Num(b)) => a.partial_cmp(b).unwrap(),
            (Value::Num(_), _) => Ordering::Less,
            (_, Value::Num(_)) => Ordering::Greater,
            #[cfg(feature = "bytes")]
            (Value::Byte(_), _) => Ordering::Less,
            #[cfg(feature = "bytes")]
            (_, Value::Byte(_)) => Ordering::Greater,
            (Value::Complex(_), _) => Ordering::Less,
            (_, Value::Complex(_)) => Ordering::Greater,
            (Value::Char(_), _) => Ordering::Less,
            (_, Value::Char(_)) => Ordering::Greater,
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id().hash(state);
        match self {
            Value::Num(arr) => arr.hash(state),
            #[cfg(feature = "bytes")]
            Value::Byte(arr) => arr.hash(state),
            Value::Complex(arr) => arr.hash(state),
            Value::Char(arr) => arr.hash(state),
            Value::Box(arr) => arr.hash(state),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => n.grid_string().fmt(f),
            #[cfg(feature = "bytes")]
            Value::Byte(b) => b.grid_string().fmt(f),
            Value::Complex(c) => c.grid_string().fmt(f),
            Value::Box(v) => v.grid_string().fmt(f),
            Value::Char(c) if c.rank() < 2 => c.fmt(f),
            Value::Char(c) => c.grid_string().fmt(f),
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct ValueBuilder {
    value: Option<Value>,
    rows: usize,
    capacity: usize,
}

impl ValueBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            value: None,
            rows: 0,
            capacity,
        }
    }
    pub fn add_row<C: FillContext>(&mut self, mut row: Value, ctx: &C) -> Result<(), C::Error> {
        if let Some(value) = &mut self.value {
            value.append(row, ctx)?;
        } else {
            row.reserve_min(self.capacity);
            row.shape_mut().insert(0, 1);
            self.value = Some(row);
        }
        self.rows += 1;
        Ok(())
    }
    pub fn finish(self) -> Value {
        self.value.unwrap_or_default()
    }
}
