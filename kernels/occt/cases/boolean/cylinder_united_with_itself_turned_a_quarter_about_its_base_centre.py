"""A cylinder united with itself turned a quarter turn about its base's centre. A cylinder of
radius 2 and height 5, axis along z, base disc in the xy plane centred on the origin. A second
cylinder is the first turned a quarter turn about the y axis through the origin, its axis along
+x. Unite them."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Trsf

from bench import Refused

RADIUS, HEIGHT = 2, 5


def build():
    first = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp.OY_s(), math.radians(90))  # takes +z to +x
    second = BRepBuilderAPI_Transform(first, turn, True).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cylinders")
    return fuse.Shape()
