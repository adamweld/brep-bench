use crate::solids::*;

pub fn build() -> R<Body> {
    let (r, h) = (0.00075, 0.0015);
    cut(&zcyl(0.0, 0.0, r * 1.01, 0.0, h)?, &zcyl(0.0, 0.0, r, 0.0, h)?)
}
