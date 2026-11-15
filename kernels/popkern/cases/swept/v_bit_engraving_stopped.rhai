// A block from (0, -10, -10) to (30, 10, 0), less everything a V bit passes through as it slides
// along x by 15: a cone on the vertical line through (5, 0), its point at (5, 0, -2), opening at
// 90° to a flat top of radius 5 at z = 3.
//
// Sliding square to its axis, the cone sweeps itself where it starts and stops and, between them,
// its section through the axis square to the slide (a triangle) carried along it.
let m = model();
let block = m.box(30.0, 20.0, 10.0, #{ at: [0.0, -10.0, -10.0] });
block = block.subtract(m.tool(|m| m.cone(0.0, 5.0, 5.0, #{ at: [5.0, 0.0, -2.0] })));
block = block.subtract(m.tool(|m| m.cone(0.0, 5.0, 5.0, #{ at: [20.0, 0.0, -2.0] })));
block = block.subtract(m.tool(|m| m.profile(yz_at(5.0)).polygon([[0.0, -2.0], [5.0, 3.0], [-5.0, 3.0]])
    .sketch("v_section").extrude(blind(15.0))));
m
