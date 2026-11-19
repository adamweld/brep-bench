//! The box (−6, −6, −8)–(36, 26, 0) less everything an upright ring (a torus about the frame's y
//! axis) passes through as a frame riding a cubic Bézier curve carries it, turning with the
//! curve's tangent.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The ring turns with the curve and is no convex solid, so its contact moves over it, its inner side touching too as the curve bends: the groove's walls are envelopes springrep's path sweep of a fixed section cannot make".into())
}
