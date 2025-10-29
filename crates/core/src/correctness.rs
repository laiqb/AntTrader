use std::fmt::{Debug, Display};

use rust_decimal::Decimal;

use crate::collections::{MapLike, SetLike};

pub const FAILED: &str = "Condition faild";

// 正确性的验证方法

#[inline(always)]
pub fn check_predicate_true(predicate: bool, fail_msg: &str) -> anyhow::Result<()>{
    if !predicate{
        anyhow::bail!("{fail_msg}")
    }
    Ok(())
}

#[inline(always)]
pub fn check_predicate_false(predicate: bool, fail_msg: &str) -> anyhow::Result<()>{
    if predicate{
        anyhow::bail!("{fail_msg}")
    }

    Ok(())
}

#[inline(always)]
pub fn check_nonempty_string<T: AsRef<str>>(s: T, param: &str) -> anyhow::Result<()> {
    if s.as_ref().is_empty() {
        anyhow::bail!("invalid string for '{param}', was empty");
    }
    Ok(())
}

#[inline(always)]
pub fn check_valid_string<T: AsRef<str>>(s: T, param: &str) -> anyhow::Result<()> {
    let s = s.as_ref();
    
    if s.is_empty(){
        anyhow::bail!("invalid string for '{param}', was empty");
    }

    let mut has_non_whitespace = false;
    for c in s.chars(){
        if !c.is_whitespace(){
            has_non_whitespace = true;
        }

        if !c.is_ascii(){
            anyhow::bail!("invalid string for '{param}' contained a non-ASCII char, was '{s}'");
        }
    }

    if !has_non_whitespace{
        anyhow::bail!("invalid string for '{param}', was all whitespace");
    }

    Ok(())
}

#[inline(always)]
pub fn check_valid_string_optional<T: AsRef<str>>(s:Option<T>, param: &str) -> anyhow::Result<()>{
    if let Some(s) = s{
        check_valid_string(s, param)?;
    }
    Ok(())
}

#[inline(always)]
pub fn check_string_contains<T: AsRef<str>>(s:T, pat:&str, param: &str) -> anyhow::Result<()>{
    let s = s.as_ref();
    if !s.contains(pat){
        anyhow::bail!("invalid string for '{param}' did not contain '{pat}', was '{s}'")
    }
    Ok(())
}

#[inline(always)]
pub fn check_equal<T: PartialEq + Debug + Display>(
    lhs: &T, 
    rhs: &T, 
    lhs_param: &str,
    rhs_param: &str,
) -> anyhow::Result<()>{
    if lhs != rhs {
        anyhow::bail!("'{lhs_param}' value of {lhs} was not equal to '{rhs_param}' value of {rhs}");
    }
    Ok(())
}   


/// Checks the `u8` values are equal.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_equal_u8(lhs: u8, rhs: u8, lhs_param: &str, rhs_param: &str) -> anyhow::Result<()> {
    if lhs != rhs {
        anyhow::bail!("'{lhs_param}' u8 of {lhs} was not equal to '{rhs_param}' u8 of {rhs}")
    }
    Ok(())
}


/// Checks the `usize` values are equal.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_equal_usize(
    lhs: usize,
    rhs: usize,
    lhs_param: &str,
    rhs_param: &str,
) -> anyhow::Result<()> {
    if lhs != rhs {
        anyhow::bail!("'{lhs_param}' usize of {lhs} was not equal to '{rhs_param}' usize of {rhs}")
    }
    Ok(())
}


/// Checks the `u64` value is positive (> 0).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_positive_u64(value: u64, param: &str) -> anyhow::Result<()> {
    if value == 0 {
        anyhow::bail!("invalid u64 for '{param}' not positive, was {value}")
    }
    Ok(())
}


/// Checks the `u128` value is positive (> 0).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_positive_u128(value: u128, param: &str) -> anyhow::Result<()> {
    if value == 0 {
        anyhow::bail!("invalid u128 for '{param}' not positive, was {value}")
    }
    Ok(())
}


/// Checks the `i64` value is positive (> 0).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_positive_i64(value: i64, param: &str) -> anyhow::Result<()> {
    if value <= 0 {
        anyhow::bail!("invalid i64 for '{param}' not positive, was {value}")
    }
    Ok(())
}


/// Checks the `i64` value is positive (> 0).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_positive_i128(value: i128, param: &str) -> anyhow::Result<()> {
    if value <= 0 {
        anyhow::bail!("invalid i128 for '{param}' not positive, was {value}")
    }
    Ok(())
}

/// Checks the `f64` value is non-negative (>= 0).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_non_negative_f64(value: f64, param: &str) -> anyhow::Result<()> {
    if value.is_nan() || value.is_infinite() {
        anyhow::bail!("invalid f64 for '{param}', was {value}")
    }
    if value < 0.0 {
        anyhow::bail!("invalid f64 for '{param}' negative, was {value}")
    }
    Ok(())
}

/// Checks the `u8` value is in range [`l`, `r`] (inclusive).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_in_range_inclusive_u8(value: u8, l: u8, r: u8, param: &str) -> anyhow::Result<()> {
    if value < l || value > r {
        anyhow::bail!("invalid u8 for '{param}' not in range [{l}, {r}], was {value}")
    }
    Ok(())
}

/// Checks the `u64` value is range [`l`, `r`] (inclusive).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_in_range_inclusive_u64(value: u64, l: u64, r: u64, param: &str) -> anyhow::Result<()> {
    if value < l || value > r {
        anyhow::bail!("invalid u64 for '{param}' not in range [{l}, {r}], was {value}")
    }
    Ok(())
}

