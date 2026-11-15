// The Utah teapot made a solid, from its 32 bicubic Bézier patches.
//
// Refused: popkern's scripts make each Bézier patch (sheet_nurbs) but sew no faces into a solid,
// and the teapot's four solids are each bounded by several patches and flat faces.
throw "popkern's scripts sew no faces into a solid: the teapot's patches cannot close its four solids";
