// #3DBenchy, whose answer is its designer's own B-rep: the reference STEP file's 1385 faces (361
// B-spline surfaces and 506 surfaces of extrusion among them).
//
// Refused: Solvent reads no STEP or other B-rep file (solventc only writes STEP), so the reference
// cannot be taken in, and a document drawing the boat from scratch — a hull lofted through its
// stations, the deck well, the wheelhouse with its framed doors and windows, the chimney's taper,
// the wheel, the lettering and the fillets — to the reference's volume, area and inertia within
// 2e-5 is out of reach of this implementation: geomsolver has no B-spline surface from a control
// net to take the reference's free-form faces as measured, so even a faithful redrawing would
// approximate them.
