"""A prism with a curved spline wall, its top flat moved in past the wall's middle. A prism of length 10 standing on the xy plane, from z = 0 to z = 10, whose section is the region in
the plane z = 0 bounded by three edges: a cubic Bézier curve (one span of a B-spline of degree 3,
knots 0 and 1 each repeated four times) from (0, 0) to (4, 4) with control points (0, 0),
(4/3, 0), (8/3, 4/3) and (4, 4), which is the parabola y = x²/4, x from 0 to 4; a straight edge
from (4, 4) to (0, 4); and a straight edge from (0, 4) to (0, 0). Its faces are the curved wall,
which is the cylinder over the parabola; the flat top, in the plane y = 4, whose outward normal is
+y; the flat side in the plane x = 0; and the two ends, z = 0 and z = 10.

Then edit the solid directly: move the top flat 3 inward, against its outward normal, to the plane
y = 1. The faces beside it are trimmed: the curved wall is cut off where its parabola reaches
y = 1, at x = 2, and the flat side in x = 0 is cut down to y ≤ 1. The solid is left with the
curved wall from (0, 0) to (2, 1), the new top in y = 1 from x = 2 back to x = 0, and the flat
side from (0, 1) down to (0, 0); the right-hand end of the old section, at (4, 4), is gone."""
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
    solid = prism(CUBIC, (0, 0, 10))
    top = face_through(solid, (2, 4, 5))  # y = 4: its outward normal is +y
    return moved(solid, [(top, -3)])
