use crate::solids::*;

/// The block from the origin to (10, 10, 5) with its vertical edge at x = y = 10 broken by a flat
/// 0.0001 back along each side: the prism on (0, 0), (10, 0), (10, 9.9999), (9.9999, 10), (0, 10).
/// The flat's two vertical edges rounded together with a radius of 1, each picked at its middle;
/// the round is wider than the flat and swallows it, so this is `fillet_removing`, the fillet whose
/// rounds may overflow their faces and take them away.
pub fn build() -> R<Body> {
    let b = 10.0 - 0.0001;
    let section = polygon(&[(0.0, 0.0), (10.0, 0.0), (10.0, b), (b, 10.0), (0.0, 10.0)]);
    let block = extrude(&Plane::XY, section, 0.0, 5.0)?;
    let picks = [v3(10.0, b, 2.5), v3(b, 10.0, 2.5)];
    let (rounded, _gone) =
        solidcraft_kernel::fillet_removing(&block, &picks, 1.0, solidcraft_kernel::FilletTurn::Mitre)?;
    Ok(rounded)
}
