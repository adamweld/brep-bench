// The paraboloid x² + y² ≤ 8z up to z = 8, kept below the plane z = 1e-4: its tip.
let m = model();
let r = 8.0;
let p = m.profile(xz()).path(0.0, 0.0).conic_to([r / 2.0, 0.0], [r, 8.0], 1.0).line_to(0.0, 8.0).close()
    .sketch("paraboloid").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
p = p.subtract(m.tool(|m| m.box(20.0, 20.0, 10.0, #{ at: [-10.0, -10.0, 0.0001] })));
m
