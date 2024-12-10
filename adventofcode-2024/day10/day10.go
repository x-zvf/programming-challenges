package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

var DELTA = [][]int{
	{0, 1},
	{1, 0},
	{0, -1},
	{-1, 0},
}

func dfs(grid [][]int, y, x int, path []int) (paths [][]int) {
	paths = make([][]int, 0)
	if grid[y][x] == 9 {
		paths = append(paths, path)
		return
	}

	w := len(grid[0])
	h := len(grid)

	for _, d := range DELTA {
		dx, dy := d[0], d[1]
		ny, nx := y+dy, x+dx
		if nx >= 0 && ny >= 0 && ny < h && nx < w && grid[ny][nx]-grid[y][x] == 1 {
			pathcpy := make([]int, len(path)+1)
			copy(pathcpy, path)
			pathcpy[len(pathcpy)-1] = ny*w + nx
			ps := dfs(grid, ny, nx, pathcpy)
			paths = append(paths, ps...)
		}
	}

	return
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

	grid := make([][]int, 0)

	trailheads := make(map[int]bool, 0)
	y := 0
	for sc.Scan() {
		line := sc.Text()
		row := make([]int, 0)
		for x, v := range line {
			n, _ := strconv.Atoi(string(v))
			if n == 0 {
				trailheads[y*len(line)+x] = true
			}
			row = append(row, n)
		}
		grid = append(grid, row)
		y += 1
	}

	w := len(grid[0])
	decode := func(c int) (int, int) {
		return c / w, c % w
	}

	scores := 0
	ratings := 0
	for th := range trailheads {
		ty, tx := decode(th)
		p := make([]int, 0)
		trails := dfs(grid, ty, tx, p)
		peaks := make(map[int]bool, 0)
		for _, trail := range trails {
			if len(trail) > 0 {
				peak := trail[len(trail)-1]
				peaks[peak] = true
			}
		}
		scores += len(peaks)
		ratings += len(trails)
	}
	fmt.Println("part1: ", scores)
	fmt.Println("part2: ", ratings)
}
