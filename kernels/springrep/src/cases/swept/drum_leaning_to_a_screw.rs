//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a drum
//! leaning 25° toward +y about (10, 0, 0) passes through as it is screwed about the z axis,
//! rising 1 a turn, from −140° to 140°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The drum leans to the screw, so its side and rims touch what they sweep along curves off its section through the axis: the groove's walls are envelopes springrep cannot make from its profile sweeps".into())
}
