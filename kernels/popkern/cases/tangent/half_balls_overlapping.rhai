// The half of the ball of radius 5 where z ≥ 0 (poles on z) united with the half where x ≥ 0
// (poles on x).
let m = model();
let a = m.sphere(5.0, #{ lat: [0.0, 90.0] });
a = a.union(m.tool(|m| m.sphere(5.0, #{ axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0], lat: [0.0, 90.0] })));
m
