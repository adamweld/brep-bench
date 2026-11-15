// The solid a torus (axis the vertical through (8, 0), middle radius 3, tube 1) fills as it turns a
// whole turn about the z axis.
//
// A whole turn about z sweeps the torus's shadow on a half-plane through z revolved: at height z
// the torus reaches from 5 - √(1 - z²) to 11 + √(1 - z²) from the axis, a stadium between half
// discs of radius 1 about (5, 0) and (11, 0).
let m = model();
m.profile(xz()).path(5.0, -1.0).line_to(11.0, -1.0).arc_to([12.0, 0.0], [11.0, 1.0]).line_to(5.0, 1.0)
    .arc_to([4.0, 0.0], [5.0, -1.0]).close().sketch("shadow")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
