"""A cube united with a cone frustum as tall as it, inside it. A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum inside it on the z axis: base radius 2 at z = 0, top radius 1 at z = 6, so its base disc lies in the cube's bottom face and its top disc in the cube's top face. Make their union. The frustum adds nothing: its wall is inside the cube and touches nothing, and its two end discs lie in the cube's end faces. What is left is the cube, whole. (A frustum that lay wholly inside the cube without touching its faces would likewise leave it unchanged.)"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-3, -3, 0), gp_Pnt(3, 3, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 2, 1, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
