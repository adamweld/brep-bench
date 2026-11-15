// The lens common to balls of radius 5 on (0, 0, 3) and (0, 0, -3); its rim chamfered 2 on the
// upper face (of the ball about (0, 0, -3)) and 1 on the lower.
let m = model();
let b = m.sphere(5.0, #{ at: [0.0, 0.0, -3.0] });
m.label_face("upper_face", faces().kind("sphere"));
b = b.intersect(m.tool(|m| m.sphere(5.0, #{ at: [0.0, 0.0, 3.0] })));
b = b.chamfer(edges().at_height(0.0), 2.0,
    #{ kind: two_distance(faces().labeled("upper_face"), 1.0), label: "bevel" });
m
