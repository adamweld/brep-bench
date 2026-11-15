// A cylinder's beveled rim deleted: the top and the side meet again.
let m = model();
let b = m.cylinder(6.0, 10.0);
b = b.chamfer(edges().at_height(10.0), 2.0, #{ label: "bevel" });
b = b.delete_faces(faces().kind("cone"), #{ label: "bevel_deleted" });
m
