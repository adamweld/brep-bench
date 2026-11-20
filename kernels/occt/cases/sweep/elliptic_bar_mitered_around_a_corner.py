"""An elliptic bar mitred round a right-angled corner. A bar of elliptic section swept along a path
with one right-angled corner, mitred there.

The path: two straight runs, from (0, 0, 0) along +x to (40, 0, 0), then turning a right angle to
the left (toward +y) and on 40 to (40, 40, 0).

The section: an ellipse square to the path and centred on it, with semi-axis 6 along z and
semi-axis 4 across the run, in the xy plane.

Each run is the ellipse swept straight along it, and at the corner the two are mitred: each is
cut by, and they meet in, the vertical plane through the corner that bisects it, x + y = 40. The
bar's ends are flat and square to the path: in the plane x = 0, and in the plane y = 40."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from corner import MITRE, polyline, swept

RUN = 40


def build():
    path = polyline([(0, 0, 0), (RUN, 0, 0), (RUN, RUN, 0)])
    # The ellipse at the path's start, in the plane x = 0: major axis 6 along z, minor 4 along y.
    ellipse = BRepBuilderAPI_MakeEdge(
        gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0), gp_Dir(0, 0, 1)), 6, 4)).Edge()
    return swept(path, BRepBuilderAPI_MakeWire(ellipse).Wire(), MITRE, binormal=gp_Dir(0, 0, 1))
