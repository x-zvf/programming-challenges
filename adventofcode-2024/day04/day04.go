package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

var DELTA = [][]int{
	{0, 1},
	{0, -1},
	{1, 0},
	{-1, 0},
	{1, 1},
	{-1, -1},
	{-1, 1},
	{1, -1},
}
var DELTA2 = [][]int{
	{1, 1},
	{-1, -1},
	{1, -1},
	{-1, 1},
}

func countAt(sx, sy int, grid [][]rune) (sum int) {
	if (grid)[sy][sx] != 'X' {
		return 0
	}
	w := len(grid[0])
	h := len(grid)
	sum = 0
	for _, d := range DELTA {
		dx := d[0]
		dy := d[1]
		if sx+3*dx < w && sx+3*dx >= 0 && sy+3*dy < h && sy+3*dy >= 0 {
			if grid[sy+1*dy][sx+1*dx] == 'M' && grid[sy+2*dy][sx+2*dx] == 'A' && grid[sy+3*dy][sx+3*dx] == 'S' {
				sum += 1
			}
		}
	}
	return
}

func countAt2(sx, sy int, grid [][]rune) (sum int) {
	if grid[sy][sx] != 'A' {
		return 0
	}
	sum = 0
	for _, d := range DELTA2 {
		dx := d[0]
		dy := d[1]
		if grid[sy-dy][sx-dx] == 'M' && grid[sy+dy][sx+dx] == 'S' {
			fmt.Println(sy, sx)
			sum += 1
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

	grid := make([][]rune, 0)
	for sc.Scan() {
		line := sc.Text()
		grid = append(grid, []rune(line))
	}

	sum := 0
	for y := 0; y < len(grid); y++ {
		for x := 0; x < len(grid[0]); x++ {
			sum += countAt(x, y, grid)
		}
	}

	fmt.Println("part1: ", sum)

	sum2 := 0
	for y := 1; y < len(grid)-1; y++ {
		for x := 1; x < len(grid[0])-1; x++ {
			if countAt2(x, y, grid) >= 2 {
				sum2 += 1
			}
		}
	}

	fmt.Println("part2: ", sum2)
}
