// A frustum on z (radius 6 at z = 0, 3 at z = 4) bored through by a hole of radius 1, shelled
// inward to a wall 1 thick, open at its top.
let m = model();
let b = m.cone(6.0, 3.0, 4.0);
b = b.subtract(m.tool(|m| m.cylinder(1.0, 6.0, #{ at: [0.0, 0.0, -1.0] })));
b = b.shell(faces().facing("z").at_height(4.0), 1.0, #{ label: "hollow" });
m
