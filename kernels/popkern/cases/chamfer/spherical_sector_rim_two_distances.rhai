// The spherical sector of a ball of radius 5 inside the cone about +z of half angle α, cos α = 3/5;
// its rim chamfered 2 down the cone and 1 on the ball.
let m = model();
let b = m.profile(xz()).path(0.0, 0.0).line_to(4.0, 3.0).arc_to([sqrt(5.0), 2.0 * sqrt(5.0)], [0.0, 5.0]).close()
    .sketch("sector").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
b = b.chamfer(edges().between("cone", "sphere"), 2.0,
    #{ kind: two_distance(faces().kind("cone"), 1.0), label: "bevel" });
m
