// A plate from (-10, -10, 0) to (10, 10, 4) less everything a core drill passes through as it
// slides 24 along its own leaning axis (through (0, 0, 2) along (sin 30°, 0, cos 30°)): a tube
// between radii 1.5 and 2.5.
//
// Sliding along its own axis, the tube sweeps itself lengthened, from 18 before (0, 0, 2) to 18
// past it. The core inside it is cut free and stays.
let m = model();
let pi = 3.141592653589793;
let u = [sin(pi / 6.0), 0.0, cos(pi / 6.0)];
let start = [-18.0 * u[0], 0.0, 2.0 - 18.0 * u[2]];
let tube = m.tool(|m| m.cylinder(2.5, 36.0, #{ at: start, axis: u, ref: [0.0, 1.0, 0.0] })
    .subtract(m.tool(|m| m.cylinder(1.5, 36.0, #{ at: start, axis: u, ref: [0.0, 1.0, 0.0] }))));
let plate = m.box(20.0, 20.0, 4.0, #{ at: [-10.0, -10.0, 0.0] });
plate = plate.subtract(tube);
m
