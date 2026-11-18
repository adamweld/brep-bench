+++
title = "A square bar carried round on a planet gear"
tags = ["swept", "solid-sweep", "planar", "general-motion", "prism", "trochoid", "envelope"]
origin = "geomsolver #721, box K3 (nested relative motions)"

[volume]
value = 155.3366775343
exact = "no closed form: cad-kernel, next.BREP and cadrs agree to 5e-9; the mean of their values"

[area]
value = 282.1927120073
exact = "no closed form: cad-kernel, next.BREP and cadrs agree to 4e-10; the mean of their values"
+++

The solid a square bar fills as it is carried round on a planet, turning about its own center
against the carrier as a planet gear does.

The bar is a prism standing from z = 0 to z = 2, its section the square of side 2 centered at (5, 0)
with its sides along x and y.

It moves with one parameter t, from 0° to 360°: its center is carried round the circle of radius 5
about the z axis, to (5 cos t, 5 sin t), and the bar turns about its own center by −2t, so that it
turns by −3t relative to the carrier. Make the solid of everything the bar passes through.

Its section sweeps a region of the plane bounded by envelopes of its sides and the trochoids its
corners trace; the solid is that region carried from z = 0 to z = 2.
