//! The crossed-forward tumbling torus's swept solid less the crossed-reverse one's.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Under two turns about skew axes no line of the torus stays in contact, so the swept solid's boundary is an envelope of the torus's poses of no simpler kind, which springrep cannot make from its profile sweeps; neither operand can be built".into())
}
