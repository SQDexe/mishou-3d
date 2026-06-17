use {
    color_art::Error as ColorError,
    thiserror::Error,
    core::{
        num::{
            ParseFloatError,
            ParseIntError
            },
        str::FromStr
        }
    };



/** Error representing a failure to parse an array of elements. */
#[derive(Debug, Error)]
pub enum ParseArrayError<T>
where T: FromStr {
    /** A failure occurred whilst parsing one of the array elements. */
    #[error("Failed to parse array element: {0}")]
    ParseElement(#[source] T::Err),
    /** The input was improperly separated, the number of elements is incorrect, or the text is generally invalid. */
    #[error("Provided input does not match the expected array format")]
    InvalidArrayFormat
    }

// /** Error indicating that a parsed numerical value evaluated to a non-finite state. */
// #[derive(Debug, Error)]
// #[error("The parsed floating-point number is not finite")]
// pub struct NotFiniteError;

// /** Enumeration of errors that can occur whilst parsing a finite floating-point number. */
// #[derive(Debug, Error)]
// pub enum ParseFiniteFloatError {
//     /** A failure occurred during the initial string parsing into a floating-point value. */
//     #[error("Failed to parse the floating-point number: {0}")]
//     Parse(#[from] ParseFloatError),
//     /** The parsed number was mathematically valid but evaluated to a non-finite value (infinity or NaN). */
//     #[error(transparent)]
//     NotFinite(#[from] NotFiniteError),
//     /** The parsed floating-point number exceeds the allowed permissible bounds. */
//     #[error("The parsed floating-point number does not fit within the set range")]
//     OutOfRange
//     }

/** Enumeration of errors that can occur whilst parsing colour values. */
#[derive(Debug, Error)]
pub enum ParseColorError {
    /** Error originating from the underlying colour processing library. */
    #[error("Failed to process colour value: {0}")]
    Color(#[from] ColorError),
    /** Error encountered whilst parsing a floating-point colour channel value. */
    #[error("Failed to parse floating-point colour channel: {0}")]
    ParseFloat(#[from] ParseFloatError),
    /** Error encountered whilst parsing an integer colour channel value. */
    #[error("Failed to parse integer colour channel: {0}")]
    ParseInt(#[from] ParseIntError),
    /** The provided input string does not match any recognised colour format. */
    #[error("The provided colour input is invalid")]
    InvalidInput,
    /** The selected input parsing method is unknown or unsupported. */
    #[error("An unknown colour input method was chosen")]
    UnknownInputMethod
    }

impl From<ParseArrayError<f64>> for ParseColorError {
    fn from(value: ParseArrayError<f64>) -> Self {
        match value {
            ParseArrayError::InvalidArrayFormat =>
                Self::InvalidInput,
            ParseArrayError::ParseElement(error) =>
                Self::ParseFloat(error)
            }
        }
    }

// /** Enumeration of errors that can occur whilst parsing mathematical vectors. */
// #[derive(Debug, Error)]
// pub enum ParseVectorError {
//     /** Error encountered whilst parsing the sequence of array elements. */
//     #[error("Failed to parse vector array format: {0}")]
//     ParseArray(#[from] ParseArrayError<f32>),
//     /** One or more parsed vector components evaluated to a non-finite value. */
//     #[error(transparent)]
//     NotFinite(#[from] NotFiniteError)
//     }

// /** Enumeration of errors that can occur whilst parsing rotational vectors. */
// #[derive(Debug, Error)]
// pub enum ParseRotationVectorError {
//     /** Error encountered whilst parsing the sequence of array elements. */
//     #[error("Failed to parse rotation vector array format: {0}")]
//     ParseArray(#[from] ParseArrayError<f32>),
//     /** One or more parsed vector components evaluated to a non-finite value. */
//     #[error(transparent)]
//     NotFinite(#[from] NotFiniteError),
//     /** The parsed pitch angle falls outside the permissible bounds to prevent gimbal lock. */
//     #[error("The parsed pitch angle falls outside the permissible range")]
//     InvalidPitchRange
//     }