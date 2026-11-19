//! The solid a regular tetrahedron with corners (5, 1, 1), (5, −1, −1), (3, 1, −1) and (3, −1,
//! 1) fills as it is screwed about the z axis, rising 5 a turn, from 0° to 540°.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep of a solid. None of the tetrahedron's faces is level or upright, so under the screw its edges sweep ruled helicoidal sheets that are no screw of a section through the axis, and its silhouette turns at its corners: springrep's profile sweeps cannot make them".into())
}
