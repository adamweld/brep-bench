"""A slot swung by a pin lying along a radius. A curved block with a slot swung through it by a
pin.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The pin is a solid cylinder of radius 0.5 whose axis is the x axis, from x = 6 to x = 11, its ends
flat. Its axis meets the block's at the origin.

The pin turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it
starts clear of the block, passes through it, and ends clear of it. Cut from the block everything
the pin passes through.

Each circle of the pin about its own axis touches at its two points in the plane of the two axes,
so the pin's side sweeps the slot's flat roof and floor, z = ±0.5. On the rim of the pin's far end
the circle whose normals pass through the origin touches all round and sweeps a zone of the sphere
of radius √(11² + 0.5²) about the origin, the slot's end wall. The slot runs right through the
block's inner wall and both its flat sides, so the block falls into what lies above the slot and
what lies below it, joined beyond the sphere."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
RADIUS, NEAR, FAR = 0.5, 6, 11  # the pin, along the x axis
START, STOP = -140, 140  # degrees about the z axis


def build():
    pin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(NEAR, 0, 0), gp.DX_s()), RADIUS,
                                   FAR - NEAR).Shape()
    tool = turn(pin, gp.OZ_s(), START)
    # The pin's axis meets the turn's. Its side, and its flat ends (each square to a line
    # through the turn's axis), meet the turn square-on where their normals lie in the plane
    # through both axes, which cuts them along their contact.
    through_axes = turn(plane((0, 0, 0), (0, 1, 0)), gp.OZ_s(), START)
    swept = turned(tool, gp.OZ_s(), STOP - START, [through_axes])
    return cut(ring_sector(*BLOCK), swept)
