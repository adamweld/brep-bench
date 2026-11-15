// A block from (-10, -10, 0) to (10, 10, 10) united with the paraboloid x² + y² ≤ 8z up to
// z = 12.5, its vertex touching the block's bottom face.
let m = model();
let b = m.box(20.0, 20.0, 10.0, #{ at: [-10.0, -10.0, 0.0] });
b = b.union(m.tool(|m| m.profile(xz()).path(0.0, 0.0).conic_to([5.0, 0.0], [10.0, 12.5], 1.0).line_to(0.0, 12.5).close()
    .sketch("paraboloid").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())));
m
