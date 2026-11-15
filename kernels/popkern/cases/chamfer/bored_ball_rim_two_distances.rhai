// A ball of radius 5 bored through along z by a hole of radius 2; the upper rim chamfered 2 down
// the bore and 1 on the ball.
let m = model();
let b = m.sphere(5.0);
b = b.subtract(m.tool(|m| m.cylinder(2.0, 12.0, #{ at: [0.0, 0.0, -6.0] })));
b = b.chamfer(edges().between("cylinder", "sphere").at_height(sqrt(21.0)), 2.0,
    #{ kind: two_distance(faces().kind("cylinder"), 1.0), label: "bevel" });
m
