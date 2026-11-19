//! A dumbbell-shaped plate, two 10 x 10 squares joined by a neck 4 long and 2 wide, extruded 6 along
//! +z, shelled inward to a wall 1.5 thick, open at its top: the wall is every point within 1.5 of
//! the faces other than the top, so the neck is left solid and each pocket bulges toward the neck
//! in arcs of radius 1.5 about the neck's concave corners.
//!
//! Refused. springrep's shell (`edit::shell`) moves each face 1.5 inward and runs the moved faces on
//! until they meet, so at the plate's concave corners the cavity's walls meet sharp instead of
//! keeping 1.5 from the corner: called here it builds each pocket as a plain square (volume 807,
//! not 804.84). Nor can the cavity be made as the points 1.5 inside the walls, as `edit::offset`
//! refuses an inward offset.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep's shell runs the moved walls on until they meet, sharp at the plate's concave \
         corners, where the wall should keep 1.5 from the corner (arcs of radius 1.5); it has no \
         shell by distance, and edit::offset refuses an inward offset"
        .into())
}
