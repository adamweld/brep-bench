"""Two cylinders a few microns across, crossed. Two cylinders of radius 0.003, one along the x axis
from x = −0.01 to 0.01, one along the y axis from y = −0.01 to 0.01. Make the solid common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

RADIUS, HALF_LENGTH = 0.003, 0.01


def build():
    along_x = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-HALF_LENGTH, 0, 0), gp.DX_s()), RADIUS,
                                       2 * HALF_LENGTH).Shape()
    along_y = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -HALF_LENGTH, 0), gp.DY_s()), RADIUS,
                                       2 * HALF_LENGTH).Shape()
    common = BRepAlgoAPI_Common(along_x, along_y)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two cylinders")
    return common.Shape()
