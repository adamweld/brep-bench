"""A cube mirrored, repeated in a row and turned four ways. The cube [10, 20] x [0, 10] x [0, 10]
is mirrored across the plane x = 0, the pair repeated along +y at spacing 10 (three in all), and the
whole turned about the z axis in four copies at 0, 90, 180 and 270 degrees; all fused."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused


def moved(shape, trsf):
    return BRepBuilderAPI_Transform(shape, trsf, True).Shape()


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
    cube = BRepPrimAPI_MakeBox(gp_Pnt(10, 0, 0), gp_Pnt(20, 10, 10)).Shape()
    mirror = gp_Trsf()
    mirror.SetMirror(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)))  # the plane x = 0
    pair = [cube, moved(cube, mirror)]
    row = []
    for k in range(3):
        shift = gp_Trsf()
        shift.SetTranslation(gp_Vec(0, 10 * k, 0))
        row += [moved(s, shift) for s in pair]
    copies = []
    for k in range(4):
        turn = gp_Trsf()
        turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), k * 3.141592653589793 / 2)
        copies += [moved(s, turn) for s in row]
    result = fuse_all(copies)
    result = _simplify(result)
    return result
