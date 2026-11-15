// The solid a square prism (section (4, 0), (6, 2), (8, 0), (6, -2), z = 0 to 3) fills as it turns
// 90° about the z axis.
//
// A turn about z sweeps the bar where it starts and stops and, between them, its shadow on a
// half-plane through the axis revolved: the bar is symmetric about the xz plane and reaches from
// 4 to 8 from the axis at every height, so its shadow is the rectangle 4 ≤ ρ ≤ 8, 0 ≤ z ≤ 3.
let m = model();
let bar = |m| m.profile(xy()).polygon([[4.0, 0.0], [6.0, 2.0], [8.0, 0.0], [6.0, -2.0]]).sketch("bar").extrude(blind(3.0));
let s = m.profile(xz()).polygon([[4.0, 0.0], [8.0, 0.0], [8.0, 3.0], [4.0, 3.0]]).sketch("shadow")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(90.0));
s = s.union(m.tool(|m| m.profile(xy()).polygon([[4.0, 0.0], [6.0, 2.0], [8.0, 0.0], [6.0, -2.0]]).sketch("start").extrude(blind(3.0))));
s = s.union(m.tool(|m| m.profile(xy()).polygon([[0.0, 4.0], [-2.0, 6.0], [0.0, 8.0], [2.0, 6.0]]).sketch("stop").extrude(blind(3.0))));
m
