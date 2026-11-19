//! Two boxes united on the plane z = 4, where each has a flat B-spline face: (0, 0, 0)–(10, 10, 4)
//! with a bicubic top and (5, 3, 4)–(15, 13, 7) with a biquadratic bottom.
use springrep::boolean::union;
use springrep::primitive::slab;
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    // A slab's top is a B-spline height field; at one height everywhere it is flat. Its control
    // points stand evenly over the square, not at the case's uneven x and y, which moves no point
    // of the face: each patch is its square of the plane either way.
    let cubic = [0., 0., 0., 0., 1., 1., 1., 1.];
    let lower = slab(&at([0., 0., 0.]), [10., 10.], (&cubic, &cubic), &vec![vec![4.; 4]; 4], "a")
        .map_err(e)?;
    // the upper box hangs from its top, z = 7, its B-spline face 3 below it
    let quadratic = [0., 0., 0., 1., 1., 1.];
    let hanging = frame([5., 13., 7.], [0., 0., -1.], [1., 0., 0.])?;
    let upper = slab(&hanging, [10., 10.], (&quadratic, &quadratic), &vec![vec![3.; 3]; 3], "b")
        .map_err(e)?;
    union(&lower, &upper).map_err(e)
}
