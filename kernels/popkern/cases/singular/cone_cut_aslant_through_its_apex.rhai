// A cone on the disk of radius 5 at z = 0, apex (0, 0, 10), less the side 4x + z > 10 of the
// plane through its apex.
let m = model();
let cone = m.cone(5.0, 0.0, 10.0);
let beyond = m.tool(|m| m.profile(xz()).path(2.75, -1.0).line_to(20.0, -1.0).line_to(20.0, 11.0)
    .line_to(-0.25, 11.0).close().sketch("beyond").extrude(mid_plane(30.0)));
cone = cone.subtract(beyond);
m
