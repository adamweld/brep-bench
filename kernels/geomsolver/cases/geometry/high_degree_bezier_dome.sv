// A block under a Bézier patch of degree 15.
//
// Refused: geomsolver has no surface patch, and its splines are cubic B-splines only (any knots,
// any weights, but degree 3), so neither the degree-15 patch nor its degree-15 edge and iso-curves
// can be drawn. The construction its cubic patches use here — the block as a face family whose
// section is the patch's curve across y on the columns' crossings (`nurbs/bezier-dome`) — needs
// that curve as a spline of the patch's degree, and a curve written as a formula (a traced
// Bernstein sum) cannot stand in a face family's section: a trace in the station's component is
// not elaborated (`no such entity` for the station's own plane).
