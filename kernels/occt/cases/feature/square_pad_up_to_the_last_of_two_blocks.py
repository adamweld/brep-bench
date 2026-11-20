"""A square pad extruded through one block and up to the far face of a second. Blocks
[10, 18] x [-8, 8]^2 and [26, 34] x [-8, 8]^2; the square |y|, |z| <= 3 in the plane x = 0 extruded
along +x to x = 34, the far face of the last block; all fused."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


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
    first = BRepPrimAPI_MakeBox(gp_Pnt(10, -8, -8), gp_Pnt(18, 8, 8)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(26, -8, -8), gp_Pnt(34, 8, 8)).Shape()
    pad = BRepPrimAPI_MakeBox(gp_Pnt(0, -3, -3), gp_Pnt(34, 3, 3)).Shape()
    result = fuse_all([first, second, pad])
    result = _simplify(result)
    return result
