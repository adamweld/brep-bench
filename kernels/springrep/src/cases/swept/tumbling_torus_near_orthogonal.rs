//! The solid the torus about (2.02, 0, 1) (radii 1 and 0.08) fills as it tumbles about two
//! nearly square axes, for t from −180° to 180°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Under two turns about skew axes no line of the torus stays in contact, so the swept solid's boundary is an envelope of the torus's poses of no simpler kind, which springrep cannot make from its profile sweeps".into())
}
