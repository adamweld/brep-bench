// A frustum on z (radius 6 at z = 0, 3 at z = 4) shelled inward to a wall 2.7 thick, open at both
// ends.
let m = model();
let b = m.cone(6.0, 3.0, 4.0);
b = b.shell(faces().kind("plane"), 2.7, #{ label: "hollow" });
m
