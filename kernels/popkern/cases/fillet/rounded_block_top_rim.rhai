// A block 20 by 12 by 5 on (-10, -6, 0), its vertical edges rounded to 3; then the rim of its top
// rounded to 1.
let m = model();
let b = m.box(20.0, 12.0, 5.0, #{ at: [-10.0, -6.0, 0.0] });
b = b.fillet(edges().straight().direction("z"), 3.0, #{ label: "corners" });
b = b.fillet(edges().at_height(5.0), 1.0, #{ label: "rim" });
m
