// A block from the origin to (20, 10, 10), the edge where its top meets x = 20 beveled 3 back on
// each face; then the bevel deleted, the top and the side run on until they meet.
//
// Refused: popkern has no direct edit that deletes a face and heals the gap. Its remove_faces
// takes faces away and leaves the body open, a sheet, as thicken takes it; nothing runs the
// neighbouring faces on along their surfaces until they meet.
throw "popkern has no delete-face edit that heals the gap: remove_faces leaves the body open, \
a sheet";
