+++
title = "A fuselage of ruled sections with two wings of ruled sections fused to it"
tags = ["loft", "ruled", "fuse", "ellipse", "freeform"]
origin = "OCCT tests/bugs/modalg_7/bug22750"
settled = false
+++

**The fuselage** is a ruled loft, straight between neighbours, through five circles centred on
the x axis, each in a plane square to it: at x = 0, 5, 10, 15 and 20 with radii 2, 4, 5, 4 and 2,
closed at its two ends by flat discs. (Four cone frusta stacked.)

**A wing** is a ruled loft through three ellipses, each in a plane y = const, closed at its two
ends by flat ellipse faces. The ellipse in y = 0 has centre (10, 0, 0), semi-axis 4 along x and
0.8 along z; the one in y = 7 has centre (12, 7, 0), semi-axes 3 along x and 0.6 along z; and the
one in y = 14 has centre (15, 14, 0), semi-axes 2 along x and 0.4 along z. Between neighbours the
loft is the ruled surface joining points of equal ellipse parameter, measured from the +x end
of each ellipse's x axis, counterclockwise seen from +y. It leaves the fuselage's side where its
root section, at y = 0, lies inside the fuselage, which there has radius 5 to 4.

**The solid:** the fuselage fused with the wing, and with a second wing, the first turned 120°
about the x axis. The three bodies cross where the wings come out through the fuselage's side.

The creases where the wings leave the fuselage have no closed form, so the case is unsettled until
two kernels agree.
