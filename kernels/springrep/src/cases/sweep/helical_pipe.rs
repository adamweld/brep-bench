//! A pipe of radius 1 along three right-handed turns of the helix of radius 10 about the z axis,
//! rising 5 a turn from (10, 0, 0), its section square to the path.
use springrep::topo::Brep;

use crate::shared::sweep::helical_pipe;

pub fn build() -> Result<Brep, String> {
    helical_pipe()
}
