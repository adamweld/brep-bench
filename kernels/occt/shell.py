"""What the OCCT implementations of the shell cases share: hollowing a solid to a wall of constant
thickness with BRepOffsetAPI_MakeThickSolid (and `bench.face_through`, picking a face by a point).

MakeThickSolid removes the faces it is given (the openings), moves every other face by the
thickness, and joins the moved faces to the solid's own across the openings, as rims on the
openings' surfaces. Inward (a negative offset) the solid's faces stay the outside; outward (a
positive one) they become the inside. Joined by arcs (GeomAbs_Arc), moved faces that part at an
edge are joined by a round about it; joined by intersection (GeomAbs_Intersection), they are
extended until they meet and every edge stays sharp.

MakeThickSolid wants at least one opening: given none it returns the moved faces' solid alone, not
the hollow. A closed hollow is made by `hollow`, the solid's shell and its offset's, reversed, as the
two shells of one solid.
"""
from OCP.BRep import BRep_Builder
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepOffset import BRepOffset_Skin
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffsetShape, BRepOffsetAPI_MakeThickSolid
from OCP.GeomAbs import GeomAbs_Arc
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS_Solid
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp_Pnt

from bench import Refused, face_through
from offset import offset

__all__ = ["face_through", "hollow", "shell", "shell_to_plane"]  # face_through, as the shell cases pick faces

TOLERANCE = 1e-7


def shell(solid, openings, thickness, outward=False, join=GeomAbs_Arc):
    """`solid` hollowed to a wall `thickness` thick, open at the faces `openings`: inside its faces,
    or outside them where `outward`. Refused if OCCT declines."""
    closing = List_TopoDS_Shape()
    for face in openings:
        closing.Append(face)
    maker = BRepOffsetAPI_MakeThickSolid()
    # Skin mode moves every face but the openings. Intersection = True intersects each moved face
    # with every other, not only with those beside it, so moved faces that cross faces across a
    # narrow neck, or a round thinner than the wall, are trimmed too.
    maker.MakeThickSolidByJoin(solid, closing, thickness if outward else -thickness, TOLERANCE,
                               BRepOffset_Skin, True, False, join)
    maker.Build()
    if not maker.IsDone():
        raise Refused(f"BRepOffsetAPI_MakeThickSolid declined to shell the solid {thickness} thick")
    return maker.Shape()


def hollow(solid, thickness, join=GeomAbs_Arc):
    """`solid` hollowed to a wall `thickness` thick with no opening: one solid whose shells are the
    solid's own and, reversed to face the void, the shell of the solid shrunk by `thickness`."""
    maker = BRepOffsetAPI_MakeOffsetShape()
    maker.PerformByJoin(solid, -thickness, TOLERANCE, BRepOffset_Skin, True, False, join)
    if not maker.IsDone():
        raise Refused(f"BRepOffsetAPI_MakeOffsetShape declined to shrink the solid by {thickness}")
    builder, result = BRep_Builder(), TopoDS_Solid()
    builder.MakeSolid(result)
    builder.Add(result, only_shell(solid))
    builder.Add(result, only_shell(maker.Shape()).Reversed())
    return result


def only_shell(shape):
    shells = TopExp_Explorer(shape, TopAbs_SHELL)
    assert shells.More(), "no shell"
    found = shells.Current()
    shells.Next()
    assert not shells.More(), "more than one shell"
    return found


def shell_to_plane(solid, extended, thickness, z):
    """`solid` hollowed to a wall `thickness` thick, open at its face in the plane z = `z`, where
    `extended` is the same solid with its side continued past that plane. The cavity is `extended`
    shrunk by `thickness`, cut off at the plane; the result is `solid` less it, so the moved side
    runs on to the plane of the opening, which is the rim."""
    cavity = offset(extended, -thickness)
    above = BRepPrimAPI_MakeBox(gp_Pnt(-1000, -1000, z), gp_Pnt(1000, 1000, 1000)).Shape()
    trimmed = BRepAlgoAPI_Cut(cavity, above)
    if not trimmed.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the cavity at the opening's plane")
    result = BRepAlgoAPI_Cut(solid, trimmed.Shape())
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the cavity from the solid")
    return result.Shape()
