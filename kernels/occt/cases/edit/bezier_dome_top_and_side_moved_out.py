"""The Bezier dome block, its top moved up and a side moved out. The block under the bicubic Bezier dome
of nurbs/bezier-dome, edited directly: its top moved 1 along its normal, upward, and its side
y = 20 moved out by 2."""
from bench import face_through, other_case
from edit import moved

TOP_MOVE = 1
SIDE_MOVE = 2


def build():
    dome = other_case("nurbs/bezier-dome")
    block = dome.build()
    bernstein = (1 / 8, 3 / 8, 3 / 8, 1 / 8)  # B_i(1/2)
    height = sum(bernstein[i] * bernstein[j] * dome.HEIGHTS[i][j] for i in range(4) for j in range(4))
    top = face_through(block, (10, 10, height))  # the patch over the middle of the square
    side = face_through(block, (10, 20, 1))      # the upright side y = 20
    return moved(block, [(top, TOP_MOVE), (side, SIDE_MOVE)])
