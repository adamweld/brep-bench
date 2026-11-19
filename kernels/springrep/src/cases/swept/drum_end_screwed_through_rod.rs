//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a cylinder of radius
//! 1.2 along the x axis from x = 4 to 12 passes through as it is screwed about the z axis,
//! rising 6 a turn.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The drum lies across the screw's axis: its level sections are rectangles whose width changes with height, each turned by its own lag, so what it covers at a height is bounded by the envelope of their long sides, a curve springrep cannot make, nor the surface it sweeps".into())
}
