// A ring sector (8 to 12 from the z axis, |z| ≤ 1, 0° to 60°) less everything a domed mill (a cylinder of radius 1 about the vertical through (10, 0) up to z = 3, ended by a cap of the sphere of radius 2 about (10, 0, 1.2)) passes through as it turns about z from -140° to 30°.
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
fn cutter(m) { m.profile(xz()).path(10.0, -0.8).arc_to([10.0 + 2.0 * sin(0.25), 1.2 - 2.0 * cos(0.25)], [11.0, 1.2 - sqrt(3.0)]).line_to(11.0, 3.0).line_to(10.0, 3.0).close().sketch("mill").revolve(axis_at([10.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()) }
let m = model();
let b = block(m);
b = b.subtract(m.tool(|m| turned(cutter(m), -140.0)));
b = b.subtract(m.tool(|m| swing(m, m.profile(xz()).path(9.0, 1.2 - sqrt(3.0)).arc_to([10.0, -0.8], [11.0, 1.2 - sqrt(3.0)]).line_to(11.0, 3.0).line_to(9.0, 3.0).close(), -140.0, 30.0)));
b = b.subtract(m.tool(|m| turned(cutter(m), 30.0)));
m
