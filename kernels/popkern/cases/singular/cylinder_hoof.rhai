// A cylinder of radius 5 on z from z = -1 to 10, kept above z = 0 and below z = x tan 30°.
let m = model();
let pi = 3.141592653589793;
let t = tan(30.0 * pi / 180.0);
let hoof = m.cylinder(5.0, 11.0, #{ at: [0.0, 0.0, -1.0] });
hoof = hoof.subtract(m.tool(|m| m.box(12.0, 12.0, 2.0, #{ at: [-6.0, -6.0, -2.0] })));
// the half-space above z = x tan 30°: its section in y = 0, extruded both ways
let above = m.tool(|m| m.profile(xz()).path(-6.0, -6.0 * t).line_to(6.0, 6.0 * t).line_to(6.0, 12.0)
    .line_to(-6.0, 12.0).close().sketch("above").extrude(mid_plane(14.0)));
hoof = hoof.subtract(above);
m
