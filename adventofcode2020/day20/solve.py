#!/bin/env python3

with open("input") as f:
    text = f.read()

tiles_s = text.split("\n\n")


def fliph(tile):
    return ["".join(reversed(x)) for x in tile]

def transpose(tile):
    return ["".join(x) for x in zip(*tile)]

def rotations(tile):
    rot = [tile]
    for _ in range(3):
        rot.append(fliph(transpose(rot[-1])))
    return rot

def perm(tile):
    return rotations(tile) + rotations(fliph(tile))


tiles = {}
for ts in tiles_s:
    n, *tile = ts.strip().split("\n")
    n = int(n[5:-1])
    tiles[n] = perm(tile)


sidelen = int(len(tiles) ** 0.5)
solution = [[0] * sidelen for _ in range(sidelen)]

stack = list(reversed([(row, col) for col in range(sidelen) for row in range(sidelen)]))

def solve():
    if len(stack) == 0:
        return True
    row, col = stack.pop()
    tids = list(tiles.keys())
    for tid in tids:
        perm = tiles[tid]
        del tiles[tid]
        for p in perm:
            if row > 0:
                if not solution[row-1][col][1][-1] == p[0]:
                    continue
            if col > 0:
                if not [x[-1] for x in solution[row][col-1][1]] == [x[0] for x in p]:
                    continue
            solution[row][col] = (tid, p)
            if solve():
                return True
        tiles[tid] = perm
    stack.append((row, col))

solve()

print(solution[0][0][0] * solution[0][-1][0] * solution[-1][0][0] * solution[-1][-1][0])

def no_border(tile):
    return [row[1:-1] for row in tile[1:-1]]

n_b_tiles = []
for tr in solution:
    a = []
    for i, tile in tr:
        a.append(no_border(tile))
    n_b_tiles.append(a)


sidelength = len(n_b_tiles) * len(n_b_tiles[0][0])
tile_sl = len(n_b_tiles[0][0])
image = [[0 for _ in range(sidelength)] for x in range(sidelength)]

for nrow, trow in enumerate(n_b_tiles):
    for ncol, tile in enumerate(trow):
        for ty, row in enumerate(tile):
            for tx, char in enumerate(row):
                y = (tile_sl*nrow)+ty
                x = (tile_sl*ncol)+tx
                image[y][x] = char

for row in image:
    print("".join(row))

monster = [
"                  # ",
"#    ##    ##    ###",
" #  #  #  #  #  #   ",
]

matches = 0

for p in perm(image):
    #if matches > 0:
    #    break
    for y_base in range(len(image)-len(monster)):
        for x_base in range(len(image[0])-len(monster[0])):
            match = True
            for y in range(len(monster)):
                for x in range(len(monster[0])):
                    if monster[y][x] == "#" and image[y_base+y][x_base+x] != "#":
                        match = False
                        break
                if not match:
                    break
            if match:
                matches += 1

n_sharp = sum(["".join(row).count("#") for row in image])
n_monster = sum([row.count("#") for row in monster])
print(n_sharp-n_monster * matches)
