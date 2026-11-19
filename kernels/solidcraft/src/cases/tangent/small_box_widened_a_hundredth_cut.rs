use crate::solids::*;

pub fn build() -> R<Body> {
    let (w, h) = (0.0015, 0.0015);
    let wide = w * 1.01;
    let copy = block((-wide / 2.0, -wide / 2.0, 0.0), (wide / 2.0, wide / 2.0, h))?;
    cut(&copy, &block((-w / 2.0, -w / 2.0, 0.0), (w / 2.0, w / 2.0, h))?)
}
