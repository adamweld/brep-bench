//! A cup, a sheet of revolution about z: the floor, the disk of radius 4 at z = 0; the bend, a
//! quarter of the torus of tube radius 2 about the circle of radius 4 at z = 2, out to radius 6;
//! and the wall, the cylinder of radius 6 up to z = 10. Thickened by 3 to the inside, past the
//! bend's radius.
use crate::sketch::faces_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

/// The floor's radius, where the bend starts.
const FLOOR: f64 = 4.;
/// The bend's radius.
const BEND: f64 = 2.;
/// The wall's top.
const HEIGHT: f64 = 10.;
/// How far the sheet is thickened, to the inside.
const THICKNESS: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    // cad-kernel thickens a sheet as each face swept along its normal, and refuses the bend's
    // torus swept past its tube's centre line ("no point stands that far off it"), as a bend of
    // radius 2 is by 3. So the cup is thickened while its floor and wall meet in a sharp rim, and
    // bent after: the rim rounded on the outside, as a modeller orders the two features when the
    // thickening fails. Inside, the moved floor and wall meet in the same edge either way, and the
    // round takes only material within 2 of the rim, inside the thickened sheet.
    let wall = FLOOR + BEND;
    let cup = prism(&disc([0., 0., 0.], [0., 0., 1.], wall), 0., HEIGHT)?;
    // its floor and wall lifted out as the sheet, each picked by a point on it; their normals face
    // out of the cup, so its inside is the negative side
    let faces = faces_through(&cup, &[[0., 0., 0.], [0., wall, HEIGHT / 2.]])?;
    let body = thickened(&sheet_of(&cup, &faces)?, [-THICKNESS, 0.], Join::Round)?;
    // the rim picked by a point on it; the ring outside the round cut off
    let round = roll(&body, [0., wall, 0.], BEND, tolerance(body.size()))?;
    Ok(combined(&body, &round.piece, Op::Cut, 0.)?)
}
