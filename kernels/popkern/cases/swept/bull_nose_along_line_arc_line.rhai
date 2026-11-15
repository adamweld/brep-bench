// A block from (-10, -10, -10) to (40, 40, 0) less everything a bull-nose mill (radius 2, its
// bottom corner rounded to 0.5, from z = -1.5 up to 5, upright) passes through as its axis runs
// from (0, 0) along x to (20, 0), a quarter turn about the vertical through (20, 10) to (30, 10),
// and along y to (30, 30).
//
// The mill is a convex solid of revolution about its upright axis. Slid along a line it sweeps its
// section through the axis square to the line, carried along it; turned about a parallel line it
// sweeps that section in the half-plane through the turn's axis, revolved. And itself at each end.
let m = model();
let k = 0.5 * sqrt(0.5);
fn section(m, plane, c) {
    m.profile(plane).path(c - 1.5, -1.5).line_to(c + 1.5, -1.5).arc_to([c + 1.5 + 0.5 * sqrt(0.5), -1.0 - 0.5 * sqrt(0.5)], [c + 2.0, -1.0])
        .line_to(c + 2.0, 5.0).line_to(c - 2.0, 5.0).line_to(c - 2.0, -1.0)
        .arc_to([c - 1.5 - 0.5 * sqrt(0.5), -1.0 - 0.5 * sqrt(0.5)], [c - 1.5, -1.5]).close()
}
fn mill(m, x, y) {
    m.profile(xz()).path(0.0, -1.5).line_to(1.5, -1.5).arc_to([1.5 + 0.5 * sqrt(0.5), -1.0 - 0.5 * sqrt(0.5)], [2.0, -1.0])
        .line_to(2.0, 5.0).line_to(0.0, 5.0).close().sketch("mill")
        .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()).translate(x, y, 0.0)
}
let b = m.box(50.0, 50.0, 10.0, #{ at: [-10.0, -10.0, -10.0] });
b = b.subtract(m.tool(|m| section(m, yz(), 0.0).sketch("first").extrude(blind(20.0))));
// the turn's half-plane through (20, 10) toward -y holds the section about 10 out from it
b = b.subtract(m.tool(|m| section(m, xz(), 10.0).sketch("turn").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(90.0))
    .rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), -90.0).translate(20.0, 10.0, 0.0)));
b = b.subtract(m.tool(|m| section(m, xz_at(10.0), 30.0).sketch("last").extrude(blind(-20.0))));
for p in [[0.0, 0.0], [20.0, 0.0], [30.0, 10.0], [30.0, 30.0]] {
    b = b.subtract(m.tool(|m| mill(m, p[0], p[1])));
}
m
