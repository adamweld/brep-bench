use crate::solids::*;

/// Where the holes cross the cube, on each axis.
const GRID: [f64; 3] = [7.5, 15.0, 22.5];

pub fn build() -> R<Body> {
    let mut cube = block((0.0, 0.0, 0.0), (30.0, 30.0, 30.0))?;
    // each drill runs a unit past the cube at both ends
    let mut drills = Vec::new();
    for g in GRID {
        drills.push(solidcraft_kernel::cylinder(v3(-1.0, g, 15.0), Vec3::X, 2.5, 32.0)?);
        drills.push(solidcraft_kernel::cylinder(v3(g, -1.0, 15.0), Vec3::Y, 2.5, 32.0)?);
        for h in GRID {
            drills.push(zcyl(g, h, 2.5, -1.0, 31.0)?);
        }
    }
    for drill in &drills {
        cube = cut(&cube, drill)?;
    }
    Ok(cube)
}
