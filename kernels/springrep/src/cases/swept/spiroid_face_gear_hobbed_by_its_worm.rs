//! A face gear's 30 teeth cut from a coned ring by a tapered worm thread turning in mesh.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The tooth (or thread) is carried by two turns at once, about the blank's axis and the mating one, so the space it cuts is bounded by envelopes of its flanks under that motion, which springrep cannot make from its profile sweeps".into())
}
