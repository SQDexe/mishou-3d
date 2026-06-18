use {
    color_art::Color,
    glam::{
        Vec2,
        Vec3
        },
    itertools::Itertools,
    core::{
        array::from_fn,
        str::FromStr,
        iter::zip,
        mem::MaybeUninit
        },
    crate::{
        args::checks::{
            // check_hex_colour,
            check_named_colour
            },
        // consts::ranges::PITCH_RANGE_LIMIT,
        error::{
            // NotFiniteError,
            ParseArrayError,
            ParseColorError,
            // ParseFiniteFloatError,
            // ParseRotationVectorError,
            // ParseVectorError
            }
        }
    };



/**
Parses a comma-separated string into an array of a specific type and size.

In the future, this could be rewritten using [`next_chunk`], and [`try_map`].

# Errors

Returns a `ParseArrayError` if the input format is invalid or if any individual element fails to parse.

[`next_chunk`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.next_chunk
[`try_map`]: https://doc.rust-lang.org/std/primitive.array.html#method.try_map
*/
pub fn parse_array<T, const N: usize>(value: &str) -> Result<[T; N], ParseArrayError<T>>
where T: FromStr {
    let split: [&str; N] = value.splitn(N, ',')
        .next_array()
        .ok_or(ParseArrayError::InvalidArrayFormat)?;

    let mut maybe_parsed = from_fn(|_| MaybeUninit::uninit());

    for (output, input) in zip(&mut maybe_parsed, split) {
        output.write(input.parse().map_err(ParseArrayError::ParseElement)?);
        }

    let parsed: MaybeUninit<_> = maybe_parsed.into();
    
    /*
    SAFETY: 
    This is safe as the whole array range is initialised earlier,
    as both are of the same, `const` size
    */
    unsafe {
        Ok(parsed.assume_init())
        }
    }

/**
Parses a comma-separated string into a 2-D vector.

# Errors

Returns a `ParseArrayError` if the input format is incorrect or the values cannot be parsed as floats.
*/
pub fn parse_vec2(value: &str) -> Result<Vec2, ParseArrayError<f32>> {
    let split = parse_array(value)?;

    Ok(Vec2::from_array(split))
    }

/**
Parses a comma-separated string into a 3-D floating-point vector.

# Errors

Returns a `ParseArrayError` if the input format is incorrect or the values cannot be parsed as floats.
*/
pub fn parse_vec3(value: &str) -> Result<Vec3, ParseArrayError<f32>> {
    let split = parse_array(value)?;

    Ok(Vec3::from_array(split))
    }

/**
Parses a string representation into a colour object.

Supports hexadecimal codes, named colours, and specific formatted methods

# Examples

```rust,ignore
let hex_colour = parse_colour("hex-ff0000").unwrap();
let named_colour = parse_colour("red").unwrap();
let rgb_colour = parse_colour("rgb-255,0,0").unwrap();

// Verify that different input formats map to the same underlying colour
assert_eq!(hex_colour.hex(), named_colour.hex());
assert_eq!(named_colour.hex(), rgb_colour.hex());

// Other supported formats
assert!(parse_colour("hsl-360,100,100").is_ok());
assert!(parse_colour("hsv-360,100,100").is_ok());
assert!(parse_colour("cmyk-100,100,100,100").is_ok());

```

# Errors

Returns a `ParseColorError` if the input does not match any recognised colour format or if channel values are invalid.
*/
pub fn parse_colour(value: &str) -> Result<Color, ParseColorError> {
    let lowercase = value.to_lowercase();
    if check_named_colour(&lowercase) {
        let colour = Color::from_name(&lowercase)?;
        return Ok(colour);
        }

    let method_params_pair = split_method_params(value)
        .ok_or(ParseColorError::InvalidInput)?;

    let colour = match method_params_pair {
        ("hex", params) if params.len() == 6 => {
            let num = u32::from_str_radix(params, 16)?;
            Color::from_num(num)?
            },
        ("rgb", params) => {
            let [r, g, b] = parse_array(params)?;
            Color::from_rgb(r, g, b)?
            },
        ("hsl", params) => {
            let [h, s, l] = parse_array(params)?;
            Color::from_hsl(h, s, l)?
            },
        ("hsv", params) => {
            let [h, s, v] = parse_array(params)?;
            Color::from_hsv(h, s, v)?
            },
        ("cmyk", params) => {
            let [c, m, y, k] = parse_array(params)?;
            Color::from_cmyk(c, m, y, k)?
            },
        _ => return Err(ParseColorError::UnknownInputMethod)
        };

    Ok(colour)
    }

/**
Splits a string by a hyphen into a parsing method identifier and its associated parameter string.

Returns `None` if the input does not contain a hyphen.
*/
fn split_method_params(value: &str) -> Option<(&str, &str)> {
    let mut iter = value.splitn(2, '-');

    iter.next().zip(iter.next())
    }

// /**
// Parses a comma-separated string into a finite 3-D vector representing a camera position.
//
// # Errors
// 
// Returns a `ParseVectorError` if the parsing fails or if any evaluated coordinate is non-finite.
// */
// pub fn parse_camera_position(value: &str) -> Result<Vec3, ParseVectorError> {
//     let split = parse_array(value)?;
//
//     if ! split.iter().all(|n| n.is_finite()) {
//         return Err(NotFiniteError.into());
//         }
//
//     Ok(Vec3::from_array(split))
//     }

// /**
// Parses a comma-separated string into a finite 2-D vector representing the light direction.
// 
// # Errors
// 
// Returns a `ParseVectorError` if the parsing fails or if any evaluated direction component is non-finite.
// */
// pub fn parse_light_direction(value: &str) -> Result<Vec2, ParseVectorError> {
//     let split = parse_array(value)?;
//
//     if ! split.iter().all(|n| n.is_finite()) {
//         return Err(NotFiniteError.into());
//         }
//
//     Ok(Vec2::from_array(split))
//     }

// /**
// Parses a comma-separated string into a 2-D vector representing camera rotation.
// 
// # Errors
// 
// Returns a `ParseRotationVectorError` if the parsing fails, a coordinate is non-finite, or if the pitch angle falls outside the permissible range.
// */
// pub fn parse_camera_rotation(value: &str) -> Result<Vec2, ParseRotationVectorError> {
//     let split: [f32; 2] = parse_array(value)?;
//
//     if ! split.iter().all(|n| n.is_finite()) {
//         return Err(NotFiniteError.into());
//         }
//
//     if let [_, ref pitch] = split && ! PITCH_RANGE_LIMIT.contains(pitch) {
//         return Err(ParseRotationVectorError::InvalidPitchRange);
//         }
//
//     Ok(Vec2::from_array(split))
//     }