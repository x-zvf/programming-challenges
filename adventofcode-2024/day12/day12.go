package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

var DELTA = [][]int{
	{-1, 0}, //up (2 ^ 0)
	{0, -1}, //left (2 ^ 1)
	{1, 0},  //down (2 ^ 2)
	{0, 1},  //right (2 ^ 3)
}

// see graph paper
var OUTER_CORNERS = []int{
	0, 0, 0, 1, 0, 0, 1,
	2, 0, 1, 0, 2,
	1, 2, 2, 4,
}
var CHECK_INNER_CORNERS = [][][]int{
	{{-1, -1}, {-1, 1}, {1, -1}, {1, 1}},
	{{1, -1}, {1, 1}},
	{{-1, 1}, {1, 1}},
	{{1, 1}},
	{{-1, -1}, {-1, 1}},
	{},
	{{-1, 1}},

	{},
	{{-1, -1}, {1, -1}},
	{{1, -1}},
	{},
	{},

	{{-1, -1}},
	{},
	{},
	{},
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day12 <FILE>")
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

	w := len(grid[0])
	h := len(grid)

	encode := func(y, x int) int { return y*w + x }
	decode := func(c int) (int, int) {
		return c / w, c % w
	}

	visited := make([][]bool, 0)
	for range h {
		visited = append(visited, make([]bool, w))
	}

	part1 := 0
	part2 := 0
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if visited[y][x] {
				continue
			}
			c := grid[y][x]
			area := 0
			perimeter := 0
			corners := 0

			stack := make([]int, 0)
			stack = append(stack, encode(y, x))
			for len(stack) > 0 {
				cy, cx := decode(stack[len(stack)-1])
				stack = stack[:len(stack)-1]

				if visited[cy][cx] {
					continue
				}
				visited[cy][cx] = true

				area += 1
				perimeter += 4
				cornertype := 0
				for i, d := range DELTA {
					dy, dx := d[0], d[1]
					ny, nx := cy+dy, cx+dx
					if ny < 0 || nx < 0 || ny >= h || nx >= w {
						cornertype += (1 << i)
					} else if grid[ny][nx] == c {
						perimeter -= 1
						stack = append(stack, encode(ny, nx))
					} else {
						cornertype += (1 << i)
					}
				}
				outercorners := OUTER_CORNERS[cornertype]
				innercorners := 0
				for _, corner := range CHECK_INNER_CORNERS[cornertype] {
					dy, dx := corner[0], corner[1]
					ny, nx := cy+dy, cx+dx
					if ny < 0 || nx < 0 || ny >= h || nx >= w {
						log.Fatal("impossible corner", cy, cx, ny, nx)
					} else if grid[ny][nx] != c {
						innercorners += 1
					}
				}
				corners += outercorners + innercorners
			}
			part1 += area * perimeter
			part2 += area * corners
		}
	}
	fmt.Println("part1: ", part1)
	fmt.Println("part2: ", part2)
}
