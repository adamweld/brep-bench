//! The forward member of a crossed pair of torus sweeps. See cases/swept/tumbling-torus-crossed-forward.md for the continuous solid.
use crate::tumble::Tumble;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Tumble {
        centre: [3., 0., 2.],
        hub: 2., tube: 0.22, ratio: 0.7,
        lean: 45., heading: 20., height: 0.7071,
        observer: 1., roll: [-100., 100.],
    }.build()
}
