use autocxx::c_int;
use depthai_sys::depthai;

use crate::error::{Result, clear_error_flag, last_error, take_error_if_any};
use crate::host_node::Buffer;

/// Camera control message, mirrors `dai::CameraControl`.
///
/// Apply it before the pipeline starts with
/// [`CameraNode::set_initial_control`](crate::camera::CameraNode::set_initial_control), or send it
/// at runtime to the camera's `inputControl` input queue.
///
/// Only the settings needed so far are exposed. Further `dai::CameraControl` setters follow the
/// same pattern.
///
/// # Example
///
/// ```no_run
/// # use depthai::{Pipeline, Result};
/// # use depthai::camera::{CameraNode, CameraBoardSocket};
/// # use depthai::CameraControl;
/// # fn main() -> Result<()> {
/// # let pipeline = Pipeline::new().build()?;
/// # let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
/// let mut control = CameraControl::new()?;
/// control.set_auto_exposure_compensation(-1)?;
/// camera.set_initial_control(&control)?;
/// # Ok(())
/// # }
/// ```
///
/// Wraps a heap-allocated `std::shared_ptr<dai::CameraControl>` as a `Buffer` handle.
/// Released automatically when dropped.
pub struct CameraControl {
    buffer: Buffer,
}

impl CameraControl {
    /// A control message with no settings applied.
    pub fn new() -> Result<Self> {
        clear_error_flag();
        let handle = depthai::dai_camera_control_new();
        if handle.is_null() {
            Err(last_error("failed to create CameraControl"))
        } else {
            Ok(Self {
                buffer: Buffer::from_handle(handle),
            })
        }
    }

    /// Shift the auto exposure brightness target. Range -9..9, default 0, positive is brighter.
    pub fn set_auto_exposure_compensation(&mut self, compensation: i32) -> Result<()> {
        clear_error_flag();
        unsafe {
            depthai::dai_camera_control_set_auto_exposure_compensation(
                self.buffer.handle(),
                c_int(compensation),
            )
        };
        if let Some(err) = take_error_if_any("failed to set auto exposure compensation") {
            return Err(err);
        }
        Ok(())
    }

    /// ISP sharpening strength. Range 0..4, default 1.
    pub fn set_sharpness(&mut self, sharpness: i32) -> Result<()> {
        clear_error_flag();
        unsafe {
            depthai::dai_camera_control_set_sharpness(self.buffer.handle(), c_int(sharpness))
        };
        if let Some(err) = take_error_if_any("failed to set sharpness") {
            return Err(err);
        }
        Ok(())
    }

    /// Borrow the underlying `Buffer` handle so it can be sent via ```rust,ignore [`InputQueue::send_buffer`]```.
    pub fn as_buffer(&self) -> &Buffer {
        &self.buffer
    }
}

// Allow `ctrl_queue.send_buffer(&camera_ctrl)` directly without calling `.as_buffer()`.
impl std::ops::Deref for CameraControl {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
