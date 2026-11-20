"""A cube united with a cone frustum standing on its top. A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum standing on its top face: axis the z axis, base radius 2 at z = 6, top radius 1 at z = 9. Make their union. The two touch only over the frustum's base disc, which lies wholly inside the cube's top face; nothing overlaps. The result is the cube with a cone frustum standing on it."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-3, -3, 0), gp_Pnt(3, 3, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 6), gp.DZ_s()), 2, 1, 3).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
