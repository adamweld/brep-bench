// A cylinder's rounded rim deleted: the top and the side meet again.
let m = model();
let b = m.cylinder(6.0, 10.0);
b = b.fillet(edges().at_height(10.0), 2.0, #{ label: "round" });
b = b.delete_faces(faces().kind("torus"), #{ label: "round_deleted" });
m
