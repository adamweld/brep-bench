// A box with every edge rounded, all twenty rounds deleted.
let m = model();
let b = m.box(20.0, 16.0, 12.0);
b = b.fillet(edges(), 2.0, #{ label: "rounds" });
b = b.delete_faces(faces().kind("cylinder").or(faces().kind("sphere")), #{ label: "rounds_deleted" });
m
