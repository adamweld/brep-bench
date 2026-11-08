"""A cube tumbled through space. The solid a cube fills as it tumbles: it turns about one axis
while being watched from a frame that turns about another.

The cube is the box from (2.4, −0.6, 1.4) to (3.6, 0.6, 2.6): edge 1.2, centered at (3, 0, 2).

It moves by two turns at once, both driven by one parameter t:

- it turns by t/4 about the vertical line through (2, 0, 0), right-handed (from +x toward +y);
- the frame it is seen from turns by t about the line through (0, 0.5, 0.5) parallel to the x axis,
  right-handed (from +y toward +z).

Seen from that frame, the cube's place at t is its place turned by t/4 about the first line and
then by −t about the second. Make the solid of everything the cube passes through, so seen, as t
runs from −75° to 75°.

No turn keeps any line of the cube fixed, and its silhouette changes from face to face as it
tumbles."""
from bench import Refused


def build():
    # The cube moves by a turn about a vertical line followed by a turn about a line along x,
    # both driven by t: its axis of turning changes from instant to instant. OCCT's sweeps slide
    # (BRepPrimAPI_MakePrism), turn about one axis (BRepPrimAPI_MakeRevol) or carry a shape along
    # a path under a law of its trihedron (BRepOffsetAPI_MakePipe, MakePipeShell), and none of
    # their laws turns a shape like this along its centre's path. And its flat faces touch what
    # they sweep along lines that move across them, so no fixed split of them sweeps without
    # folding: the boundary is the envelope of the cube, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid along a pair of turns about skew axes: it "
                  "slides, turns about one axis, or follows a path with a trihedron law, none "
                  "of which is this tumble; the boundary is the envelope of the tumbling cube, "
                  "which occt has no operation to find")
