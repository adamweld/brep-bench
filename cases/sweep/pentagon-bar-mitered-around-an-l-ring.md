+++
title = "A bar of pentagonal section mitered around an L-shaped ring"
tags = ["sweep", "planar-path", "closed", "corner", "miter", "concave", "analytic"]
origin = "OCCT tests/evolved/voluved/AGT002"

[volume]
value = 4920.0
exact = "A(P + 8s̄): the section's area A = 27, the path's length P = 160, and its centroid's distance outward s̄ = 25/9; the five convex corners each add 2s̄A (a 90° miter, tan 45° = 1) and the one concave corner takes it away: = 4320 + 600"
tex = 'A\left(P + 8\bar{s}\right),\quad A = 27,\ P = 160,\ \bar{s} = \tfrac{25}{9}:\ = 4320 + 600'

[area]
value = 3750.688049990942
exact = "Σ|e|(P + 8sₑ) over the section's five edges e, sₑ the distance outward of each edge's middle: 6·184 + 3·208 + √13·196 + 3·172 + 5·160 = 3044 + 196√13"
tex = '\sum_e |e|\left(P + 8s_e\right) = 6\cdot 184 + 3\cdot 208 + 196\sqrt{13} + 3\cdot 172 + 5\cdot 160 = 3044 + 196\sqrt{13}'
+++

A bar swept around the outside of an L-shaped outline, mitered at each of its six corners, one of
which is concave.

The path: the closed outline in the xy plane through (0, 0, 0), (40, 0, 0), (40, 20, 0),
(20, 20, 0), (20, 40, 0) and (0, 40, 0), and back to the first, run in that order, counterclockwise.
It is an L whose notch, 20 square, is cut from the corner at (40, 40). Five of its corners turn left,
into the L, and one, at (20, 20, 0), turns right.

The section stands square to the path. It is given by s, the distance outward from the path (to its
right as it is run, away from the L), and z: the polygon through (s, z) = (0, 0), (6, 0), (6, 3),
(3, 5), (0, 5). That is a bar 6 wide and 5 tall lying outside the L, its top outer corner cut off.
On the side y = 0, for example, the point (s, z) of the section is at (x, −s, z).

Each side of the outline is the section swept straight along it, and at each corner the two sides are
mitered: each is cut by, and they meet in, the vertical plane through the corner at 45° to both
sides, whose normal is the sum of the two sides' directions. At (0, 0, 0) that is the plane y = x, and
so it is at the concave corner (20, 20, 0), where the two bars, which lie in the notch, meet along
that line within the square from (20, 20) to (26, 26). The path is closed, so the bar has no ends:
it is a ring.
