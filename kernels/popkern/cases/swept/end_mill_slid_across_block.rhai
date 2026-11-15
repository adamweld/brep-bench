// A block from (0, -5, 0) to (20, 5, 10), less everything an upright end mill (radius 2, z = 7 to
// 15) passes through as it slides along x from x = -5 to 25.
//
// What a cylinder sliding square to its axis sweeps is the mill where it starts and where it stops,
// and between them its section through the axis square to the slide carried along it.
let m = model();
let block = m.box(20.0, 10.0, 10.0, #{ at: [0.0, -5.0, 0.0] });
let swept = m.tool(|m| m.box(30.0, 4.0, 8.0, #{ at: [-5.0, -2.0, 7.0] }));
block = block.subtract(swept);
block = block.subtract(m.tool(|m| m.cylinder(2.0, 8.0, #{ at: [-5.0, 0.0, 7.0] })));
block = block.subtract(m.tool(|m| m.cylinder(2.0, 8.0, #{ at: [25.0, 0.0, 7.0] })));
m
