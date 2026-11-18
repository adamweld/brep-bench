// A helicon face gear hobbed by its worm: a ring blank less everything the worm's thread (a
// right-hand thread of lead 2 pi about the line through (0, 30, 10) along x, its section a row of
// trapezoids, cut off at x = 0 and x = 52) passes through as the worm turns once a radian of the
// roll and the gear 1/30 of it, seen from the gear, over thirty turns of the worm.
//
// Refused: the tool here is itself a swept solid, and geomsolver sweeps no swept solid. The worm's
// thread is a helicoid, which Solvent states only as a sweep: its trapezoidal section carried up a
// screw about the worm's axis, as a face family over a declared parameter (or a face read at a roll
// under the screw). Hobbing then moves that body again, `solid({ p | some t in (0deg, 10800deg): p
// in thread at(under: worm_turn, seen_from: gear_turn) t })`, and a body made by a sweep or a face
// family, even clipped by a Boolean, is no source for another sweep (E080: "material fields
// currently require prisms, revolutions, Boolean bodies or named motions"; for a swept body,
// "nested continuous sweeps are not yet supported"). Nor has Solvent a set for the thread written
// in its own coordinates (x - theta reduced mod 2 pi), a set body bounding no expression in a
// point's coordinates.
