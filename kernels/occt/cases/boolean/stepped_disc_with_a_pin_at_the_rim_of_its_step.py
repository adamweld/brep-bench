"""A stepped disc with a pin through its step, tangent to the step's top rim: a disc of radius 10
from z = 0 to z = 2, a frustum from radius 6 at z = 2 to radius 4 at z = 6, and a cylinder of
radius 1 from z = 1 to z = 9 about the vertical line through (3, 0), united."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    z = gp.DZ_s()
    disc = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), z), 10, 2).Shape()
    step = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 2), z), 6, 4, 4).Shape()
    pin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(3, 0, 1), z), 1, 8).Shape()
    body = BRepAlgoAPI_Fuse(disc, step)
    if not body.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the disc and the step")
    fuse = BRepAlgoAPI_Fuse(body.Shape(), pin)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the body and the pin")
    fuse.SimplifyResult()
    return fuse.Shape()
