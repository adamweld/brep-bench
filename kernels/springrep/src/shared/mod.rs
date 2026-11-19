//! Helpers shared by the case files: springrep's errors in words, and the frames the cases place
//! their profiles in. Each category's agent-written helpers live in a module of their own here.
#![allow(dead_code)]

use springrep::math::{Frame, Vec3};

/// A kernel error as the reason a case gives.
pub fn e(err: springrep::Error) -> String {
    err.to_string()
}

/// The frame at `origin` whose normal is `z` and whose first axis is `x`.
pub fn frame(origin: [f64; 3], z: [f64; 3], x: [f64; 3]) -> Result<Frame, String> {
    Frame::new(Vec3::from_array(origin), Vec3::from_array(z), Vec3::from_array(x)).map_err(e)
}

/// The world frame moved to `origin`.
pub fn at(origin: [f64; 3]) -> Frame {
    Frame::new(Vec3::from_array(origin), Vec3::Z, Vec3::X).expect("the world's axes")
}
