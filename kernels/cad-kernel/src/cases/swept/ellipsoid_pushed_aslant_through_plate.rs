//! A plate, the box (−15, −15, 0)–(15, 15, 4), with a hole punched through it by the ellipsoid of
//! semi-axes 3, 2 and 1.5 along x, y and z centred at (−2, −1.5, −4), wholly below it, slid without
//! turning along (4, 3, 12) to centre (2, 1.5, 8), wholly above it.
//!
//! cad-kernel's sweep of a solid (`swept::closed::cut_from`) refuses the ellipsoid under a slide
//! (`Unbuilt::Curved` at its pole): it takes a face that is a plane, a cylinder, a cone, a ball or
//! a torus, and an ellipsoid's is a ball's affine image. So the sweep is built through that image:
//! an affine map carries everything a solid passes through under a slide to everything its image
//! passes through under the image of the slide. The unit ball about the centre, slid along the push
//! read back through the stretch, is swept by the kernel's own sweep of a ball (`build::canal`: its
//! disc square to the slide carried along it, a half ball beyond each end), and the stretch that
//! makes the ball the ellipsoid (`Affine::stretch`, `build::ellipsoid`'s) carries that sweep onto
//! the ellipsoid's.
use crate::sketch::block;
use cad_kernel::affine::Affine;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::canal;
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{norm, unit};

const CENTRE: V = [-2., -1.5, -4.];
const SEMI: [f64; 3] = [3., 2., 1.5];
const PUSH: V = [4., 3., 12.];

pub fn build() -> Result<Brep, String> {
    let plate = block([-15., -15., 0.], [15., 15., 4.])?;
    // the stretch carrying the unit ball about the centre onto the ellipsoid, along the world's
    // axes
    let frame = Frame::new(CENTRE, [0., 0., 1.], [1., 0., 0.]);
    let stretch = Affine::stretch(&frame, SEMI)?;
    // the push read back through the stretch: what the unit ball slides along
    let back = stretch.inverse().vector(PUSH);
    let ball = canal(CENTRE, 1., CENTRE, unit(back), 0., norm(back), "hole")?;
    let swept = ball.mapped(&stretch);
    Ok(combined(&plate, &swept, Op::Cut, 0.)?)
}
