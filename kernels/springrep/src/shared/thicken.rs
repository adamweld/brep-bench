//! Helpers shared by the thicken cases. springrep has no sheet bodies: its thicken sweeps faces of a
//! solid along their outward normals (inward for a negative thickness), and the faces round them,
//! extended, bound the slab at the sheet's edges. So each case's sheet is a face, or faces, of a
//! solid whose faces at the sheet's edges lie along its normals there.
#![allow(dead_code)]

use springrep::edit::thicken;
use springrep::topo::Brep;

use crate::shared::e;
pub use crate::shared::shell::{arc, faces, line, turned_whole};

/// The faces of `b` named `names` (each `feature.face`) thickened by `thickness`: along their
/// outward normals when positive, into `b` when negative.
pub fn thickened(b: &Brep, names: &[&str], thickness: f64) -> Result<Brep, String> {
    let sheet = faces(b, names)?;
    thicken(b, &sheet, thickness).map_err(e)
}
pub use crate::shared::shell::edges_where;

/// The closed outline `segments` drawn in the xz plane (its x along world x, its y along world z)
/// and extruded along +y from y = 0 to `length`.
pub fn swept_along_y(
    segments: Vec<springrep::profile::NamedSegment>,
    length: f64,
    feature: &str,
) -> Result<Brep, String> {
    use springrep::math::{Frame, Point3, Vec3};
    // the sketch plane's normal is -y, so that its y runs up world z; extruding against the normal
    // runs along +y
    let plane = Frame::new(Point3::ZERO, -Vec3::Y, Vec3::X).map_err(e)?;
    let outline = springrep::profile::Profile::outline(segments);
    springrep::sweep::extrude(&outline, &plane, -length, feature).map_err(e)
}

/// The bent sheet of `thicken/bent-flange` as three faces of a solid, `flange.floor`,
/// `flange.bend` and `flange.wall`: the profile from (-10, 0) to the origin, round the quarter
/// circle of radius 2 about (0, 2) to (2, 2) and up to (2, 12), each (x, z), closed on the bend's
/// inside by the plane z = 12 over the wall's top and x = -10 at the floor's end (each along the
/// sheet's normal at that edge), swept 8 along y. The faces' outward normals point away from the
/// bend's axis.
pub fn flange() -> Result<Brep, String> {
    use std::f64::consts::FRAC_PI_2;
    swept_along_y(
        vec![
            line([-10., 0.], [0., 0.], "floor"),
            arc([0., 2.], 2., -FRAC_PI_2, FRAC_PI_2, "bend"),
            line([2., 2.], [2., 12.], "wall"),
            line([2., 12.], [-10., 12.], "over"),
            line([-10., 12.], [-10., 0.], "end"),
        ],
        8.,
        "flange",
    )
}

/// A cup of revolution about the z axis as three faces of a solid, `cup.floor`, `cup.bend` and
/// `cup.wall`: in the half-plane (distance from the axis, z), the floor from the axis out to
/// `inner`, the quarter circle of radius `bend` about (`inner`, `bend`) out and up to
/// (`inner + bend`, `bend`), and the wall up to `height`, closed by the plane z = `height` across
/// the cup's mouth. The faces' outward normals point out of the cup.
pub fn cup(inner: f64, bend: f64, height: f64) -> Result<Brep, String> {
    use std::f64::consts::FRAC_PI_2;
    let outer = inner + bend;
    turned_whole(
        vec![
            line([0., 0.], [inner, 0.], "floor"),
            arc([inner, bend], bend, -FRAC_PI_2, FRAC_PI_2, "bend"),
            line([outer, bend], [outer, height], "wall"),
            line([outer, height], [0., height], "mouth"),
            line([0., height], [0., 0.], "axis"),
        ],
        "cup",
    )
}
