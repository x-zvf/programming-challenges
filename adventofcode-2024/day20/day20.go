package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Item struct {
	x, y  int
	score int
}

var DIR = [][]int{
	{0, -1},
	{1, 0},
	{0, 1},
	{-1, 0},
}

func mod(a, b int) int {
	return (a%b + b) % b
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: ./day20 <FILE> <mincount>")
		os.Exit(1)
	}
	mincount, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal("failed to parse mincount")
	}

	bytes, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal("failed to read")
	}
	input := strings.Split(strings.Trim(string(bytes), " \r\n\t"), "\n")
	grid := make([][]rune, 0)
	for _, l := range input {
		grid = append(grid, []rune(l))
	}
	w := len(grid[0])
	h := len(grid)
	encode := func(x, y int) int { return y*w + x }
	decode := func(n int) (int, int) { return n % w, n / w }
	vp := func(x, y int) bool {
		return x >= 0 && y >= 0 && x < w && y < h
	}

	sx, sy := -1, -1
	for y, r := range grid {
		for x, c := range r {
			if c == 'S' {
				sx, sy = x, y
				break
			}
		}
		if sy != -1 {
			break
		}
	}

	distances := make([]int, w*h)
	for i := range distances {
		distances[i] = -1
	}

	relevant := make([]int, 0)

	queue := make([]Item, 1)
	queue[0] = Item{sx, sy, 0}

	basedist := -1
	for len(queue) > 0 {
		item := queue[0]
		queue = queue[1:]

		ie := encode(item.x, item.y)
		if distances[ie] >= 0 {
			continue
		}
		relevant = append(relevant, ie)
		if grid[item.y][item.x] == 'E' && basedist < 0 {
			basedist = item.score
			//break
		}

		distances[ie] = item.score

		for _, d := range DIR {
			dx, dy := d[0], d[1]
			nx, ny := item.x+dx, item.y+dy
			if vp(nx, ny) && grid[ny][nx] != '#' && distances[encode(nx, ny)] < 0 {
				queue = append(queue, Item{nx, ny, item.score + 1})
			}
		}
	}
	part1 := 0
	part2 := 0
	for n1 := 0; n1 < len(relevant)-1; n1++ {
		i1 := relevant[n1]
		for n2 := n1 + 1; n2 < len(relevant); n2++ {
			i2 := relevant[n2]
			n1x, n1y := decode(i1)
			n2x, n2y := decode(i2)
			dx, dy := n2x-n1x, n2y-n1y
			if dx < 0 {
				dx = -dx
			}
			if dy < 0 {
				dy = -dy
			}
			manh := dx + dy
			dd := distances[i2] - distances[i1]
			if dd < 0 {
				dd = -dd
			}
			if dd-manh >= mincount {
				if manh == 2 {
					part1 += 1
				}
				if manh <= 20 {
					part2 += 1
				}
			}
		}
	}
	fmt.Println("part1: ", part1)
	fmt.Println("part2: ", part2)
}
