//! The solid the torus about (3, 0, 2) (radii 1 and 0.015) fills as it tumbles about two skew
//! axes, for t from −75° to 75°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Under two turns about skew axes no line of the torus stays in contact, so the swept solid's boundary is an envelope of the torus's poses of no simpler kind, which springrep cannot make from its profile sweeps".into())
}
