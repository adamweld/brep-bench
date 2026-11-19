//! The cube [-5, 5]²×[-10, 0] and the half-space above the plane z = y·10⁻⁶, which crosses its top
//! at a slope of 10⁻⁶: their common part, a wedge 5·10⁻⁶ thick at its end.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::{between, half_space};

pub fn build() -> Result<Brep, String> {
    let cube = between([-5., -5., -10.], [5., 5., 0.], "cube")?;
    // the plane's normal toward the kept side, up and a millionth toward -y
    let above = half_space([0., 0., 0.], [0., -1e-6, 1.], 20., "above")?;
    intersection(&cube, &above).map_err(e)
}