/// Checks the `i64` value is in range [`l`, `r`] (inclusive).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_in_range_inclusive_i64(value: i64, l: i64, r: i64, param: &str) -> anyhow::Result<()> {
    if value < l || value > r {
        anyhow::bail!("invalid i64 for '{param}' not in range [{l}, {r}], was {value}")
    }
    Ok(())
}

/// Checks the `f64` value is in range [`l`, `r`] (inclusive).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_in_range_inclusive_f64(value: f64, l: f64, r: f64, param: &str) -> anyhow::Result<()> {
    // SAFETY: Hardcoded epsilon is intentional and appropriate here because:
    // - 1e-15 is conservative for IEEE 754 double precision (machine epsilon ~2.22e-16)
    // - This function is used for validation, not high-precision calculations
    // - The epsilon prevents spurious failures due to floating-point representation
    // - Making it configurable would complicate the API for minimal benefit
    const EPSILON: f64 = 1e-15;

    if value.is_nan() || value.is_infinite() {
        anyhow::bail!("invalid f64 for '{param}', was {value}")
    }
    if value < l - EPSILON || value > r + EPSILON {
        anyhow::bail!("invalid f64 for '{param}' not in range [{l}, {r}], was {value}")
    }
    Ok(())
}

/// Checks the `usize` value is in range [`l`, `r`] (inclusive).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_in_range_inclusive_usize(
    value: usize,
    l: usize,
    r: usize,
    param: &str,
) -> anyhow::Result<()> {
    if value < l || value > r {
        anyhow::bail!("invalid usize for '{param}' not in range [{l}, {r}], was {value}")
    }
    Ok(())
}

/// Checks the slice is empty.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_slice_empty<T>(slice: &[T], param: &str) -> anyhow::Result<()> {
    if !slice.is_empty() {
        anyhow::bail!(
            "the '{param}' slice `&[{}]` was not empty",
            std::any::type_name::<T>()
        )
    }
    Ok(())
}

/// Checks the slice is **not** empty.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_slice_not_empty<T>(slice: &[T], param: &str) -> anyhow::Result<()> {
    if slice.is_empty() {
        anyhow::bail!(
            "the '{param}' slice `&[{}]` was empty",
            std::any::type_name::<T>()
        )
    }
    Ok(())
}

/// Checks the hashmap is empty.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_map_empty<M>(map: &M, param: &str) -> anyhow::Result<()>
where
    M: MapLike,
{
    if !map.is_empty() {
        anyhow::bail!(
            "the '{param}' map `&<{}, {}>` was not empty",
            std::any::type_name::<M::Key>(),
            std::any::type_name::<M::Value>(),
        );
    }
    Ok(())
}

/// Checks the map is **not** empty.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_map_not_empty<M>(map: &M, param: &str) -> anyhow::Result<()>
where
    M: MapLike,
{
    if map.is_empty() {
        anyhow::bail!(
            "the '{param}' map `&<{}, {}>` was empty",
            std::any::type_name::<M::Key>(),
            std::any::type_name::<M::Value>(),
        );
    }
    Ok(())
}

/// Checks the `key` is **not** in the `map`.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_key_not_in_map<M>(
    key: &M::Key,
    map: &M,
    key_name: &str,
    map_name: &str,
) -> anyhow::Result<()>
where
    M: MapLike,
{
    if map.contains_key(key) {
        anyhow::bail!(
            "the '{key_name}' key {key} was already in the '{map_name}' map `&<{}, {}>`",
            std::any::type_name::<M::Key>(),
            std::any::type_name::<M::Value>(),
        );
    }
    Ok(())
}

/// Checks the `key` is in the `map`.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_key_in_map<M>(
    key: &M::Key,
    map: &M,
    key_name: &str,
    map_name: &str,
) -> anyhow::Result<()>
where
    M: MapLike,
{
    if !map.contains_key(key) {
        anyhow::bail!(
            "the '{key_name}' key {key} was not in the '{map_name}' map `&<{}, {}>`",
            std::any::type_name::<M::Key>(),
            std::any::type_name::<M::Value>(),
        );
    }
    Ok(())
}

/// Checks the `member` is **not** in the `set`.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_member_not_in_set<S>(
    member: &S::Item,
    set: &S,
    member_name: &str,
    set_name: &str,
) -> anyhow::Result<()>
where
    S: SetLike,
{
    if set.contains(member) {
        anyhow::bail!(
            "the '{member_name}' member was already in the '{set_name}' set `&<{}>`",
            std::any::type_name::<S::Item>(),
        );
    }
    Ok(())
}

/// Checks the `member` is in the `set`.
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_member_in_set<S>(
    member: &S::Item,
    set: &S,
    member_name: &str,
    set_name: &str,
) -> anyhow::Result<()>
where
    S: SetLike,
{
    if !set.contains(member) {
        anyhow::bail!(
            "the '{member_name}' member was not in the '{set_name}' set `&<{}>`",
            std::any::type_name::<S::Item>(),
        );
    }
    Ok(())
}

/// Checks the `Decimal` value is positive (> 0).
///
/// # Errors
///
/// Returns an error if the validation check fails.
#[inline(always)]
pub fn check_positive_decimal(value: Decimal, param: &str) -> anyhow::Result<()> {
    if value <= Decimal::ZERO {
        anyhow::bail!("invalid Decimal for '{param}' not positive, was {value}")
    }
    Ok(())
}