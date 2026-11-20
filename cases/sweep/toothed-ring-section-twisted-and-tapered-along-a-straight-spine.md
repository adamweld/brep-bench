+++
title = "A toothed ring section twisted and tapered along a straight spine"
tags = ["sweep", "twisted", "scaled", "helix", "arcs", "unsettled"]
origin = "OCCT bugs/modalg_5/bug24758_1"
settled = false
+++

A gear-like ring section swept along a straight spine, turned a quarter turn and shrunk to three fifths of its size as it goes: a stub of a toothed ring that twists and tapers.

The spine is the z axis from z = 0 to z = 10. The section at z = 0 is a toothed ring with six teeth, in the xy plane about the z axis:

- its bore is the circle of radius 1.5;
- its outer boundary goes round once, counter-clockwise, in twelve pieces, for k = 0 … 5 the tooth centered at azimuth 60°·k: the tip, an arc of the circle of radius 4 from azimuth 60°·k − 12° to 60°·k + 12°; then the straight flank from the end of that arc, (4, 60°·k + 12°), to (3, 60°·k + 18°) (polar coordinates, radius then azimuth); then the root, an arc of the circle of radius 3 from azimuth 60°·k + 18° to 60°·k + 42°; then the straight flank from its end to the start of the next tip, (4, 60°·(k + 1) − 12°).

The ring is the region between the bore and the outer boundary, so a tooth stands 1 proud of its root and is 24° wide at its tip.

The section at height z is the section at z = 0 scaled about the z axis by s(z) = 1 − 0.04·z and turned about the z axis, right-handed (from +x toward +y), by 9°·z. Both laws are linear in z: the section at z = 5 is scaled to 0.8 and turned 45°, and the one at z = 10 to 0.6 and turned 90°. A point of the section at distance ρ from the axis and azimuth φ is therefore at distance ρ·s(z) and azimuth φ + 9°·z at height z. The solid is bounded by the flat section at z = 0, the flat section at z = 10 (scaled to 0.6 and turned 90°), and the surfaces swept by the bore, the twelve tips and roots (each an arc swept into a twisted, tapering surface), and the twelve flanks (each a line swept into a twisted ruled surface).
