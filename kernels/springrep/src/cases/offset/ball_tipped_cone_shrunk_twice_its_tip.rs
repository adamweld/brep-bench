//! A cone of base radius 6 and height 12 with its tip rounded by a ball of radius 1, shrunk by 2:
//! the ball's face vanishes and the moved side runs on to its own apex, a plain cone.
//! springrep 1.0.1 refuses it: its face offset cannot follow the ball-rounded tip moved past its
//! radius ("an offset … closes a sphere to nothing").
use springrep::topo::Brep;

use crate::shared::offset::{ball_tipped_cone, shrunk};

pub fn build() -> Result<Brep, String> {
    shrunk(&ball_tipped_cone()?, 2.)
}
