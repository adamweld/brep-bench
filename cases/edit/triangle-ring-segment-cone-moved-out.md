+++
title = "A triangular ring segment, one of its cones moved out"
tags = ["edit", "move-face", "cone", "revolved", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/M4"

[volume]
value = 290.03008845015775
exact = "(2π/3)·A·ρ̄ = (2π/3)·(289/12)·(23/4) = 6647π/72 (Pappus): the new triangle's area A and the distance ρ̄ of its centroid from the axis, turned 2π/3"
tex = '\tfrac{2\pi}{3}\cdot\tfrac{289}{12}\cdot\tfrac{23}{4} = \tfrac{6647\pi}{72}'

[area]
value = 321.1361616785798
exact = "(2π/3)·(391/8) + (2π/3)·(85/12)·(29/8 + 63/8) + 2·(289/12) = 782π/9 + 289/6: the bottom, a sector of the annulus between radii 3/2 and 10; the two cones, each a sector of 2π/3 of a cone whose slant edge is 85/12 long, by Pappus (edge length times the distance of its middle from the axis); and the two flat triangles"
tex = '\tfrac{2\pi}{3}\cdot\tfrac{391}{8} + \tfrac{2\pi}{3}\cdot\tfrac{85}{12}\left(\tfrac{29}{8} + \tfrac{63}{8}\right) + 2\cdot\tfrac{289}{12} = \tfrac{782\pi}{9} + \tfrac{289}{6}'
+++

A ring segment of triangular section: the triangle with corners (4, 0, 0), (10, 0, 0) and (7, 0, 4),
in the half-plane y = 0, x ≥ 0, turned about the z axis through a third of a turn (120°), from
φ = 0° to φ = 120°, where φ is the angle about the z axis from +x toward +y. Its faces are a flat
bottom in the plane z = 0, a sector of the annulus between radii 4 and 10; two conical faces, the
inner one swept by the side from (4, 0, 0) to (7, 0, 4) and the outer one swept by the side from
(10, 0, 0) to (7, 0, 4); and two flat triangles, in φ = 0° and φ = 120°.

Then edit the solid directly: move the inner cone 2 outward along its outward normal, which points
up and in. The cone moves to the cone coaxial with it and parallel to it, 2 away: the cone through
the circle of radius 3/2 in the plane z = 0 and (23/4, 17/3) in a half-plane through the axis. The
faces beside it follow: the bottom runs on in to radius 3/2, the outer cone is cut to meet the
moved cone, at the height 17/3 and radius 23/4, and the two flat triangles are the section of the
new solid by their half-planes, the triangle with corners (3/2, 0), (10, 0) and (23/4, 17/3) in
radius and height.

So the result is the ring segment of triangular section whose triangle has those corners, the same
third of a turn.

**As a set:**

$$\{\, (\rho,\ z) \in T,\ 0^\circ \le \varphi \le 120^\circ \,\},\quad T = \text{the triangle } (\tfrac{3}{2}, 0),\ (10, 0),\ (\tfrac{23}{4}, \tfrac{17}{3})$$
