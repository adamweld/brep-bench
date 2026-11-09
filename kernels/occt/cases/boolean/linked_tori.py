"""Two disjoint but linked tori, both of major radius 3 and tube radius 0.5: one at the origin
about z, one at (3, 0, 0) about y. Their union keeps two separate solids."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 3, 0.5).Shape()
    second = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(3, 0, 0), gp_Dir(0, 1, 0)), 3, 0.5).Shape()
    union = BRepAlgoAPI_Fuse(first, second)
    if not union.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two linked rings")
    return union.Shape()
