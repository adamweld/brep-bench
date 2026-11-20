"""An elliptic cone frustum shelled with its narrow top open. The frustum made by a ruled loft
between the ellipse x²/36 + y²/16 = 1 at z = 0 and the ellipse x²/9 + y²/4 = 1 at z = 8, closed by
the two elliptic disks, shelled inward to a wall 1 thick, open at its top face, z = 8."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from shell import face_through, shell

HEIGHT = 8
THICKNESS = 1


def ellipse(z, major, minor):
    edge = BRepBuilderAPI_MakeEdge(gp_Elips(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), major, minor))
    return BRepBuilderAPI_MakeWire(edge.Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(ellipse(0, 6, 4))
    loft.AddWire(ellipse(HEIGHT, 3, 2))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the frustum")
    frustum = loft.Shape()
    return shell(frustum, [face_through(frustum, (0, 0, HEIGHT))], THICKNESS)
