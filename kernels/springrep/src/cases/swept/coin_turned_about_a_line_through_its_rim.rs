//! The solid a thick disc (radius 2 about the line through (2, 0, 0) along y, from y = 0 to 0.5)
//! fills as it turns 90° about the z axis.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. The disc's rim on its far face is a circle leaning to the turn's velocity: circles about the z axis near it meet the disc in arcs that no one half-plane through the axis crosses, and that rim sweeps a surface of revolution whose meridian is the rim's shadow on a half-plane through the axis, a curve springrep cannot make (it revolves only profiles drawn in a plane through the axis)".into())
}
