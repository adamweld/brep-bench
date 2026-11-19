//! A hollow square duct narrowing round a quarter turn of radius 30 about the z axis, its outer
//! half-width 9 → 6 linearly with the angle, its wall 2.
use springrep::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("springrep has no sweep with a scaling law: sweep::along and sweep::revolve carry a \
         fixed section, and a loft through sections (which takes only outer loops) skins its \
         sides as Bézier patches across them, not the spirals the section's corners trace"
        .into())
}
