// The ball of radius 5 on the origin below z = 3; its rim chamfered 2 on the flat face and 1 on
// the ball.
let m = model();
let b = m.sphere(5.0);
b = b.subtract(m.tool(|m| m.box(12.0, 12.0, 4.0, #{ at: [-6.0, -6.0, 3.0] })));
b = b.chamfer(edges().between("plane", "sphere"), 2.0,
    #{ kind: two_distance(faces().kind("plane"), 1.0), label: "bevel" });
m
