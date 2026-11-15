// The paraboloid x² + y² ≤ 10z up to z = 5, united with the ball of radius 5 on (0, 0, 5): its
// bottom fits the cup's vertex to more than first order.
let m = model();
// the parabola z = x²/10 from the vertex to (√50, 5) as one exact conic: a quadratic Bézier
// whose middle control point is where its end tangents meet, (√50/2, 0)
let r = sqrt(50.0);
let cup = m.profile(xz()).path(0.0, 0.0).conic_to([r / 2.0, 0.0], [r, 5.0], 1.0).line_to(0.0, 5.0).close()
    .sketch("cup").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
cup = cup.union(m.tool(|m| m.sphere(5.0, #{ at: [0.0, 0.0, 5.0] })));
m
