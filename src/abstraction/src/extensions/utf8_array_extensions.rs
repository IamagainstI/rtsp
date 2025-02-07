use encoding_rs::Encoding;
use num::{CheckedAdd, CheckedDiv, CheckedMul, Num, NumCast};
use num::traits::CheckedNeg;

use super::cast_error::CastError;


type Result<'a, T> = std::result::Result<T, CastError>;

const STEP: i32 = 10;

/// A trait that provides additional methods for byte slices (`[u8]`).
pub trait U8ArrayExt {
    /// Checks if the byte slice is valid UTF-8.
    ///
    /// # Returns
    ///
    /// `true` if the byte slice is valid UTF-8, `false` otherwise.
    fn is_utf8(&self) -> bool;

    /// Converts the byte slice to a number.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the number to convert to. Must implement `Num`, `CheckedAdd`, `CheckedMul`, `CheckedNeg`, `CheckedDiv`, and `NumCast`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the converted number if successful, or a `CastError` if the conversion fails.
    fn utf8_to_number<T>(&self) -> Result<T>
    where
        T: Num + CheckedAdd + CheckedMul + CheckedNeg + CheckedDiv + NumCast;

    /// Converts the byte slice to a string slice (`&str`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the string slice if successful, or a `CastError` if the conversion fails.
    fn utf8_to_str(&self) -> Result<&str>;
}

impl U8ArrayExt for [u8] {
    fn is_utf8(&self) -> bool {
        let idx = Encoding::utf8_valid_up_to(self);
        let len = self.len();
        idx == len
    }
    
    fn utf8_to_number<T>(&self) -> Result<T>
        where T: Num + CheckedAdd + CheckedMul + CheckedNeg + CheckedDiv + NumCast 
    {
        std::io::Error::new(std::io::ErrorKind::AddrInUse, "error");
        if !self.is_utf8() {
            return Err(CastError::InvalidData("Invalid UTF-8 source".to_string()));
        }

        let step = T::from(STEP).ok_or_else(|| CastError::InvalidData("Conversion failed".to_string()))?;
        let mut number: T = T::zero();
        let mut is_negative = false;

        for (i, &byte) in self.iter().enumerate() {
            match byte {
                b'0'..=b'9' => {
                    let digit = T::from(byte - b'0').ok_or_else(|| CastError::InvalidData("Conversion failed".to_string()))?;
                    number = number
                            .checked_mul(&step)
                            .and_then(|n| n.checked_add(&digit))
                            .ok_or_else(|| CastError::InvalidData("Overflow occurred".to_string()))?;
                },
                b'-' if i == 0 => {
                    is_negative = true;
                },
                _ => return Err(CastError::InvalidData("Invalid byte in input".to_string())),
            }
        }

        if is_negative {
            number = number.checked_neg().ok_or_else(|| CastError::InvalidData("Overflow occurred".to_string()))?;
        }

        Ok(number)
    }
    
    fn utf8_to_str(&self) -> Result<&str> {
        std::str::from_utf8(self).map_err(|e| CastError::Utf8Error(e))
    }
}