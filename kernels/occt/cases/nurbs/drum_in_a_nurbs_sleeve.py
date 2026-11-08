"""A drum united with a sleeve whose bore is a NURBS cylinder. A drum and a sleeve around it,
united. The sleeve's bore is the drum's own cylinder, but written as a NURBS surface, so the union
must recognise a B-spline face lying exactly on a cylinder.

The drum is the solid cylinder of radius 5 about the z axis from z = 0 to z = 10, its side a
circular cylinder.

The sleeve is everything between 5 and 7 from the z axis with 2 ≤ z ≤ 8. Its outer side is the
circular cylinder of radius 7 and its ends are flat annuli. Its bore, the side of radius 5, is a
rational B-spline surface of degree 2 around the axis and 1 along it: around the axis, the knot
vector (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1) and the nine control points, in (x, y), (0, 5), (−5,
5), (−5, 0), (−5, −5), (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights 1, √2/2, 1, √2/2, 1,
√2/2, 1, √2/2, 1; along the axis, the knot vector (0, 0, 1, 1) and the heights z = 2 and z = 8. So
it is exactly the cylinder of radius 5, its seam on the line through (0, 5).

The union is the drum with a ring 2 thick around its middle."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_MakeWire,
                                BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.Geom import Geom_BSplineSurface, Geom_CylindricalSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array1_double, Array1_int, Array2_double, Array2_gp_Pnt
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Pnt

from bench import Refused

DRUM_RADIUS, DRUM_HEIGHT = 5, 10
BORE, OUTER = 5, 7  # the sleeve's radii
LOW, HIGH = 2, 8  # its ends
H = math.sqrt(2) / 2
# the bore's nine control points round the axis, in (x, y), and their weights
CIRCLE = [(0, 5), (-5, 5), (-5, 0), (-5, -5), (0, -5), (5, -5), (5, 0), (5, 5), (0, 5)]
WEIGHTS = [1, H, 1, H, 1, H, 1, H, 1]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def bore():
    """The sleeve's bore: degree 2 round the axis on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1,
    1), degree 1 along it from z = 2 to z = 8, each column weighted as its circle pole."""
    poles = Array2_gp_Pnt(1, len(CIRCLE), 1, 2)
    weights = Array2_double(1, len(CIRCLE), 1, 2)
    for i, ((x, y), w) in enumerate(zip(CIRCLE, WEIGHTS)):
        for j, z in enumerate((LOW, HIGH)):
            poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, z))
            weights.SetValue(i + 1, j + 1, w)
    surface = Geom_BSplineSurface(poles, weights,
                                  array(Array1_double, [0, 0.25, 0.5, 0.75, 1]),
                                  array(Array1_double, [0, 1]),
                                  array(Array1_int, [3, 2, 2, 2, 3]), array(Array1_int, [2, 2]),
                                  2, 1)
    return BRepBuilderAPI_MakeFace(surface, 1e-9).Face()


def circle(radius, z):
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(
        gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp.DZ_s()), radius)).Edge()).Wire()


def sleeve():
    """The sleeve's four faces sewn into a shell and closed into a solid: the NURBS bore, the
    circular cylinder of radius 7 and the flat annuli at z = 2 and z = 8."""
    outer = BRepBuilderAPI_MakeFace(Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), OUTER),
                                    0, 2 * math.pi, LOW, HIGH, 1e-9).Face()
    sewing = BRepBuilderAPI_Sewing(1e-7)
    sewing.Add(bore())
    sewing.Add(outer)
    for z in (LOW, HIGH):
        annulus = BRepBuilderAPI_MakeFace(circle(OUTER, z))
        annulus.Add(TopoDS.Wire(circle(BORE, z).Reversed()))
        sewing.Add(annulus.Face())
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the sleeve's faces")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined the sleeve's sewn shell")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    drum = BRepPrimAPI_MakeCylinder(gp.XOY_s(), DRUM_RADIUS, DRUM_HEIGHT).Shape()
    # the union must find that the bore lies on the drum's side
    fuse = BRepAlgoAPI_Fuse(drum, sleeve())
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the drum and the sleeve")
    return fuse.Shape()
