// The upper half of a ball of radius 5; its rim chamfered 2 on the flat face and 1 on the ball.
let m = model();
let b = m.sphere(5.0, #{ lat: [0.0, 90.0] });
b = b.chamfer(edges().between("plane", "sphere"), 2.0,
    #{ kind: two_distance(faces().kind("plane"), 1.0), label: "bevel" });
m
