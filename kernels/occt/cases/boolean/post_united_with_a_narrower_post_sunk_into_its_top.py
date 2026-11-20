"""A post united with a narrower post sunk partway into its top and standing out of it. A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A second cylinder of radius 3 with the same axis, from z = 6 to z = 14: it is sunk 4 into the post and stands 4 out of its top. Make the union of the two. The narrow cylinder's wall crosses the post's top face square, along the circle ρ = 3, z = 10. Below it the narrow cylinder is inside the post and goes; its base, at z = 6, is inside the post too."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    wide = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    narrow = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 6), gp.DZ_s()), 3, 8).Shape()
    fuse = BRepAlgoAPI_Fuse(wide, narrow)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
