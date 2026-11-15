// A boss's rounded root deleted: the boss stands on the plate with a sharp foot.
let m = model();
let b = m.cylinder(8.0, 4.0, #{ at: [0.0, 0.0, -4.0] });
b = b.union(m.tool(|m| m.cylinder(3.0, 8.0)));
b = b.fillet(edges().between("cylinder", "plane").at_height(0.0), 1.0, #{ label: "round" });
b = b.delete_faces(faces().kind("torus"), #{ label: "round_deleted" });
m
