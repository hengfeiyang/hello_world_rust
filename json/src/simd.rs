use crate::errors::Error;

#[inline(always)] 
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: ?Sized + serde::Serialize,
{
    Ok(simd_json::to_string(value)?)
}

#[inline(always)] 
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + serde::Serialize,
{
    Ok(simd_json::to_vec(value)?)
}

#[inline(always)] 
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    Ok(from_slice(s.as_bytes())?)
}

#[inline(always)] 
pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    Ok(serde_json::from_slice(v)?)
}
