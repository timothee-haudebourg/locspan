use super::{Stripped, StrippedPartialEq};
use crate::Meta;
use std::collections::HashMap;
use std::hash::Hash;

/// Defines the total equality of values
/// without considering metadata.
pub trait StrippedEq: StrippedPartialEq {}

impl<T: StrippedEq> Eq for Stripped<T> {}

impl<'t, T: StrippedEq> StrippedEq for &'t T {}

impl<T: StrippedEq, M> StrippedEq for Meta<T, M> {}

impl<T: StrippedEq> StrippedEq for Box<T> {}

impl<T: StrippedEq> StrippedEq for Option<T> {}

impl<T: StrippedEq> StrippedEq for Vec<T> {}

impl<K: Eq + Hash, V: StrippedEq> StrippedEq for HashMap<K, V> {}
