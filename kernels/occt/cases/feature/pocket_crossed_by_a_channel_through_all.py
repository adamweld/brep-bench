"""A blind pocket crossed by a channel cut through all from the side. A plate with a square blind
pocket in its top, and a channel cut across the plate from one side right through to the other,
crossing the pocket and running deeper than its floor.

- **The plate:** the box from the origin to (80, 50, 12).
- **The pocket:** the square 25 ≤ x ≤ 55, 10 ≤ y ≤ 40 on the top face, cut 7 deep, so its floor is
  at z = 5.
- **The channel:** the rectangle 34 ≤ x ≤ 46, 3 ≤ z ≤ 12 on the side face y = 0, square to the
  pocket's sketch, cut through all along +y, out through the side face y = 50. It is 12 wide, open
  at the top, with its floor at z = 3.

The channel crosses the pocket's middle: inside the pocket it is a trench 2 deeper than the pocket's
floor, which it splits in two, and it cuts an opening 12 wide and 7 tall in the pocket's walls y =
10 and y = 40."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Dir, gp_Pnt

from bench import Refused, face_through


def polygon_face(points):
    outline = BRepBuilderAPI_MakePolygon()
    for point in points:
        outline.Add(gp_Pnt(*point))
    outline.Close()
    return BRepBuilderAPI_MakeFace(outline.Wire()).Face()


def build():
    plate = BRepPrimAPI_MakeBox(80, 50, 12).Shape()

    # The pocket: a square sketched on the top face, cut (fuse 0) 7 down.
    square = polygon_face([(25, 10, 12), (55, 10, 12), (55, 40, 12), (25, 40, 12)])
    pocket = BRepFeat_MakePrism(plate, square, face_through(plate, (5, 5, 12)),
                                gp_Dir(0, 0, -1), 0, True)
    pocket.Perform(7.0)
    if not pocket.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not cut the pocket: {pocket.CurrentStatusError()}")
    pocketed = pocket.Shape()

    # The channel: a rectangle sketched on the side face y = 0, cut through all along +y.
    rectangle = polygon_face([(34, 0, 3), (46, 0, 3), (46, 0, 12), (34, 0, 12)])
    channel = BRepFeat_MakePrism(pocketed, rectangle, face_through(pocketed, (5, 0, 5)),
                                 gp_Dir(0, 1, 0), 0, True)
    channel.PerformThruAll()
    if not channel.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not cut the channel: {channel.CurrentStatusError()}")
    return channel.Shape()
