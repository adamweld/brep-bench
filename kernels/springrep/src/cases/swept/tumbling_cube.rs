//! The solid the cube (2.4, −0.6, 1.4)–(3.6, 0.6, 2.6) fills as it turns by t/4 about the
//! upright line through (2, 0, 0) and by −t about the line through (0, 0.5, 0.5) along x, for t
//! from −75° to 75°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Under two turns about skew axes no line of the cube stays fixed and its silhouette changes from face to face: the swept solid's walls are envelopes of its faces and edges that springrep cannot make from its profile sweeps".into())
}
