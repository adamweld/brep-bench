//! A prism 0.001515 square about the z axis, 0.0015 tall, less the prism 0.0015 square between the
//! same planes: a square tube whose walls are 7.5·10⁻⁶ thick.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::tangent::between;

pub fn build() -> Result<Brep, String> {
    let (h, w) = (0.0015 / 2., 0.001515 / 2.);
    let wide = between([-w, -w, 0.], [w, w, 0.0015], "wide")?;
    let prism = between([-h, -h, 0.], [h, h, 0.0015], "prism")?;
    difference(&wide, &prism).map_err(e)
}
