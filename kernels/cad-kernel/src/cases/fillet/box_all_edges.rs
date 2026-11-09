//! A cube of side 10 with all twelve edges rounded to radius 1, three rounds meeting at each corner.
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::rounds::rounded;
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    // every edge, rounded together: the kernel puts a cylinder along each edge, shrinks each face to
    // where the rounds touch it and closes each corner with the ball touching its three faces
    let edges: Vec<usize> = (0..cube.edges.len()).collect();
    rounded(&cube, &edges, ROUND, tolerance(cube.size()))
}
