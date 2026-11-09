+++
title = "A Reuleaux tetrahedron: four balls in common"
tags = ["boolean", "common", "analytic", "sphere", "corner"]

[volume]
value = 422.1577331158267
exact = "s³(3√2 + 32π − 81·acos(1/3))/12, s = 10"
tex = '\tfrac{10^3}{12}\left(3\sqrt2 + 32\pi - 81\arccos\tfrac13\right)'

[area]
value = 297.5471716584401
exact = "s²(8π − 18·acos(1/3)), s = 10"
tex = '10^2\left(8\pi - 18\arccos\tfrac13\right)'
+++

Four balls of radius 10, centered at the vertices of a regular tetrahedron of side 10:

- (0, 0, 0);
- (10, 0, 0);
- (5, 5√3, 0);
- (5, 5√3/3, 10√(2/3)).

Make the solid common to all four balls: a Reuleaux tetrahedron. Its boundary consists of four
spherical triangular patches meeting along six circular arcs, with three patches meeting at each
of the four vertices. Keep those arcs; this is not the constant-width Meissner body made by
rounding three of them.

For side s, put θ = acos(1/3), in radians. Each spherical patch has area
s²(2π − 9θ/2), giving the total area above. The closed form for the Reuleaux tetrahedron's
volume is s³(3√2 + 32π − 81θ)/12.
