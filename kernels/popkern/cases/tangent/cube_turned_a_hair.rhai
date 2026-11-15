// A cube from (-5, -5, 0) to (5, 5, 10) united with itself turned 0.001° about z.
let m = model();
let c = m.box(10.0, 10.0, 10.0, #{ at: [-5.0, -5.0, 0.0] });
c = c.union(m.tool(|m| m.box(10.0, 10.0, 10.0, #{ at: [-5.0, -5.0, 0.0] })
    .rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 0.001)));
m
