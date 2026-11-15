// A barrel-vaulted block on the square |x|, |y| <= 12, with everything above its crown, z = 16,
// cut away by a box whose floor is that plane: the floor touches the vault along its crown.
//
// The vault's top is the case's rational B-spline from its control net (sheet_nurbs: degree 1
// along x, 2 along y, weights 1, 3/5, 1), the block split by it and the part below kept.
let m = model();
let row = |x| [[x, -12.0, 10.0], [x, 0.0, 26.0], [x, 12.0, 10.0]];
let top = m.sheet_nurbs([row.call(-12.0), row.call(12.0)],
    #{ degree: [1, 2], weights: [[1.0, 0.6, 1.0], [1.0, 0.6, 1.0]], label: "vault" });
let block = m.box(24.0, 24.0, 30.0, #{ at: [-12.0, -12.0, 0.0] }).split([top], containing([0.0, 0.0, 1.0]))[0];
block = block.subtract(m.tool(|m| m.box(40.0, 40.0, 20.0, #{ at: [-20.0, -20.0, 16.0] })), #{ label: "cut" });
m
