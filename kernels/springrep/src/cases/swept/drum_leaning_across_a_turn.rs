//! The ring between radii 3 and 8 about the z axis, heights −2.5 to 2.5, less everything a drum
//! (radius 1, length 3, leaning 25° in the plane y = 1.2 about (5, 1.2, 0)) passes through as it
//! turns 100° about the z axis.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The leaning drum's side touches what it sweeps along two curves, and its rims, sharp circles leaning to the turn's axis, sweep rings whose sections are no circles: surfaces of revolution of curves springrep would have to compute".into())
}
