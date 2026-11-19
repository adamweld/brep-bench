//! A gear blank (radius 22 about the y axis) with one tooth space cut by a rack tooth rolled
//! around it.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. Rolled, the rack's flanks cut involutes of the base circle and its tip corners trace the root: envelopes and trochoids springrep cannot make from its profile sweeps (it has no involute curve)".into())
}
