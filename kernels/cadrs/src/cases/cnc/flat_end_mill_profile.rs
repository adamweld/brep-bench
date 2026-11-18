//! A groove profiled round a cross by a flat end mill in three step-downs. The plate is the box
//! from (0, 0, −15) to (120, 90, 0); a flat end mill of radius 3, upright, plunges at (60, 10)
//! and runs anticlockwise round the outline of a cross (a bar 80 by 30 across a bar 30 by 70,
//! every corner rounded to 6) at z = −3, −6 and −9, then rises.
//!
//! The program goes to cadrs whole, as one path the mill is carried along upright (its cut by
//! a moving solid): the kernel reads each lap's groove as the ground within the mill's radius
//! of the outline, flat, and cuts the deepest; the shallower laps lie within it.
use crate::bench::{block, cylinder, p3, v3};
use crate::shared::cnc::Program;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const CROSS: [(f64, f64); 12] = [
    (45.0, 10.0), (75.0, 10.0), (75.0, 30.0), (100.0, 30.0), (100.0, 60.0), (75.0, 60.0),
    (75.0, 80.0), (45.0, 80.0), (45.0, 60.0), (20.0, 60.0), (20.0, 30.0), (45.0, 30.0),
];

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (min, max) = (p3(0.0, 0.0, -15.0), p3(120.0, 90.0, 0.0));
    let plate = block(k, min, max)?;
    // The mill: radius 3, 30 long, upright with its tip at the origin; the lap starts at
    // (60, 10), the middle of the cross's first side, and the whole program goes to the
    // kernel as one path.
    let mill = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 3.0, 30.0)?;
    let mut p = Program::new(plate, (min, max), mill, 3.0, p3(60.0, 10.0, 5.0)).whole();
    for depth in [-3.0, -6.0, -9.0] {
        p.line(k, (60.0, 10.0, depth))?;
        p.rounded_loop(k, &CROSS, 6.0)?;
        p.line(k, (60.0, 10.0, 5.0))?;
    }
    p.run(k)?;
    Ok(p.body)
}
