//! The box (−6, −6, −8)–(36, 26, 0) less everything a bull-nose mill leaning 30° passes through
//! as a frame riding a cubic Bézier curve carries it, turning with the curve's tangent.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The frame turns the leaning mill as the curve turns, so the curve along which the mill touches what it sweeps moves over the mill from instant to instant: the groove's walls are envelopes springrep's path sweep of a fixed section cannot make".into())
}
