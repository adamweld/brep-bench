// A frustum on z (radius 7 at z = 0, 1 at z = 8) shelled inward to a wall 2 thick, open at its
// top.
let m = model();
let b = m.cone(7.0, 1.0, 8.0);
b = b.shell(faces().facing("z").at_height(8.0), 2.0, #{ label: "hollow" });
m
