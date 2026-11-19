// A cylinder of radius 5 on the z axis from z = 0 to z = 10 whose side is one NURBS surface, the
// tensor product of the rational quadratic circle of nine control points and a line of degree 1,
// its top rim rounded to radius 1.
//
// Refused: Solvent's splines are all cubic (one degree, 3, for every curve), so the side's
// rational quadratic circle — nine control points on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1)
// — cannot be written: `spline(c0, …, c8) knots [0, 0, 0, 0.25, …] weights [1, √2/2, …]` is
// refused "a curve needs more than 3 control points and a matching knot vector"; and the language
// has no surface stated by a control net, so the side cannot be given as the NURBS the case
// specifies.
