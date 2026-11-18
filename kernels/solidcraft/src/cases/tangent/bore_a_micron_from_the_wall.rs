use crate::solids::*;

pub fn build() -> R<Body> {
    // the hole runs past both ends, as the description asks of a hole cut with a cylinder
    cut(&zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?, &zcyl(0.999999, 0.0, 4.0, -1.0, 11.0)?)
}
