// A rounded block's top moved up, its round carried with it.
//
// Refused: popkern's offset_faces moves a face whose neighbours stand square to it, so they run on
// along their own surfaces; the round leans onto the top, and carrying a blend with the face it
// joins (the round kept, not stretched) is not a direct edit popkern has.
throw "popkern moves a face whose neighbours stand square to it; carrying a round with the face it blends is not an edit popkern has";
