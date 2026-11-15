// A block's beveled edge deleted: the top and the side meet again.
let m = model();
let b = m.box(20.0, 10.0, 10.0);
b = b.chamfer(edges().straight().direction("y").at_vertex([20.0, 0.0, 10.0]), 3.0, #{ label: "bevel" });
b = b.delete_faces(faces().facing("x").facing("z").or(faces().kind("plane").except(faces().facing("x")).except(faces().facing("y")).except(faces().facing("z")).except(faces().facing_neg("x")).except(faces().facing_neg("y")).except(faces().facing_neg("z"))), #{ label: "bevel_deleted" });
m
