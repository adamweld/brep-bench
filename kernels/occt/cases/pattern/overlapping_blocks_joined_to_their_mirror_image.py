"""Two overlapping blocks joined to their mirror image. The blocks from (0, 0, 0) to (10, 10, 10)
and from (5, 5, 0) to (15, 15, 10), united, reflected in the plane x = 7 and joined to the
original."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused
from edit import box, united


def build():
    blocks = united(box((0, 0, 0), (10, 10, 10)), box((5, 5, 0), (15, 15, 10)))
    mirror = gp_Trsf()
    mirror.SetMirror(gp_Ax2(gp_Pnt(7, 0, 0), gp_Dir(1, 0, 0)))
    image = BRepBuilderAPI_Transform(blocks, mirror, True)
    if not image.IsDone():
        raise Refused("BRepBuilderAPI_Transform could not reflect the blocks")
    return united(blocks, image.Shape())
