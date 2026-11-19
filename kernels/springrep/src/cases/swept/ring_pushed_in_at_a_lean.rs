//! The box (−8, −8, −8)–(8, 8, 0) less everything a torus (radii 3 and 1, its axis leaning 35°)
//! passes through as it slides straight down by 8.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Leaning, the torus touches what the push sweeps along two curves where its normal is level, in no plane: the hole's and the core's walls are those curves carried down, upright surfaces over offsets of ellipses that springrep cannot make".into())
}
