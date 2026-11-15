// A block from the origin to (40, 30, 10) drilled through by a hole of radius 5 on (20, 15); then
// its top face moved up 6, the sides and the hole's wall running on to meet it.
let m = model();
let b = m.box(40.0, 30.0, 10.0);
b = b.subtract(m.tool(|m| m.cylinder(5.0, 12.0, #{ at: [20.0, 15.0, -1.0] })));
b = b.offset_faces(faces().facing("z").at_height(10.0), 6.0, #{ label: "top_moved" });
m
