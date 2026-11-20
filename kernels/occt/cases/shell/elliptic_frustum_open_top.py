"""An elliptic cone frustum shelled with its narrow top open. The frustum made by a ruled loft
between the ellipse x²/36 + y²/16 = 1 at z = 0 and the ellipse x²/9 + y²/4 = 1 at z = 8, closed by
the two elliptic disks, shelled inward to a wall 1 thick, open at its top face, z = 8."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from shell import shell_to_plane

HEIGHT = 8
THICKNESS = 1


def ellipse(z, major, minor):
    edge = BRepBuilderAPI_MakeEdge(gp_Elips(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), major, minor))
    return BRepBuilderAPI_MakeWire(edge.Edge()).Wire()


def frustum(top):
    """The loft up to z = `top`, the ellipses' axes running linearly from 6, 4 at z = 0 to 3, 2 at
    z = 8 and on past it."""
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(ellipse(0, 6, 4))
    loft.AddWire(ellipse(top, 6 - 3 * top / 8, 4 - top / 4))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the frustum")
    return loft.Shape()


def build():
    # the cavity is the frustum continued to z = 10, shrunk by 1 and cut off at z = 8
    return shell_to_plane(frustum(HEIGHT), frustum(10), THICKNESS, HEIGHT)
