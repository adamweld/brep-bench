//! The solid a square bar (side 2, from z = 0 to 2) fills as it is carried round the circle of
//! radius 5 while turning by −2t about its own center.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The square turns about its own center while carried round another, so its section sweeps a region bounded by envelopes of its sides and the trochoids its corners trace, which springrep cannot make".into())
}
