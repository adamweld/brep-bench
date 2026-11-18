// A rod threaded by a wheel shaped as a flattened spheroid (semi-axes 6, 6 and 1.5), carried along
// a screw.
//
// Refused: Solvent turns only lines, arcs and circles into a solid of revolution. The wheel's
// meridian is half an ellipse; drawn as a traced `std.shapes.Ellipse`, or exactly as two rational
// cubic quarters, `std.solids.Revolution` of it is refused when the screw's sweep reads the wheel
// ("exact revolved patches currently require lines, arcs or circles"), and std.sets has no
// ellipsoid or other set that is the spheroid, so the wheel cannot be made.
