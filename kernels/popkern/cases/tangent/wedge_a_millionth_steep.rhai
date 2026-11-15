// A cube from (-5, -5, -10) to (5, 5, 0), kept above the plane z = 1e-6 y: a wedge 5e-6 thick at
// most.
let m = model();
let c = m.box(10.0, 10.0, 10.0, #{ at: [-5.0, -5.0, -10.0] });
let below = m.tool(|m| m.profile(yz()).path(-6.0, -6.0e-6).line_to(6.0, 6.0e-6).line_to(6.0, -12.0)
    .line_to(-6.0, -12.0).close().sketch("below").extrude(mid_plane(12.0)));
c = c.subtract(below);
m
