// A post (radius 3 on z, z = 0 to 10) joined to a ball of radius 5 on (0, 0, 10); the concave
// circle where they meet chamfered 2 down the post and 1 on the ball.
let m = model();
let b = m.cylinder(3.0, 10.0);
b = b.union(m.tool(|m| m.sphere(5.0, #{ at: [0.0, 0.0, 10.0] })));
b = b.chamfer(edges().between("cylinder", "sphere"), 2.0,
    #{ kind: two_distance(faces().kind("cylinder"), 1.0), label: "bevel" });
m
