// A plate from (0, -5, 0) to (20, 5, 4), less everything a drill passes through as it slides 20
// down its own axis, the vertical line through (10, 0): a cylinder of radius 2 ended in a cone of
// 118° whose point starts at (10, 0, 6).
//
// The drill slides along its own axis, so what it sweeps is the drill lengthened: its point where
// the slide ends, its top where it starts, and the cylinder between, the section below revolved.
let m = model();
let pi = 3.141592653589793;
let rise = 2.0 / tan(59.0 * pi / 180.0);
let plate = m.box(20.0, 10.0, 4.0, #{ at: [0.0, -5.0, 0.0] });
let swept = m.tool(|m| m.profile(xz()).path(10.0, -14.0).line_to(12.0, -14.0 + rise).line_to(12.0, 14.0)
    .line_to(10.0, 14.0).close().sketch("drill_swept").revolve(axis_at([10.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()));
plate = plate.subtract(swept);
m
