// A plate from the origin to (40, 30, 5) with a square boss from (12, 7, 5) to (28, 23, 15); the
// boss's upright edges rounded to 3, then its root, one tangent chain, to 2.
let m = model();
let b = m.box(40.0, 30.0, 5.0);
b = b.union(m.tool(|m| m.box(16.0, 16.0, 10.0, #{ at: [12.0, 7.0, 5.0] })));
b = b.fillet(edges().straight().direction("z").chord_length_between(9.5, 10.5), 3.0, #{ label: "uprights" });
b = b.fillet(edges().at_height(5.0).straight().chord_length_between(9.5, 10.5).or(edges().at_height(5.0).circular()),
    2.0, #{ label: "root" });
m
