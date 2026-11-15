// The ball of radius 10 less the ball of radius 8 at (1, 0, 0), joined with the ball of radius 6
// at (1, 1, 0) less that of radius 4 at (1, 1, 1), joined with the ball of radius 2 at
// (1, 1.5, 1.5): a hollow ball holding a hollow ball holding a ball.
let m = model();
let outer = m.sphere(10.0);
outer = outer.subtract(m.tool(|m| m.sphere(8.0).translate(1.0, 0.0, 0.0)));
let middle = m.tool(|m| m.sphere(6.0).translate(1.0, 1.0, 0.0)
    .subtract(m.tool(|m| m.sphere(4.0).translate(1.0, 1.0, 1.0))));
outer = outer.union(middle);
outer = outer.union(m.tool(|m| m.sphere(2.0).translate(1.0, 1.5, 1.5)));
m
