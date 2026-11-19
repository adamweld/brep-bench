//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a bar
//! drafted 2° both ways from a plane leaning 20° across the turn passes through as it turns
//! about the z axis from −140° to 140°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The bar leans across the turn and its sides are drafted, so they touch what they sweep along lines off any half-plane through the axis: the groove's walls are envelopes springrep cannot make from its profile sweeps".into())
}
