// A plate punched by a coil spring pushed aslant through it: the tool is itself a swept solid (a
// ball screwed three turns about the z axis), and the hole is everything that spring passes through
// as it slides along (3, 1, 18).
//
// Refused: geomsolver sweeps no swept solid. Written as it reads, `spring := solid({ p | some t in
// (0deg, 1080deg): p in ball at(under: coil) t })` and then `hole := solid({ p | some t in (0deg,
// 360deg): p in spring at(under: ram) t })`, the document is refused at `hole` (E080: "a
// continuous sweep cannot be used as a static source; nested continuous sweeps are not yet
// supported"), so it is neither built nor measured. Solvent has no motion of two rolls (the ball
// screwed and slid at once) to say it as one sweep of the ball, and the spring's shadow along the
// push is an envelope an implementation would have to compute itself.
