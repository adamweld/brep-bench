// A hole's countersink deleted: the hole runs straight up to the top.
let m = model();
let b = m.box(20.0, 10.0, 10.0);
b = b.subtract(m.tool(|m| m.cylinder(2.0, 12.0, #{ at: [10.0, 5.0, -1.0] })));
b = b.subtract(m.tool(|m| m.cone(2.0, 4.0, 2.0, #{ at: [10.0, 5.0, 8.0] })));
b = b.delete_faces(faces().kind("cone"), #{ label: "countersink_deleted" });
m
