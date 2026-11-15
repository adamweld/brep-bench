// A ring sector (8 to 12 from the z axis, |z| ≤ 1, 0° to 60°) less everything a flat ring (a torus about the vertical through (10, 0), middle radius 1, tube 0.4) passes through as it turns about z from -140° to 30°. Its shadow is everything within 1.4 of the vertical through (10, 0) at z = 0, rounded by the tube top and bottom.
//
// A turn about the z axis sweeps the tool where it starts and where it stops and, between them, the
// shadow of the tool on a half-plane through the axis (each point carried there round its circle
// about the axis) revolved through the turn: here the tool is symmetric about its own plane through
// the axis, so the shadow is its section in that plane.
fn block(m) {
    m.profile(xz()).polygon([[8.0, -1.0], [12.0, -1.0], [12.0, 1.0], [8.0, 1.0]]).sketch("block")
        .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(60.0))
}
fn swing(m, section, from, to) {
    section.sketch("shadow").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(to - from))
        .rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), from)
}
fn turned(body, angle) { body.rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), angle) }
fn cutter(m) { m.torus(1.0, 0.4, #{ at: [10.0, 0.0, 0.0] }) }
let m = model();
let b = block(m);
b = b.subtract(m.tool(|m| turned(cutter(m), -140.0)));
b = b.subtract(m.tool(|m| swing(m, m.profile(xz()).path(9.0, -0.4).line_to(11.0, -0.4).arc_to([11.4, 0.0], [11.0, 0.4]).line_to(9.0, 0.4).arc_to([8.6, 0.0], [9.0, -0.4]).close(), -140.0, 30.0)));
b = b.subtract(m.tool(|m| turned(cutter(m), 30.0)));
m
