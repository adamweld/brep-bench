+++
title = "A ring lying flat, slid across less than its hole"
tags = ["swept", "solid-sweep", "slide", "torus", "ring-in-plane", "lens"]
origin = "geomsolver #721, box D2 (a ring kept in a plane the motion keeps)"

[volume]
value = 1031.876234895
exact = "no closed form: next.BREP and cad-kernel agree to 4e-9; the mean of their values"

[area]
value = 748.65781772
exact = "no closed form: next.BREP and cad-kernel agree to 4e-9; the mean of their values"
+++

The solid a ring fills as it slides across, square to its own axis, by less than the width of its
hole.

The ring is a torus about the z axis centered on the origin: the circle through its tube's centers
has radius 6, and its tube has radius 2.

It slides without turning along the x axis by 6, ending centered on (6, 0, 0). Make the solid of
everything the ring passes through.

The torus's top and bottom circles stay in their planes and sweep flat faces. Its hole is never
filled: what is left of it, all the way through, is the lens the hole shares where the ring starts
and where it stops, walled by the inner halves of the torus at those two places, which meet along
two curves.
