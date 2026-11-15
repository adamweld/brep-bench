// A cylinder of radius 6 on z from 0 to 10, its top rim rounded to 2; then the round deleted.
//
// Refused: popkern has no direct edit that deletes a face and heals the gap. Its remove_faces
// takes faces away and leaves the body open, a sheet, as thicken takes it; nothing runs the
// neighbouring faces on along their surfaces until they meet.
throw "popkern has no delete-face edit that heals the gap: remove_faces leaves the body open, \
a sheet";
