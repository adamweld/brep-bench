// A NURBS ball of radius 5, its whole surface one biquadratic rational B-spline face (a 9 × 5 net
// with a pole at each end of its meridians), grown by 2 by offsetting that face along its normal.
//
// Refused: Solvent has no spline surface to make the face from, and no spline but a cubic one. A
// `spline` is a curve of degree 3 only (`knots` for the five-point rational quadratic semicircle
// are "a curve needs more than 3 control points and a matching knot vector"), and every surface
// comes of sweeping a curve, so the case's 9 × 5 biquadratic net cannot be written. The nearest
// solid Solvent builds — a meridian of two rational cubic quarter circles turned about the axis —
// is a surface of revolution, not the NURBS face the case asks for, and its `std.rounds.Grown`
// body is not built exactly either ("a face on a revolution: read by the field").
