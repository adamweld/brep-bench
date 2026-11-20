"""Two equal cubes, one slid along a side, their common part. The cubes from the origin to
(10, 10, 10) and from (4, 0, 0) to (14, 10, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common

from bench import Refused
from edit import box


def build():
    common = BRepAlgoAPI_Common(box((0, 0, 0), (10, 10, 10)), box((4, 0, 0), (14, 10, 10)))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two cubes")
    return common.Shape()
