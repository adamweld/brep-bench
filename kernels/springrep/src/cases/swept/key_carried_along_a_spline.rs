//! The box (−6, −6, −8)–(36, 26, 0) less everything a square key passes through as a frame
//! riding a cubic Bézier curve carries it, turning with the curve's tangent.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The key turns with the curve and is no solid of revolution, so its contact moves from face to face and edge to edge and its corners dig past its sides where the curve bends: the groove's walls are envelopes springrep's path sweep of a fixed section cannot make".into())
}
