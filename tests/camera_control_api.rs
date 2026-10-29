use depthai::CameraControl;

// Creating a CameraControl message needs no device, unlike Camera::set_initial_control,
// so this only covers the FFI round trip of the message and its setters.
#[test]
fn camera_control_setters_succeed_without_hardware() {
    let mut control = CameraControl::new().expect("CameraControl allocation should succeed");
    control
        .set_auto_exposure_compensation(-1)
        .expect("auto exposure compensation should be writable");
    control
        .set_sharpness(1)
        .expect("sharpness should be writable");
}
