"""A cylinder united with a cube tilted about a corner below its base. A cylinder of radius 3 and height 6 standing on the xy plane with its axis the z axis, and a cube of edge 4 with one corner at P = (0, 0, −1) on the axis, just below the cylinder's base. Start with the cube's three edges from P running along +x, +y and +z; then turn the cube 30° about the horizontal line through P in the direction (1, 1, 0), right-handed. The line is the diagonal of the cube's lower face through P, and the turn carries the cube's +z edge toward +x and −y, so that the cube's far corners rise through the base disc. Make the union of the cylinder and the cube. The cube straddles the plane z = 0: its faces cut the base disc and the wall, and the cylinder's top, at z = 6, is untouched. Its faces meet the wall in arcs of ellipses, so the case has no closed form and is unsettled."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 3, 6).Shape()
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, -1), gp_Pnt(4, 4, 3)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, -1), gp_Dir(1, 1, 0)), math.radians(30))
    cube = BRepBuilderAPI_Transform(cube, turn, True).Shape()
    fuse = BRepAlgoAPI_Fuse(post, cube)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
