#!/usr/bin/env python3


import random
import math


STAY_PROB = 0.75
CHANGE_PROB = 1.0 - STAY_PROB
STEP_SIZE = 1.0


def cube_add(c1, c2):
    return (c1[0] + c2[0], c1[1] + c2[1], c1[2] + c2[2])


def cube_scale(c, a):
    return (c[0] * a, c[1] * a, c[2] * a)


def cube_direction(direction):
    cube_directions = [
        (1, -1, 0), (1, 0, -1), (0,  1, -1),
        (-1, 1, 0), (-1, 0, 1), (0, -1,  1),
    ]

    return cube_directions[direction]


def cube_neighbor(c, direction):
    return cube_add(c, cube_direction(direction))


def cube_ring(center, radius):
    """ Doesn't work for `radius == 0` """

    results = []
    cube = cube_add(center, cube_scale(cube_direction(4), radius))
    for i in range(6):
        for j in range(radius):
            results.append(cube)
            cube = cube_neighbor(cube, i)

    return results


def sign(x): return bool(x > 0) - bool(x < 0)


def cube_to_axial(c):
    return (c[0], c[2])


def flat_axial_to_pixel(axial, size):
    x = size * (3.0 / 2.0 * axial[0])
    y = size * (math.sqrt(3.0) / 2.0 * axial[0] + math.sqrt(3.0) * axial[1])
    return (x, y)


hexes = {(0, 0, 0): (0.0, 0)}
iterations = 12
for i in range(1, iterations + 1):
    for c in cube_ring((0, 0, 0), i):
        maxima = []
        for j, coord in enumerate(c):
            if not maxima:
                maxima.append((j, coord))
            elif abs(coord) > abs(maxima[0][1]):
                maxima = [(j, coord)]
            elif abs(coord) == abs(maxima[0][1]):
                maxima.append((j, coord))

        parents = []
        if len(maxima) > 1:
            parent = [0, 0, 0]
            for j, coord in maxima:
                parent[j] = coord - sign(coord)
            parents.append(tuple(parent))
        else:
            j, coord = maxima[0]
            sgn = sign(coord)
            coord -= sgn
            for k in range(3):
                if k == j:
                    continue
                parent = list(c)
                parent[j] = coord
                parent[k] += sgn
                parents.append(tuple(parent))

        parent_height, parent_dir = hexes[parents[0]] if len(parents) == 1 else (
            hexes[parents[0]] if random.randint(0, 1) else hexes[parents[1]])

        if random.random() < STAY_PROB:
            our_dir = parent_dir
        else:
            our_dir = random.choice(list({-1, 0, 1} - {parent_dir}))
        our_height = parent_height + our_dir * STEP_SIZE

        hexes[c] = (our_height, our_dir)

translations = []
for cube, (height, _) in hexes.items():
    axial = cube_to_axial(cube)
    pixel = flat_axial_to_pixel(axial, 1.0)
    translations.append([pixel[0], pixel[1], height])

arr = sum(translations, [])

print(translations)
