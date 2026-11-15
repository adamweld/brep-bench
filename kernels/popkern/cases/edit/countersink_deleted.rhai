// A block from the origin to (20, 10, 10) drilled through by a hole of radius 2 on (10, 5),
// countersunk by the cone from radius 2 at z = 8 to 4 at z = 10; then the countersink deleted.
//
// Refused: popkern has no direct edit that deletes a face and heals the gap. Its remove_faces
// takes faces away and leaves the body open, a sheet, as thicken takes it; nothing runs the
// neighbouring faces on along their surfaces until they meet.
throw "popkern has no delete-face edit that heals the gap: remove_faces leaves the body open, \
a sheet";
