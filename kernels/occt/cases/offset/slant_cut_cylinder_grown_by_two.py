"""A cylinder cut by a plane at 45° and grown by 2. A cylinder of radius 3 about the z axis, from its flat bottom at z = −10 up to where the plane
z = x·tan 45° cuts it: the plane crosses the axis at z = 0 and leans at 45° to the bottom, so the
top is the ellipse it cuts from the cylinder, with semi-axes 3 and 3/cos 45°.

Grow it by 2 with rounded edges: the solid of every point within 2 of it. It is convex, so
each face moves out by 2 along its normal, the bottom circle's edge becomes a piece of a torus,
the elliptic edge where the slanted top meets the side becomes a canal surface (the envelope of the
balls of radius 2 along the ellipse), and the faces stay tangent to one another.

By Steiner's formula for a convex body K grown by d, its volume is V₀ + S₀d + Md² + 4πd³/3 and its
area S₀ + 2Md + 4πd², where V₀ and S₀ are K's volume and area and M is its integrated mean
curvature: the side's 10π (its area 60π times the mean curvature 1/6, the plane's top crossing the
axis at z = 0), plus half of each edge's turning angle integrated along it, 3π²/2 for the bottom
circle (a right angle along 6π) and J/2 for the ellipse, where the angle between the top's normal
and the side's at the point (3 cos φ, 3 sin φ, 3 cos φ·tan α) is arccos(−sin α cos φ) and the
ellipse's length element is 3√(1 + tan²α sin²φ) dφ. J has no elementary closed form; the value
above is it to twelve places (any quadrature agrees to well under the grade)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused
from offset import offset

TILT = 45
GROWTH = 2.0


def build():
    t = math.tan(math.radians(TILT))
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp_Dir(0, 0, 1)), 3, 30).Shape()
    # The plane's half-space below it: a long prism on its trace in the plane y = -10.
    trace = BRepBuilderAPI_MakePolygon(gp_Pnt(-20, -10, -20 * t), gp_Pnt(20, -10, 20 * t),
                                       gp_Pnt(20, -10, -40), gp_Pnt(-20, -10, -40), True)
    below = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(trace.Wire()).Face(), gp_Vec(0, 20, 0))
    cut = BRepAlgoAPI_Common(cylinder, below.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Common could not cut the cylinder")
    grown = offset(cut.Shape(), GROWTH)
    # OCCT's offset gives this solid back as its closed shell; the solid is the one it bounds.
    if grown.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        grown = BRepBuilderAPI_MakeSolid(TopoDS.Shell(grown)).Solid()
    return grown
