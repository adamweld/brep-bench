//! The box (−8, −8, −8)–(8, 8, 0) less everything a torus (radii 3 and 1, its axis leaning 65°)
//! passes through as it slides straight down by 8.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Leaning, the torus touches what the push sweeps along curves where its normal is level, in no plane, and past 54.7° the inner one cusps: the walls are those curves carried down, upright surfaces over offsets of ellipses that springrep cannot make".into())
}
