"""A cube united with a small cone frustum on its top, its base circle tangent to two top edges. A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum standing on its top face: axis the vertical line through (1.5, 1.5), base radius 1.5 at z = 6, top radius 0.75 at z = 9. Make their union. The base circle, of radius 1.5 about (1.5, 1.5), touches the cube's top edges x = 0 and y = 0 at (0, 1.5) and (1.5, 0): the frustum's base disc lies wholly in the top face and meets its boundary at those two points. Nothing overlaps."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(6, 6, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(1.5, 1.5, 6), gp.DZ_s()), 1.5, 0.75, 3).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
