// The lens common to the ball of radius 5.511 on the origin and the ball of radius 3.317 on
// (4.231, 3.467, 2.653).
let m = model();
let lens = m.sphere(5.511);
lens = lens.intersect(m.tool(|m| m.sphere(3.317).translate(4.231, 3.467, 2.653)));
m
