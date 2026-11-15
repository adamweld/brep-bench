// A block's rounded edge deleted: the top and the side meet again.
let m = model();
let b = m.box(20.0, 10.0, 10.0);
b = b.fillet(edges().straight().direction("y").at_vertex([20.0, 0.0, 10.0]), 2.0, #{ label: "round" });
b = b.delete_faces(faces().kind("cylinder"), #{ label: "round_deleted" });
m
