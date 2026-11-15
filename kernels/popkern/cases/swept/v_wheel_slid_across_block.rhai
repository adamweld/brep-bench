// A block from (0, -5, 0) to (20, 5, 10), less everything a V wheel passes through as it slides
// along x by 36: about the line through (-8, 0, 14) parallel to y, the triangle (y, z) = (0, 8),
// (∓3 tan 30°, 11) in the plane x = -8 revolved.
//
// Sliding square to its axis, the wheel sweeps itself where it starts and stops and, between them,
// its section through the axis square to the slide (the triangle and its mirror above the axis)
// carried along it.
let m = model();
let pi = 3.141592653589793;
let h = 3.0 * tan(30.0 * pi / 180.0);
let block = m.box(20.0, 10.0, 10.0, #{ at: [0.0, -5.0, 0.0] });
for x in [-8.0, 28.0] {
    block = block.subtract(m.tool(|m| m.profile(yz_at(x)).polygon([[0.0, 8.0], [h, 11.0], [-h, 11.0]])
        .sketch("wheel_section").revolve(axis_at([x, 0.0, 14.0], [0.0, 1.0, 0.0]), full_circle())));
}
for tri in [[[0.0, 8.0], [h, 11.0], [-h, 11.0]], [[0.0, 20.0], [-h, 17.0], [h, 17.0]]] {
    block = block.subtract(m.tool(|m| m.profile(yz_at(-8.0)).polygon(tri).sketch("v_section").extrude(blind(36.0))));
}
m
