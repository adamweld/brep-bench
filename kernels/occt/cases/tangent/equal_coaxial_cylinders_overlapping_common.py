"""Two equal cylinders overlapping end to end, their common part. Two cylinders of radius 5 on the
z axis, the first from z = 0 to z = 10 and the second from z = 6 to z = 16. Make the part common to
both. If the cylinders have a seam, turn the second a quarter turn about the z axis."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(5, 10).Shape()
    # OCCT's cylinder has its seam along its frame's x direction: this one's runs along y
    turned = gp_Ax2(gp_Pnt(0, 0, 6), gp.DZ_s(), gp.DY_s())
    second = BRepPrimAPI_MakeCylinder(turned, 5, 10).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two cylinders")
    return common.Shape()
