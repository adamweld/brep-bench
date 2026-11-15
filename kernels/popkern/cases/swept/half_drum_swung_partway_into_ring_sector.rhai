// A ring sector (8 to 12 from the z axis, |z| ≤ 1, 0° to 60°) less everything half a drum (the half with y ≤ 0 of the cylinder of radius 1 about the vertical through (10, 0), |z| ≤ 0.5) passes through as it turns about z from -140° to 30°, flat face first. Its flat face lies in the half-plane through the axis, so its shadow is that face.
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
fn cutter(m) { m.cylinder(1.0, 1.0, #{ at: [10.0, 0.0, -0.5] }).subtract(m.tool(|m| m.box(4.0, 2.0, 2.0, #{ at: [8.0, 0.0, -1.0] }))) }
let m = model();
let b = block(m);
b = b.subtract(m.tool(|m| turned(cutter(m), -140.0)));
b = b.subtract(m.tool(|m| swing(m, m.profile(xz()).polygon([[9.0, -0.5], [11.0, -0.5], [11.0, 0.5], [9.0, 0.5]]), -140.0, 30.0)));
b = b.subtract(m.tool(|m| turned(cutter(m), 30.0)));
m
