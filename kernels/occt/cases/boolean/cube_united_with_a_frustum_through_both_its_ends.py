"""A cube united with a taller cone frustum passing out through its top and bottom. A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum on the z axis from z = −3 to z = 9, its radius 2 − z/6 at height z: 2.5 at z = −3, 2 at the cube's bottom face, 1 at its top face, 0.5 at z = 9. Make their union. The frustum is narrower than the cube everywhere it is inside it, so it crosses the bottom face and the top face in circles (radii 2 and 1) and its wall is clear of the cube's sides. What is left is the cube with a cone frustum running through it, three of its length standing out below and three above."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-3, -3, 0), gp_Pnt(3, 3, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 2.5, 0.5, 12).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
