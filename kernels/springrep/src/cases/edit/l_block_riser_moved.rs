//! An L-block, the section (0, 0), (40, 0), (40, 10), (15, 10), (15, 30), (0, 30) in (x, z) from
//! y = 0 to y = 20; its riser x = 15 then moved out 10 to x = 25, the faces beside it following.
use springrep::edit::move_faces_by;
use springrep::math::{Transform, Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::edit::faces;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the section drawn in the plane y = 20 with x along x and y along z, extruded back to y = 0
    let plane = frame([0., 20., 0.], [0., -1., 0.], [1., 0., 0.])?;
    let section: Vec<Vec2> = [(0., 0.), (40., 0.), (40., 10.), (15., 10.), (15., 30.), (0., 30.)]
        .iter()
        .map(|&(x, z)| Vec2::new(x, z))
        .collect();
    let sides = ["floor", "end", "tread", "riser", "top", "back"];
    let l = extrude(&Profile::polygon(&section, &sides), &plane, 20., "l").map_err(e)?;
    let riser = faces(&l, &["l.riser"])?;
    move_faces_by(&l, &riser, &Transform::translation(Vec3::new(10., 0., 0.))).map_err(e)
}
