// Three rounds meeting at a cube's corner deleted, with the ball between them.
let m = model();
let b = m.box(10.0, 10.0, 10.0);
b = b.fillet(edges().at_vertex([10.0, 10.0, 10.0]), 2.0, #{ label: "rounds" });
b = b.delete_faces(faces().kind("cylinder").or(faces().kind("sphere")), #{ label: "rounds_deleted" });
m
