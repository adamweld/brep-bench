use crate::solids::*;

pub fn build() -> R<Body> {
    let s3 = 3f64.sqrt();
    let centres = [
        v3(0.0, 0.0, 0.0),
        v3(10.0, 0.0, 0.0),
        v3(5.0, 5.0 * s3, 0.0),
        v3(5.0, 5.0 * s3 / 3.0, 10.0 * (2.0f64 / 3.0).sqrt()),
    ];
    let mut solid = solidcraft_kernel::sphere(centres[0], 10.0)?;
    for &c in &centres[1..] {
        solid = common(&solid, &solidcraft_kernel::sphere(c, 10.0)?)?;
    }
    Ok(solid)
}
