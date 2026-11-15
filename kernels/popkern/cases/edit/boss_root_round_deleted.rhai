// A plate (radius 8, z = -4 to 0) with a boss (radius 3, z = 0 to 8), the boss's root rounded to
// 1; then the round deleted.
//
// Refused: popkern has no direct edit that deletes a face and heals the gap. Its remove_faces
// takes faces away and leaves the body open, a sheet, as thicken takes it; nothing runs the
// neighbouring faces on along their surfaces until they meet.
throw "popkern has no delete-face edit that heals the gap: remove_faces leaves the body open, \
a sheet";
