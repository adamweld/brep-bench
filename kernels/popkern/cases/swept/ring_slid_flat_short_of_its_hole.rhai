// The solid a torus about z (middle radius 6, tube 2) fills as it slides along x by 6.
//
// The torus with its hole filled (everything within 6 + √(4 - z²) of the z axis) is convex: it
// sweeps itself where it starts and stops and, between them, its section in the plane x = 0
// carried along the slide. Of the hole, what no place of the ring covers is what the holes where it
// starts and where it stops share (each everything within 6 - √(4 - z²) of the ring's axis).
let m = model();
fn filled(m, x) {
    m.profile(xz()).path(0.0, -2.0).line_to(6.0, -2.0).arc_to([8.0, 0.0], [6.0, 2.0]).line_to(0.0, 2.0).close()
        .sketch("filled").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()).translate(x, 0.0, 0.0)
}
fn hole(m, x) {
    m.profile(xz()).path(0.0, -2.0).line_to(6.0, -2.0).arc_to([4.0, 0.0], [6.0, 2.0]).line_to(0.0, 2.0).close()
        .sketch("hole").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()).translate(x, 0.0, 0.0)
}
let s = filled(m, 0.0);
s = s.union(m.tool(|m| filled(m, 6.0)));
s = s.union(m.tool(|m| m.profile(yz()).path(-6.0, -2.0).line_to(6.0, -2.0).arc_to([8.0, 0.0], [6.0, 2.0])
    .line_to(-6.0, 2.0).arc_to([-8.0, 0.0], [-6.0, -2.0]).close().sketch("section").extrude(blind(6.0))));
s = s.subtract(m.tool(|m| hole(m, 0.0).intersect(m.tool(|m| hole(m, 6.0)))));
m
