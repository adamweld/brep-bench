"""A cube united with a frustum as tall as it, its axis on a side face, its base circle tangent to the two neighbouring faces. A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum as tall as the cube on the vertical axis through (3, 0), the middle of the cube's side face y = 0: base radius 3 at z = 0, top radius 1 at z = 6. Make their union. The frustum's base circle, of radius 3 about (3, 0), touches the planes of the cube's faces x = 0 and x = 6 at (0, 0, 0) and (6, 0, 0), the ends of the face y = 0's bottom edge, and nowhere else; higher up the frustum is narrower. Half of the frustum, the half on y < 0, lies outside the cube, joined to it over the trapezoid the frustum's section makes in the face y = 0; the other half is inside the cube and goes.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(6, 6, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(3, 0, 0), gp.DZ_s()), 3, 1, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
