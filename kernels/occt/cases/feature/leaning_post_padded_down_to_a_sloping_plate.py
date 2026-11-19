"""A leaning square post extruded aslant down to a sloping plate. A plate whose top slopes, and a
square post on a plane above it extruded aslant, down until it meets the plate's top face, and
joined to it.

The plate: the box −30 ≤ x ≤ 30, −20 ≤ y ≤ 20, z ≥ 0 cut off above by the plane z = 6 + x/10, so it
is 3 thick at its end x = −30 and 9 thick at x = 30; its sloping top is a rectangle 40 wide.

The post: the square 5 ≤ x ≤ 15, −5 ≤ y ≤ 5 in the plane z = 40, extruded along the direction (−1,
0, −2), down and leaning toward −x, not square to the square's plane, until it meets the plate's top
face. Each point (x₀, y₀, 40) of the square runs to (x₀ − s, y₀, 40 − 2s) with s = (34 − x₀/10)/1.9,
where the line meets the plate's top: the square's centre lands at about (−7.368, 0, 5.263). The
post's foot is a parallelogram on the sloping top, wholly inside it, and its top is the square at z
= 40. Its sides square to y are parallelograms in the planes y = ±5; its other two sides slant,
along y and along the direction of extrusion."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.TopoDS import TopoDS_Face
from OCP.gp import gp_Dir, gp_Pnt, gp_Vec

from bench import Refused, face_through


def polygon_face(points):
    outline = BRepBuilderAPI_MakePolygon()
    for point in points:
        outline.Add(gp_Pnt(*point))
    outline.Close()
    return BRepBuilderAPI_MakeFace(outline.Wire()).Face()


def build():
    # The plate: its section in the plane y = −20, under the slope z = 6 + x/10, run 40 along y.
    section = polygon_face([(-30, -20, 0), (30, -20, 0), (30, -20, 9), (-30, -20, 3)])
    plate = BRepPrimAPI_MakePrism(section, gp_Vec(0, 40, 0)).Shape()
    top = face_through(plate, (0, 0, 6))
    square = polygon_face([(5, -5, 40), (15, -5, 40), (15, 5, 40), (5, 5, 40)])
    # A boss (fuse) from the square, which lies on no face of the plate, along (−1, 0, −2) until
    # the plate's sloping top.
    post = BRepFeat_MakePrism(plate, square, TopoDS_Face(), gp_Dir(-1, 0, -2), 1, True)
    post.Perform(top)
    if not post.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not extrude the post: {post.CurrentStatusError()}")
    return post.Shape()
