// A plate from the origin to (40, 30, 8) drilled through by two holes of radius 3 on (15, 15) and
// (25, 15); then both walls offset 3 into the plate: two holes of radius 6, run into one opening.
let m = model();
let b = m.box(40.0, 30.0, 8.0);
for x in [15.0, 25.0] {
    b = b.subtract(m.tool(|m| m.cylinder(3.0, 10.0, #{ at: [x, 15.0, -1.0] })));
}
b = b.offset_faces(faces().kind("cylinder"), -3.0, #{ label: "widened" });
m
