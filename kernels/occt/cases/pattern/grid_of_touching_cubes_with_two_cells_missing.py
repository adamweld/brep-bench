"""A grid of touching cubes with two cells missing. Four cells along x, three along y, each a cube of
side 10 on the grid of 10, one layer (z from 0 to 10); the cells (0, 0) and (2, 1) are left out and the
ten others fused into one solid."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

MISSING = {(0, 0), (2, 1)}


def fuse_all(shapes):
    """The shapes fused one after another into one solid."""
    result = shapes[0]
    for shape in shapes[1:]:
        fuse = BRepAlgoAPI_Fuse(result, shape)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a shape")
        result = fuse.Shape()
    return result


def _simplify(shape):
    """Merge the coplanar faces the shared walls leave."""
    from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
    unify = ShapeUpgrade_UnifySameDomain(shape, True, True, True)
    unify.Build()
    return unify.Shape()


def build():
    boxes = [BRepPrimAPI_MakeBox(gp_Pnt(10 * i, 10 * j, 0), gp_Pnt(10 * i + 10, 10 * j + 10, 10)).Shape()
             for i in range(4) for j in range(3) if (i, j) not in MISSING]
    result = fuse_all(boxes)
    result = _simplify(result)
    return result
