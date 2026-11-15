// Two spherical sectors of a ball of radius 10, half angle 45°, apex on the origin: one about +z,
// one turned 60° about y. Their common part.
fn sector(m) {
    // the cone of half angle 45° with its apex on the origin, closed by a cap of the sphere
    let pi = 3.141592653589793;
    let s = 10.0 * sin(pi / 4.0);
    m.profile(xz()).path(0.0, 0.0).line_to(s, s).arc_to([10.0 * cos(3.0 * pi / 8.0), 10.0 * sin(3.0 * pi / 8.0)], [0.0, 10.0]).close()
        .sketch("sector").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}
let m = model();
let a = sector(m);
a = a.intersect(m.tool(|m| sector(m).rotate(axis_at([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]), 60.0)));
m
