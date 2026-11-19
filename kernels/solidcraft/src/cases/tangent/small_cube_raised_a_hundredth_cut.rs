use crate::solids::*;

pub fn build() -> R<Body> {
    let s = 0.0015;
    let raised = block((0.0, 0.0, s / 100.0), (s, s, s + s / 100.0))?;
    cut(&block((0.0, 0.0, 0.0), (s, s, s))?, &raised)
}
