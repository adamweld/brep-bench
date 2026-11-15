// A ball of radius 5 on the origin, poles on z, less everything where x < 0.
let m = model();
let ball = m.sphere(5.0);
ball = ball.subtract(m.tool(|m| m.box(6.0, 12.0, 12.0, #{ at: [-6.0, -6.0, -6.0] })));
m
