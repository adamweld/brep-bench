//! The box (−8, −8, −8)–(8, 8, 0) less everything a bull-nose mill whose axis leans 30° passes
//! through as it slides straight down by 8.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Leaning, the mill's round touches what the push sweeps along a curve where its normal is level, which lies in no plane: the hole's wall is that curve carried down, an upright surface over the round's outline seen from above, an offset of an ellipse that springrep cannot make".into())
}
