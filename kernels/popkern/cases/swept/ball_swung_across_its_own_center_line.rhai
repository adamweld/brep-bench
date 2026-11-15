// The solid a ball (radius 2, centre (1, 0, 0)) fills as it turns 120° about the z axis.
//
// A turn about z sweeps the ball where it starts and stops and, between them, its shadow on a
// half-plane through the axis revolved: the disc of radius 2 about (1, 0) cut by the axis.
let m = model();
let pi = 3.141592653589793;
let s = m.sphere(2.0, #{ at: [1.0, 0.0, 0.0] });
s = s.union(m.tool(|m| m.profile(xz()).path(0.0, -sqrt(3.0)).arc_to([3.0, 0.0], [0.0, sqrt(3.0)]).close()
    .sketch("shadow").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(120.0))));
s = s.union(m.tool(|m| m.sphere(2.0, #{ at: [cos(2.0 * pi / 3.0), sin(2.0 * pi / 3.0), 0.0] })));
m
