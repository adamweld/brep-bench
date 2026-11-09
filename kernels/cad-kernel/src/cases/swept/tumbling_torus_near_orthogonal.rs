//! A thin torus tumbled about nearly perpendicular axes. See cases/swept/tumbling-torus-near-orthogonal.md for the continuous solid.
use crate::tumble::Tumble;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Tumble {
        centre: [2.02, 0., 1.],
        hub: 2., tube: 0.08, ratio: 1.01,
        lean: 1., heading: 89., height: 0.,
        observer: 1., roll: [-180., 180.],
    }.build()
}
