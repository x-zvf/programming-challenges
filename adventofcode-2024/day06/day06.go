package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

var R2ID = map[rune]int{
	// dy dx
	'^': 0,
	'>': 1,
	'v': 2,
	'<': 3,
}
var DELTA = [][]int{
	{-1, 0},
	{0, 1},
	{1, 0},
	{0, -1},
}

func eval(grid [][]rune, sy, sx, did int) (int, map[int]int) {
	w := len(grid[0])
	h := len(grid)
	ingrid := func(y, x int) bool {
		return !(y < 0 || x < 0 || y >= h || x >= w)
	}
	encode := func(y, x int) int {
		return w*y + x
	}

	visited_c := 1
	visited := make(map[int]int, 0)
	visited[encode(sy, sx)] = did
	for {
		ny := sy + DELTA[did][0]
		nx := sx + DELTA[did][1]
		for ingrid(ny, nx) && grid[ny][nx] == '#' {
			did = (did + 1) % len(DELTA)
			ny = sy + DELTA[did][0]
			nx = sx + DELTA[did][1]
		}
		if !ingrid(ny, nx) {
			break
		}
		odir, found := visited[encode(ny, nx)]
		if found {
			if odir == did {
				return -1, visited // loop
			}
		} else {
			visited_c += 1
			visited[encode(ny, nx)] = did
			//grid[ny][nx] = 'X'
		}
		sy = ny
		sx = nx
	}
	/* for _, line := range grid {
		for _, c := range line {
			fmt.Print("", string(c))
		}
		fmt.Println()
	} */
	return visited_c, visited
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day02 <FILE>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	grid := make([][]rune, 0)
	for sc.Scan() {
		line := sc.Text()
		grid = append(grid, []rune(line))
	}

	var sx, sy, did int
	found := false

	for y, r := range grid {
		if found {
			break
		}
		for x, c := range r {
			v, ok := R2ID[c]
			if ok {
				found = true
				sx = x
				sy = y
				did = v
				break
			}
		}
	}

	part1, visited := eval(grid, sy, sx, did)
	fmt.Println("part1 :", part1)

	w := len(grid[0])
	decode := func(v int) (y int, x int) {
		y = v / w
		x = v % w
		return
	}

	loops := 0
	for k := range visited {
		y, x := decode(k)
		grid[y][x] = '#'
		v, _ := eval(grid, sy, sx, did)
		if v == -1 {
			loops += 1
		}
		grid[y][x] = '.'
	}

	fmt.Println("part2 :", loops)

}
