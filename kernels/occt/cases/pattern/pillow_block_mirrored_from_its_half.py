"""A pillow block mirrored from its half. A pillow block, a bearing housing on a base with two bolt
holes, made as one half mirrored across its middle plane x = 0 and joined to its mirror image.

The half, all of it in x ≥ 0:

- the base, the box from (0, −15, 0) to (40, 15, 12);
- the housing, the box from (0, −15, 0) to (20, 15, 34), joined to the base;
- half the bore: the half-cylinder of radius 10 about the line x = 0, z = 20 (running along y) taken
  away, from y = −15 to y = 15, so the half has a half-round groove in its face x = 0 whose deepest
  point is at x = 10;
- a bolt hole of radius 4 on the vertical axis through (30, 0), right through the base from z = 0 to
  z = 12.

Mirror the half in the plane x = 0 and join the two. The halves meet face to face over the whole of
that plane, where their faces lie on each other and are no longer faces of the solid: the result is
one solid, symmetric about x = 0, with a full bore of radius 10 through the housing along y, centred
at height 20 (so it comes within 4 of the housing's top at z = 34 and 10 of the bottom), and a bolt
hole through the base on each side, at x = ±30."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused


def half():
    base = BRepPrimAPI_MakeBox(gp_Pnt(0, -15, 0), gp_Pnt(40, 15, 12)).Shape()
    housing = BRepPrimAPI_MakeBox(gp_Pnt(0, -15, 0), gp_Pnt(20, 15, 34)).Shape()
    body = BRepAlgoAPI_Fuse(base, housing).Shape()
    # Half the bore: the cylinder about the line x = 0, z = 20 takes away its half in x ≥ 0.
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -16, 20), gp_Dir(0, 1, 0)), 10, 32).Shape()
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(30, 0, -1), gp_Dir(0, 0, 1)), 4, 14).Shape()
    cut = BRepAlgoAPI_Cut(BRepAlgoAPI_Cut(body, bore).Shape(), hole)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the half's bore and bolt hole")
    return cut.Shape()


def build():
    right = half()
    mirror = gp_Trsf()
    mirror.SetMirror(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)))  # in the plane x = 0
    left = BRepBuilderAPI_Transform(right, mirror, True).Shape()
    joined = BRepAlgoAPI_Fuse(right, left)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the half to its mirror image")
    # The two halves' faces lying in one surface (bottom, top, front, back, bore) become one.
    unify = ShapeUpgrade_UnifySameDomain(joined.Shape(), True, True, True)
    unify.Build()
    return unify.Shape()
