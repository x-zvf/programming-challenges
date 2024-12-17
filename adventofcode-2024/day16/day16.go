package main

import (
	"fmt"
	pq "github.com/emirpasic/gods/queues/priorityqueue"
	"log"
	"os"
	"strings"
)

type Item struct {
	x, y  int
	dir   int
	score int
	path  []int
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
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day16 <FILE>")
		os.Exit(1)
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

	sx, sy, ex, ey := -1, -1, -1, -1
	for y := 0; y < h && (sy == -1 || ey == -1); y++ {
		for x := 0; x < w; x++ {
			if grid[y][x] == 'S' {
				sy = y
				sx = x
			} else if grid[y][x] == 'E' {
				ey = y
				ex = x
			}
		}
	}

	encode := func(x, y int) int { return y*w + x }
	//decode := func(c int) (int, int) { return c % w, c / w }
	visited := make([]int, w*h)
	minpaths := make([]bool, w*h)

	q := pq.NewWith(func(a, b interface{}) int {
		return a.(Item).score - b.(Item).score
	})

	vp := func(x, y int) bool {
		return x >= 0 && y >= 0 && x < w && y < h
	}

	minscore := 1 << 62
	sp := make([]int, 1)
	sp[0] = encode(sx, sy)
	q.Enqueue(Item{sx, sy, 1, 0, sp})
	for !q.Empty() {
		e, _ := q.Dequeue()
		i := e.(Item)
		visited[encode(i.x, i.y)] |= (1 << i.dir)
		if i.score > minscore {
			break
		}
		if i.y == ey && i.x == ex {
			minscore = i.score
			for _, x := range i.path {
				minpaths[x] = true
			}
		}

		ed := func(dir, sa int) {
			d := DIR[dir]
			dx, dy := d[0], d[1]
			nx, ny := i.x+dx, i.y+dy
			if vp(nx, ny) && grid[ny][nx] != '#' && (visited[encode(nx, ny)]&(1<<dir) == 0) {
				cpy := make([]int, len(i.path)+1)
				copy(cpy, i.path)
				cpy[len(i.path)] = encode(nx, ny)
				q.Enqueue(Item{nx, ny, dir, i.score + sa, cpy})
			}
		}
		ed(i.dir, 1)
		ed(mod(i.dir+1, 4), 1001)
		ed(mod(i.dir-1, 4), 1001)
	}

	nmp := 0
	for _, v := range minpaths {
		if v {
			nmp += 1
		}
	}
	fmt.Println("part1: ", minscore)
	fmt.Println("part2: ", nmp)
}
