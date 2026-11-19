use crate::solids::*;

pub fn build() -> R<Body> {
    let (r, h) = (0.00075, 0.0015);
    let raised = zcyl(0.0, 0.0, r, h / 100.0, h + h / 100.0)?;
    common(&zcyl(0.0, 0.0, r, 0.0, h)?, &raised)
}
