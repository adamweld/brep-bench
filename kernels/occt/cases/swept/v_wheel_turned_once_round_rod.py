"""A rod grooved by a leaning V wheel turned once round it. A rod, the cylinder of radius 5 about
the z axis from z = 0 to z = 20, grooved all round by a wheel with a sharp V rim turned once round
it, as a rod turns in a lathe against a form tool.

The wheel's axis is the line through (14, 0, 10) along the direction (0, sin 20°, cos 20°), leaning
20° from upright across the turn. The wheel is the solid of revolution, about that axis, of the
triangle in the plane through the axis and the point (4, 0, 10) whose corners are (4, 0, 10), and
(6.5, 0, 10) ± 2.5 tan 30°·(0, sin 20°, cos 20°). So its rim is a sharp circle of radius 10 that
dips to (4, 0, 10), 1 inside the rod, its flanks leaning 30° either side of its mid-plane (a 60°
V), back to its bore of radius 7.5.

It turns rigidly a whole turn about the z axis, starting and ending where it is drawn, inside the
rod. Cut from the rod everything the wheel passes through.

A whole turn sweeps a solid of revolution: the groove is the wheel's shadow on a half-plane through
the z axis, turned once round. Leaning, the wheel's flanks touch along curves that are none of its
circles, joined round the rim's fan."""
from bench import Refused


def build():
    # Turned whole about the z axis, the wheel sweeps a solid of revolution, its shadow on a
    # half-plane through the axis turned once round. But the wheel's axis runs past the turn's
    # without meeting it: its flanks, cones, touch what they sweep along curves that are none
    # of their circles or rulings and no plane sections of them, and OCCT has no operation that
    # finds them, nor one that draws a solid's shadow on a half-plane about an axis. A face it
    # turns sweeps to a solid only once split along its contact: finding those curves would be
    # computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it turns faces, and the wheel's flanks, whose "
                  "axis runs past the turn's, touch what they sweep along curves that are no "
                  "plane sections of them, which occt has no operation to find")
