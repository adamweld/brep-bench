//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a ball of
//! radius 0.8 at (10, 0, 0) on a stem leaning 20° toward +y passes through as it turns about the
//! z axis from −140° to 30°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The stem leans across the turn, so circles about the z axis meet it in arcs that no one half-plane through the axis crosses, and it touches what it sweeps along curves off its section by any such plane: the notch's walls there are envelopes springrep cannot make from its profile sweeps".into())
}
