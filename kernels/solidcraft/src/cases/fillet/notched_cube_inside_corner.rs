use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (20.0, 20.0, 20.0))?;
    let notched = cut(&cube, &block((10.0, 10.0, 10.0), (20.0, 20.0, 20.0))?)?;
    // the midpoints of the three concave edges out of the inside corner (10, 10, 10)
    let edges = [v3(15.0, 10.0, 10.0), v3(10.0, 15.0, 10.0), v3(10.0, 10.0, 15.0)];
    solidcraft_kernel::fillet(&notched, &edges, 3.0)
}
