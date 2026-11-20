"""A narrow bar carrying a row of four cubes. The bar is [0, 60] x [2, 8] x [-4, 0]; four cubes of
side 10, [10i, 10i + 10] x [0, 10] x [0, 10] for i = 1 to 4, stand on its top face and touch one
another; all fused."""
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
    bar = BRepPrimAPI_MakeBox(gp_Pnt(0, 2, -4), gp_Pnt(60, 8, 0)).Shape()
    cubes = [BRepPrimAPI_MakeBox(gp_Pnt(10 * i, 0, 0), gp_Pnt(10 * i + 10, 10, 10)).Shape()
             for i in range(1, 5)]
    result = fuse_all([bar] + cubes)
    result = _simplify(result)
    return result
