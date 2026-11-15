// A ring sector (8 to 12 from the z axis, |z| ≤ 1, 0° to 60°) less everything a pin (radius 0.5 about the x axis, x = 6 to 11) passes through as it turns about z from -140° to 140°. Its shadow runs out to its far rim's corners, √(11² + 0.5²) from the origin.
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
fn cutter(m) { m.cylinder(0.5, 5.0, #{ at: [6.0, 0.0, 0.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] }) }
let m = model();
let b = block(m);
b = b.subtract(m.tool(|m| turned(cutter(m), -140.0)));
b = b.subtract(m.tool(|m| swing(m, m.profile(xz()).path(6.0, -0.5).line_to(11.0, -0.5).arc_to([sqrt(121.25), 0.0], [11.0, 0.5]).line_to(6.0, 0.5).close(), -140.0, 140.0)));
b = b.subtract(m.tool(|m| turned(cutter(m), 140.0)));
m
