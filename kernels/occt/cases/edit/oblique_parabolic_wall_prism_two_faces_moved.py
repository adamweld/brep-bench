"""An oblique prism with a curved spline wall, its sloping top moved in and its side flat moved out along the wall. An oblique prism: the region in the plane z = 0 bounded by three edges, swept along the vector
V = (0, 6, 8), so that its far end, in the plane z = 8, is the same region shifted 6 up in y. The
region is bounded by a Bézier curve of degree 4 (one span of a B-spline of degree 4, knots 0 and 1
each repeated five times) from (0, 0) to (4, 4) with control points (0, 0), (1, 0), (2, 2/3), (3, 2)
and (4, 4), which is the parabola y = x²/4, x from 0 to 4; a straight edge from (4, 4) to (0, 4);
and a straight edge from (0, 4) to (0, 0). Its faces are the curved wall; the sloping top, the
plane through the edge y = 4, z = 0 and the direction V, that is y − 3z/4 = 4, whose outward normal
is (0, 4, −3)/5; the flat side in the plane x = 0, which contains V and whose outward normal is
−x; and the two ends, z = 0 and z = 8.

Then edit the solid directly, moving two faces at once: the sloping top 12/5 inward, against its
outward normal, which moves its plane to y − 3z/4 = 1 (a normal distance of 12/5 is a shift of 3
along y), and the side flat 1 outward, to the plane x = −1. The curved wall is cut off by the new
top, where its parabola reaches y − 3z/4 = 1, at x = 2 on every level, and runs on past (0, 0) along
its own parabola to meet the new side, at x = −1."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused, face_through
from edit import moved

CUBIC = [(0, 0), (4 / 3, 0), (8 / 3, 4 / 3), (4, 4)]
QUARTIC = [(0, 0), (1, 0), (2, 2 / 3), (3, 2), (4, 4)]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def prism(poles, vector):
    """The region bounded by the Bezier curve through `poles`, then the straight edges from its end
    to (0, 4) and from (0, 4) to (0, 0), swept along `vector`."""
    n = len(poles)
    curve = Geom_BSplineCurve(array(Array1_gp_Pnt, [gp_Pnt(x, y, 0) for x, y in poles]),
                              array(Array1_double, [0.0, 1.0]), array(Array1_int, [n, n]), n - 1)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(curve).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(4, 4, 0), gp_Pnt(0, 4, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 4, 0), gp_Pnt(0, 0, 0)).Edge())
    if not wire.IsDone():
        raise Refused("the three edges did not make a wire")
    section = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not section.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace could not fill the section")
    return BRepPrimAPI_MakePrism(section.Face(), gp_Vec(*vector)).Shape()


def build():
    solid = prism(QUARTIC, (0, 6, 8))
    top = face_through(solid, (2, 7, 4))   # y - 3z/4 = 4: its outward normal is (0, 4, -3)/5
    side = face_through(solid, (0, 5, 4))  # x = 0: its outward normal is -x
    return moved(solid, [(top, -12 / 5), (side, 1)])
