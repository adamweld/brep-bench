//! The solid a regular tetrahedron fills screwed about the z axis — right-handed, rising 5 a turn
//! — from a turn of 0° to 540°: the tetrahedron on (5, 1, 1), (5, −1, −1), (3, 1, −1), (3, −1, 1).
use crate::sketch::block;
use cad_kernel::build::kept;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{cross, dot, sub, unit};

const CORNERS: [V; 4] = [[5., 1., 1.], [5., -1., -1.], [3., 1., -1.], [3., -1., 1.]];
const LEAD: f64 = 5.;
const TURN: f64 = 540f64.to_radians();

pub fn build() -> Result<Brep, String> {
    // a block round it cut by the plane of each face, kept on the side of the fourth corner
    let mut tool = block([2., -2., -2.], [6., 2., 2.])?;
    for k in 0..4 {
        let [a, b, c] = [CORNERS[(k + 1) % 4], CORNERS[(k + 2) % 4], CORNERS[(k + 3) % 4]];
        let mut n = unit(cross(sub(b, a), sub(c, a)));
        if dot(n, sub(CORNERS[k], a)) < 0. { n = n.map(|x| -x); }
        tool = kept(&tool, a, n, 0.)?;
    }
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: [0., 0., 1.], ratio: 1., advance: LEAD,
    });
    // the kernel's closed swept solid: a polyhedral tool whose edges lean to the screw's axis,
    // each screwed into a patch of the ruled helicoid (its flat faces mesh exactly, at any bar)
    closed(&tool, &path, [0., TURN], tool.size(), tolerance(tool.size()))
        .map_err(|why| format!("the tetrahedron's sweep not built: {why:?}"))
}
