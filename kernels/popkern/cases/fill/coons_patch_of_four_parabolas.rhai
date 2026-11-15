// The solid under the bilinearly blended Coons patch of four parabolic arcs over the square
// (0, 0) to (10, 10).
//
// Refused: popkern fills only a planar wire (sheet_fill); it has no Coons patch, nor any
// surface through four boundary curves that are not ruled.
throw "popkern has no Coons fill: sheet_fill fills a planar wire only";
