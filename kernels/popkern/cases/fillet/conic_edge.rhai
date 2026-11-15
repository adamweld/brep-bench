// A block from the origin to (20, 10, 10), the edge where its top meets x = 20 blended by a conic
// of rho 0.5 (a parabola) tangent to each face 3 from the edge.
//
// Refused: popkern's blends are rolling-ball rounds (circular, or G2 with a curvature-continuous
// section) and chamfers; it has no conic blend of a given rho.
throw "popkern has no conic blend: its rounds are rolling-ball sections, circular or G2, and its \
chamfers flat";
