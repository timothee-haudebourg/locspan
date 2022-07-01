use super::{Stripped, StrippedPartialEq};
use crate::Loc;

/// Defines the total equality of located values
/// without considering locations.
pub trait StrippedEq: StrippedPartialEq {}

impl<'a, T: StrippedEq> Eq for Stripped<'a, T> {}

impl<T: StrippedEq, F, S> StrippedEq for Loc<T, F, S> {}

impl<T: StrippedEq> StrippedEq for Box<T> {}

impl<T: StrippedEq> StrippedEq for Option<T> {}

impl<T: StrippedEq> StrippedEq for Vec<T> {}
