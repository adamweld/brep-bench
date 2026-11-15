// A ball of radius 5 on the origin, poles on z, less the cap beyond the plane x + z = 5, which
// passes through the pole (0, 0, 5).
let m = model();
let ball = m.sphere(5.0);
// a slab of the half-space x + z > 5: its section in y = 0, extruded both ways along y
let beyond = m.tool(|m| m.profile(xz()).path(11.0, -6.0).line_to(20.0, -6.0).line_to(20.0, 20.0)
    .line_to(-15.0, 20.0).close().sketch("beyond").extrude(mid_plane(30.0)));
ball = ball.subtract(beyond);
m
