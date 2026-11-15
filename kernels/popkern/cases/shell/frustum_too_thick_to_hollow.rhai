// A frustum on z (radius 6 at z = 0, 3 at z = 4) shelled inward to a wall 3.5 thick, open at its
// top.
let m = model();
let b = m.cone(6.0, 3.0, 4.0);
b = b.shell(faces().facing("z").at_height(4.0), 3.5, #{ label: "hollow" });
m
