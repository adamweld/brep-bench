"""A cylinder united with a narrower cylinder crossing it at right angles through its axis. A cylinder of radius 5 and length 14 along the z axis, from z = −7 to z = 7, and a cylinder of radius 3 and length 16 along the x axis, from x = −8 to x = 8: the narrow one crosses the wide one at right angles, its axis meeting the wide one's axis at the origin, and passes right through it, out of both sides. Make their union. The narrow cylinder's wall lies wholly within the wide cylinder's length, its two end discs are clear of the wide cylinder, and the wide one's end discs, at z = ±7, are clear of the narrow one.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -7), gp.DZ_s()), 5, 14).Shape()
    narrow = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-8, 0, 0), gp.DX_s()), 3, 16).Shape()
    fuse = BRepAlgoAPI_Fuse(wide, narrow)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
