use crate::solids::*;

pub fn build() -> R<Body> {
    let bar = solidcraft_kernel::cylinder(v3(-20.0, 0.0, 0.0), Vec3::X, 6.0, 40.0)?;
    let stem = zcyl(0.0, 0.0, 3.0, 0.0, 15.0)?;
    let tee = union(&bar, &stem)?;
    // the junction, picked at its highest point over x = 3 and followed all the way round
    // through whatever edges the union split it into
    let junction = solidcraft_kernel::tangent_chain(&tee, &[v3(3.0, 0.0, 6.0)])?;
    solidcraft_kernel::fillet(&tee, &junction, 1.0)
}
