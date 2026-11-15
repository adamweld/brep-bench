// A plate from the origin to (40, 30, 8) drilled through by a hole of radius 5 on (20, 15); then the
// hole's wall offset 2 into the plate (against its outward normal): a hole of radius 7.
let m = model();
let b = m.box(40.0, 30.0, 8.0);
b = b.subtract(m.tool(|m| m.cylinder(5.0, 10.0, #{ at: [20.0, 15.0, -1.0] })));
b = b.offset_faces(faces().kind("cylinder"), -2.0, #{ label: "widened" });
m
