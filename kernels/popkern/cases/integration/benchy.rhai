// #3DBenchy, the reference solid in benchy.step.gz.
//
// Refused: the case asks for the original part itself, a B-rep of 1385 faces, 867 of them B-spline
// surfaces and surfaces of extrusion; popkern's scripts make B-spline surfaces from their nets but
// sew no faces into a solid, and an implementation may not read the reference file.
throw "popkern's scripts sew no faces into a solid, so the Benchy's 1385 faces cannot be given as one";
