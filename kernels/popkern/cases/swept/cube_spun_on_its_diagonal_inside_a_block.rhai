// A block from (-3, -3, -3) to (3, 3, 3) less everything the cube from (-1, -1, -1) to (1, 1, 1)
// passes through as it turns a whole turn about its long diagonal.
//
// A whole turn sweeps the cube's shadow on a half-plane through the diagonal revolved. Along the
// diagonal (t from -√3 to √3) the cube reaches √2 |t ± √3| from it, the cones its six edges at the
// diagonal's ends sweep, out to t = ∓1/√3; between, its six skew edges reach √(2 + 2t²), a
// hyperbola: one exact conic, its end tangents meeting at (0, √(3/2)). The shadow is drawn about
// the z axis and the void turned onto the diagonal.
let m = model();
let r3 = sqrt(3.0);
let edge = sqrt(8.0 / 3.0);
let shoulder = sqrt(1.5);
// the conic's weight puts its middle on the waist, (0, √2)
let w = (sqrt(2.0) - edge) / (shoulder - sqrt(2.0));
let cavity = m.tool(|m| m.profile(xz()).path(0.0, -r3).line_to(edge, -1.0 / r3)
    .conic_to([shoulder, 0.0], [edge, 1.0 / r3], w).line_to(0.0, r3).close().sketch("shadow")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
    .rotate(axis_at([0.0, 0.0, 0.0], [-1.0, 1.0, 0.0]), acos(1.0 / r3) * 180.0 / 3.141592653589793));
let b = m.box(6.0, 6.0, 6.0, #{ at: [-3.0, -3.0, -3.0] });
b = b.subtract(cavity);
m
