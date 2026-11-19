"""A slanted square pad extruded until it meets a block across its edge. A block, and a square pad
on a slanted plane, extruded until it meets the block. It reaches the block across one of its edges:
half of the pad ends on the block's top and half on its side.

The block is [0, 20] × [−10, 10] × [−20, 0]. Its top is the plane z = 0 and its side the plane x =
0, and they meet in the edge along the y axis.

The pad: let d = (1, 0, −1)/√2, pointing down at 45° toward +x, and n = (1, 0, 1)/√2. The square has
side 8 and centre (−5√2, 0, 5√2), 10 back along d from the block's edge. It lies in the plane
through its centre square to d, its sides parallel to the y axis and to n. It is extruded along d
until it meets the block (the block as a whole, not one chosen face of it), and joined to the block.

The line along d through the square's centre runs into the block's edge. The half of the square on
the +n side runs on to the block's top and the other half to its side. So the pad's far end is two
rectangles meeting along the block's edge: |y| ≤ 4, 0 ≤ x ≤ 4√2 on the top z = 0, and |y| ≤ 4, −4√2
≤ z ≤ 0 on the side x = 0. The pad is 10 long along d at its middle and 14 at its two sides parallel
to y, so its two sides square to y are pentagons. The pad and the block are one solid."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopoDS import TopoDS_Face
from OCP.gp import gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

HALF_SIDE, BACK = 4, 10
ALONG = gp_Vec(1, 0, -1).Normalized()  # d, down at 45° towards +x
ACROSS = gp_Vec(1, 0, 1).Normalized()  # n


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -10, -20), 20, 20, 20).Shape()
    centre = gp_Vec(0, 0, 0) - ALONG * BACK
    square = BRepBuilderAPI_MakePolygon()
    for s, t in [(-1, -1), (1, -1), (1, 1), (-1, 1)]:
        corner = centre + ACROSS * (s * HALF_SIDE) + gp_Vec(0, t * HALF_SIDE, 0)
        square.Add(gp_Pnt(corner.XYZ()))
    square.Close()
    sketch = BRepBuilderAPI_MakeFace(square.Wire()).Face()
    # A boss (fuse) along d up to the block itself, the whole shape, not one face of it.
    pad = BRepFeat_MakePrism(block, sketch, TopoDS_Face(), gp_Dir(ALONG), 1, True)
    pad.Perform(block)
    if not pad.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not extrude the pad: {pad.CurrentStatusError()}")
    return pad.Shape()
