// A block from the origin to (40, 30, 10), the edge where its top meets its back rounded to 3, then
// its top moved up 6: popkern's offset_faces carries the round with the face it blends.
let m = model();
let b = m.box(40.0, 30.0, 10.0);
b = b.fillet(edges().straight().direction("x").at_vertex([0.0, 30.0, 10.0]), 3.0, #{ label: "round" });
b = b.offset_faces(faces().facing("z").at_height(10.0), 6.0, #{ label: "moved" });
m
