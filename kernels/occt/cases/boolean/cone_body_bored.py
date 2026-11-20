"""A cone body, bored. A frustum from radius 6 at z = 0 to radius 3 at z = 10, a second from
radius 3 to radius 2 at z = 14 on top of it, a conical pocket with its apex at (0, 0, 7) opening
upward, and a bore of radius 0.5 along the axis."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def ax(z):
    return gp_Ax2(gp_Pnt(0, 0, z), gp.DZ_s())


def build():
    lower = BRepPrimAPI_MakeCone(ax(0), 6, 3, 10).Shape()
    upper = BRepPrimAPI_MakeCone(ax(10), 3, 2, 4).Shape()
    body = BRepAlgoAPI_Fuse(lower, upper)
    if not body.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cones")
    # The pocket's cone opens as rho = (z - 7)/4; it runs on to z = 15, past the top.
    pocket = BRepPrimAPI_MakeCone(ax(7), 0, 2, 8).Shape()
    pocketed = BRepAlgoAPI_Cut(body.Shape(), pocket)
    if not pocketed.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    bore = BRepPrimAPI_MakeCylinder(ax(-1), 0.5, 16).Shape()
    bored = BRepAlgoAPI_Cut(pocketed.Shape(), bore)
    if not bored.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    return bored.Shape()
