// The solid a rod (radius 0.5 about the x axis, x = -3 to 5) fills as it turns 120° about the z
// axis.
//
// A turn about z sweeps the rod where it starts and stops and, between them, each arm's shadow on
// a half-plane through the axis revolved: the arm along +x reaches √(25.25 - z²) from the origin at
// height z (its far rim's corners), a zone of a sphere, and the arm along -x √(9.25 - z²); the -x
// arm's shadow turns from 180° to 300°.
let m = model();
let fan = |m, r| m.profile(xz()).path(0.0, -0.5).line_to(sqrt(r * r - 0.25), -0.5)
    .arc_to([r, 0.0], [sqrt(r * r - 0.25), 0.5]).line_to(0.0, 0.5).close().sketch("fan")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(120.0));
let s = m.cylinder(0.5, 8.0, #{ at: [-3.0, 0.0, 0.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] });
s = s.union(m.tool(|m| fan.call(m, sqrt(25.25))));
s = s.union(m.tool(|m| fan.call(m, sqrt(9.25)).rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 180.0)));
s = s.union(m.tool(|m| m.cylinder(0.5, 8.0, #{ at: [-3.0, 0.0, 0.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] })
    .rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 120.0)));
m
