//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a prism of closed
//! B-spline section in the plane y = 0 passes through as it is screwed about the z axis, rising
//! 6 a turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The bar's spline side touches what it sweeps along curves off its section through the axis, found only numerically on the spline at each ruling: the groove is an envelope springrep cannot make from its profile sweeps".into())
}
