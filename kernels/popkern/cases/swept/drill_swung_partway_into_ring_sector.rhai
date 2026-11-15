// A ring sector (8 to 12 from the z axis, |z| ≤ 1, 0° to 60°) less everything a drill (a cylinder of radius 1 about the vertical through (10, 0) up to z = 3, ended in a cone of 118° whose point is (10, 0, -0.8)) passes through as it turns about z from -140° to 30°.
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
fn cutter(m) { m.profile(xz()).polygon([[10.0, -0.8], [11.0, -0.8 + 1.0 / tan(59.0 * 3.141592653589793 / 180.0)], [11.0, 3.0], [10.0, 3.0]]).sketch("drill").revolve(axis_at([10.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()) }
let m = model();
let b = block(m);
b = b.subtract(m.tool(|m| turned(cutter(m), -140.0)));
b = b.subtract(m.tool(|m| swing(m, m.profile(xz()).polygon([[10.0, -0.8], [11.0, -0.8 + 1.0 / tan(59.0 * 3.141592653589793 / 180.0)], [11.0, 3.0], [9.0, 3.0], [9.0, -0.8 + 1.0 / tan(59.0 * 3.141592653589793 / 180.0)]]), -140.0, 30.0)));
b = b.subtract(m.tool(|m| turned(cutter(m), 30.0)));
m
