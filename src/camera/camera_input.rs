use {
    bitfields::bitfield,
    crate::camera::{
        camera_movement::CameraMovement,
        matrix_unit::MatrixUnit
        }
    };


/**
Bitfield representation of currently active camera movement inputs.

This structure compactly stores boolean flags for each spatial movement direction within a single byte.
*/
#[bitfield(
    u8,
    new = false,
    builder = false,
    debug = true,
    default = true,
    from_into_bits = false,
    bit_ops = false
    )]
pub struct CameraInput {
    /** Flag indicating whether forward movement is currently requested. */
    #[bits(access = rw)]
    forward: bool,
    /** Flag indicating whether backward movement is currently requested. */
    #[bits(access = rw)]
    backward: bool,
    /** Flag indicating whether leftward movement is currently requested. */
    #[bits(access = rw)]
    left: bool,
    /** Flag indicating whether rightward movement is currently requested. */
    #[bits(access = rw)]
    right: bool,
    /** Flag indicating whether upward movement is currently requested. */
    #[bits(access = rw)]
    upward: bool,
    /** Flag indicating whether downward movement is currently requested. */
    #[bits(access = rw)]
    downward: bool,
    /** Unused padding bits to align the structure to a full byte. */
    #[bits(2)]
    _padding: u8,
    }

impl CameraInput {
    /** Calculates the aggregated directional vector based on the currently active movement flags. */
    pub fn get_input(&self) -> CameraMovement {
        let mut axis = CameraMovement::default();
        
        if self.forward()
            { axis.z += MatrixUnit::Positiv; }
        if self.backward()
            { axis.z -= MatrixUnit::Positiv; }
        if self.right()
            { axis.x += MatrixUnit::Positiv; }
        if self.left()
            { axis.x -= MatrixUnit::Positiv; }
        if self.upward()
            { axis.y += MatrixUnit::Positiv; }
        if self.downward()
            { axis.y -= MatrixUnit::Positiv; }
        
        axis
        }

    /** Clears all active input flags, resetting the movement state to neutral. */
    pub fn reset(&mut self) {
        *self = Self::default();
        }
    }