"""A square bar carried round on a planet gear. The solid a square bar fills as it is carried round
on a planet, turning about its own center against the carrier as a planet gear does.

The bar is a prism standing from z = 0 to z = 2, its section the square of side 2 centered at (5,
0) with its sides along x and y.

It moves with one parameter t, from 0° to 360°: its center is carried round the circle of radius 5
about the z axis, to (5 cos t, 5 sin t), and the bar turns about its own center by −2t, so that it
turns by −3t relative to the carrier. Make the solid of everything the bar passes through.

Its section sweeps a region of the plane bounded by envelopes of its sides and the trochoids its
corners trace; the solid is that region carried from z = 0 to z = 2."""
from bench import Refused


def build():
    # The bar turns about its own centre by −2t while its centre is carried round the z axis
    # by t: its axis of turning moves. OCCT's sweeps slide (BRepPrimAPI_MakePrism), turn about
    # one axis (BRepPrimAPI_MakeRevol) or carry a shape along a path under a law of its
    # trihedron (BRepOffsetAPI_MakePipe, MakePipeShell), and along the circle the centre runs
    # on the bar turns against that circle's own frame. And its flat sides touch what they
    # sweep along lines that move across them: the walls are envelopes of the sides and
    # trochoids of the corners, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid along a planet's motion: it slides, turns about "
                  "one axis, or follows a path with its tangent, and the bar turns against the "
                  "circle its centre runs on; the walls are envelopes of its sides, which occt "
                  "has no operation to find")
