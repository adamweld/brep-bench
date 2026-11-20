"""A post united with a narrower post of its own height, hidden inside it. A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A second cylinder of radius 3 with the same axis and the same ends, from z = 0 to z = 10: it lies inside the post, its two end discs in the post's end faces. Make the union of the two. The narrow cylinder adds nothing; its wall is inside the post and touches nothing, and its end discs lie in the post's end faces. What is left is the post, whole."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    narrow = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 3, 10).Shape()
    fuse = BRepAlgoAPI_Fuse(wide, narrow)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
