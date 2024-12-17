package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

var DELTA = map[rune][]int{
	'^': {0, -1},
	'>': {1, 0},
	'v': {0, 1},
	'<': {-1, 0},
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day15 <FILE>")
		os.Exit(1)
	}
	bytes, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal("failed to read")
	}
	input := strings.Split(string(bytes), "\n\n")
	moves := input[1]

	gridlines := strings.Split(strings.Trim(input[0], " \t\r\n"), "\n")
	grid := make([][]rune, 0)
	for _, l := range gridlines {
		grid = append(grid, []rune(l))
	}
	w := len(grid[0])
	h := len(grid)

	ry, rx := -1, -1
	for y := 0; y < h && ry == -1; y++ {
		for x := 0; x < w; x++ {
			if grid[y][x] == '@' {
				ry = y
				rx = x
				break
			}
		}
	}
	orx, ory := rx, ry

	getFree := func(x, y, dx, dy int) (cx, cy int, free bool) {
		cx, cy, free = x, y, false
		for {
			cx, cy = cx+dx, cy+dy
			if !(cx+dx >= 0 && cx+dx < w && cy+dy >= 0 && cy+dy < h) {
				return
			}
			if grid[cy][cx] == '#' {
				return
			}
			if grid[cy][cx] == '.' {
				free = true
				return
			}
		}
	}

	for _, m := range moves {
		delta, ok := DELTA[m]
		if !ok {
			continue
		}
		dx, dy := delta[0], delta[1]
		if cx, cy, free := getFree(rx, ry, dx, dy); free {
			for cx != rx || cy != ry {
				grid[cy][cx], grid[cy-dy][cx-dx] = grid[cy-dy][cx-dx], grid[cy][cx]
				cy -= dy
				cx -= dx
			}
			rx += dx
			ry += dy
		}
	}

	score := func(grid [][]rune) (s int) {
		s = 0
		for y, l := range grid {
			for x, c := range l {
				if c == 'O' || c == '[' {
					s += 100*y + x
				}
			}
		}
		return
	}

	fmt.Println("part1: ", score(grid))

	grid2 := make([][]rune, 0)
	for _, l := range gridlines {
		l2 := make([]rune, 0)
		for _, c := range l {
			switch c {
			case '@':
				l2 = append(l2, '@', '.')
				break
			case 'O':
				l2 = append(l2, '[', ']')
				break
			case '#':
				l2 = append(l2, '#', '#')
				break
			case '.':
				l2 = append(l2, '.', '.')
				break

			}
		}
		grid2 = append(grid2, l2)
	}

	grid = grid2
	w = len(grid2[0])
	h = len(grid2)

	rx, ry = orx*2, ory

	for _, m := range moves {
		delta, ok := DELTA[m]
		if !ok {
			continue
		}
		dx, dy := delta[0], delta[1]

		encode := func(x, y int) int { return y*w + x }
		decode := func(c int) (int, int) {
			return c % w, c / w
		}

		tomove := make([]int, 0)
		tocheck := make([]int, 0)
		tocheck = append(tocheck, encode(rx, ry))

		blocked := false
		for i := 0; i < len(tocheck); i++ {
			cx, cy := decode(tocheck[i])
			if grid[cy][cx] == '#' {
				blocked = true
				break
			}
			if grid[cy][cx] == '.' {
				continue
			}
			ncx, ncy := cx+dx, cy+dy

			tocheck = append(tocheck, encode(ncx, ncy))
			tomove = append(tomove, encode(ncx, ncy))
			if dy != 0 {
				if grid[ncy][ncx] == '[' {
					tocheck = append(tocheck, encode(ncx+1, ncy))
				} else if grid[ncy][ncx] == ']' {
					tocheck = append(tocheck, encode(ncx-1, ncy))
				}
			}
		}
		if blocked {
			continue
		}
		moved := make(map[int]bool, 0)
		for i := len(tomove) - 1; i >= 0; i-- {
			if moved[tomove[i]] {
				continue
			}
			mx, my := decode(tomove[i])
			grid[my][mx], grid[my-dy][mx-dx] = grid[my-dy][mx-dx], grid[my][mx]
			moved[tomove[i]] = true
		}
		rx, ry = rx+dx, ry+dy
	}
	fmt.Println("part2: ", score(grid))
}
