+++
title = "The union of opposing torus tumbles"
tags = ["swept", "solid-sweep", "tumble", "torus", "boolean", "union", "regression"]
origin = "geomsolver 55e64186 tumbling-torus failure isolation; docs/tumbling-torus-regressions.md"
settled = false
+++

Make two swept solids A and B from the same torus: center (3, 0, 2), axis parallel to z, major
radius 1, tube radius 0.22, including its interior.

The first oriented rotation axis is the vertical line through (2, 0, 0). The second is through q =
(0, 0.7071 cos 45°, 0.7071 sin 45°), in unit direction u = (cos 20°, sin 20° cos 45°, sin 20° sin
45°). Both axes stay fixed in the world.

For A, at roll t first turn the torus by 0.7t about the first axis and then by −t about the second.
For B, first turn it by 0.7t about the first axis and then by +t about the second. Each solid is the
union of all its positions for every real t from −100° to +100°, including the end poses. These are
the solids separately described in
[swept/tumbling-torus-crossed-forward](tumbling-torus-crossed-forward.md) and
[swept/tumbling-torus-crossed-reverse](tumbling-torus-crossed-reverse.md).

The result is the regularized union: the closure of the interior of the set of points in A or in B.
Each sweep quantifies its own roll independently. In particular, this is the union of the two
completed sweeps, **not** a sweep of the instantaneous union at a shared t. Keep every volumetric
component. Count overlaps once and discard internal sheets.

This isolates the interaction between two envelopes from the construction of either operand. The
boundary switches between envelope sheets at their intersections, forming sharp curves and
potentially narrow surviving regions.

No closed-form volume or area is asserted; this case is unsettled.
