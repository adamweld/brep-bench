+++
title = "A quarter-round bar mitered round a rectangle"
tags = ["sweep", "planar-path", "closed", "corner", "miter", "arc", "analytic"]
origin = "OCCT evolved/voluved/AGT008"

[volume]
value = 110.0825932025646
exact = "A(P + 8s̄) with the section's area A = 2 + π/4, the path's length P = 32 and ∫s dA = 11/6 + π/4 (so As̄ = 11/6 + π/4): 32(2 + π/4) + 8(11/6 + π/4) = 64 + 44/3 + 10π"
tex = 'A P + 8\int s\,dA = 32\left(2 + \tfrac{\pi}{4}\right) + 8\left(\tfrac{11}{6} + \tfrac{\pi}{4}\right) = 64 + \tfrac{44}{3} + 10\pi'

[area]
value = 258.8318530717959
exact = "∫(P + 8s) dℓ over the section's boundary, s its distance outward: 2·40 + 0.5·48 + (20π + 8) + 1·36 + 1.5·32 (bottom, outer side, arc with s = 1 + cos θ, top, inner side) = 196 + 20π"
tex = '\oint (P + 8s)\,d\ell = 80 + 24 + (20\pi + 8) + 36 + 48 = 196 + 20\pi'
+++

A bar with one rounded corner swept once round a rectangle, mitered at the four corners: a ring, a rectangle's border with a quarter-round edge.

The path: the rectangle in the xy plane with corners (±5, ±3, 0), 10 × 6, closed.

The section stands square to the path. It is given by s, the distance outward from the path (away from the rectangle's center), and z: the region 0 ≤ s ≤ 2, 0 ≤ z ≤ 1.5 with its corner at (s, z) = (2, 1.5) rounded off by a quarter of a circle of radius 1, centered on (s, z) = (1, 0.5). Its boundary is, in order: the line from (0, 0) to (2, 0); the line up to (2, 0.5); the arc of that circle, convex, from (2, 0.5) through (1 + 1/√2, 0.5 + 1/√2) to (1, 1.5); the line back to (0, 1.5); and the line down to (0, 0). That is a bar 2 wide and 1.5 tall lying outside the path, its outer top edge rounded. On the side y = −3, for example, the point (s, z) of the section is at (x, −3 − s, z).

Each side of the rectangle is the section swept straight along it, and at each corner the two sides are mitered: each is cut by, and they meet in, the vertical plane through the corner along its diagonal, at 45° to both sides. At (5, −3, 0) that is the plane x + y = 2, running from the path's corner out to the ring's outer corner at (7, −5). The path is closed, so the ring has no ends: it is 14 × 10 outside, with a plain rectangular opening 10 × 6.

**As a set:**

$$\{\,(x, y, z) : s = \max(|x| - 5,\ |y| - 3) \ge 0,\ 0 \le s \le 2,\ 0 \le z \le 1.5,\ \neg\,(s > 1 \wedge z > 0.5 \wedge (s - 1)^2 + (z - 0.5)^2 > 1)\,\}$$
