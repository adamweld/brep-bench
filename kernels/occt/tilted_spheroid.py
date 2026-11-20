"""What the OCCT implementations of the box-and-tilted-spheroid cases share: the block, the oblate
spheroid of semi-axes 4 and 2 centred on the origin with its short axis the z axis tilted 30° about
the x axis toward +y, and the Boolean of the two."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfEllipse
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt, gp_Trsf

from bench import Refused

EQUATOR, POLE = 4, 2
TILT = math.radians(30)


def block():
    return BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, -6), gp_Pnt(10, 10, 0)).Shape()


def spheroid():
    """The ellipse x²/16 + z²/4 ≤ 1 of the xz plane turned about the z axis, the whole then turned
    about the x axis so that +z goes to (0, sin 30°, cos 30°)."""
    frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0))  # its x along +x
    ellipse = gp_Elips(frame, EQUATOR, POLE)
    south, north = gp_Pnt(0, 0, -POLE), gp_Pnt(0, 0, POLE)
    arc = GC_MakeArcOfEllipse(ellipse, south, north, True).Value()
    if arc.Value(0.5 * (arc.FirstParameter() + arc.LastParameter())).X() < 0:
        arc = GC_MakeArcOfEllipse(ellipse, north, south, True).Value()
    half = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(north, south).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(half, True).Face()
    upright = BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), -TILT)  # +z goes to (0, sin, cos)
    return BRepBuilderAPI_Transform(upright, turn, True).Shape()


def boolean(kind):
    """The block and the spheroid, "fuse", "cut" (block less spheroid) or "common"."""
    op = {"fuse": BRepAlgoAPI_Fuse, "cut": BRepAlgoAPI_Cut, "common": BRepAlgoAPI_Common}[kind]
    result = op(block(), spheroid())
    if not result.IsDone():
        raise Refused(f"BRepAlgoAPI {kind} of the block and the tilted spheroid declined")
    return result.Shape()
