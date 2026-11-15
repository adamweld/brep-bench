// A cube from the origin to (30, 30, 30), drilled through by fifteen holes of radius 2.5: three
// along x and three along y at z = 15, nine along z, on the lines 7.5, 15 and 22.5. Each hole
// runs 1 past both faces.
let m = model();
let cube = m.box(30.0, 30.0, 30.0);
for a in [7.5, 15.0, 22.5] {
    cube = cube.subtract(m.tool(|m| m.cylinder(2.5, 32.0, #{ at: [-1.0, a, 15.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] })));
    cube = cube.subtract(m.tool(|m| m.cylinder(2.5, 32.0, #{ at: [a, -1.0, 15.0], axis: [0.0, 1.0, 0.0], ref: [0.0, 0.0, 1.0] })));
    for b in [7.5, 15.0, 22.5] {
        cube = cube.subtract(m.tool(|m| m.cylinder(2.5, 32.0, #{ at: [a, b, -1.0] })));
    }
}
m
