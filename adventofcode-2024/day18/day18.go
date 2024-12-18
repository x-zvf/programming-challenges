package main

import (
	"fmt"
	"strconv"
	//pq "github.com/emirpasic/gods/queues/priorityqueue"
	"log"
	"os"
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
	if len(os.Args) < 4 {
		fmt.Println("Usage: ./day16 <FILE> <size> <simfirst>")
		os.Exit(1)
	}
	size, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal("failed to parse size")
	}
	simfirst, err := strconv.Atoi(os.Args[3])
	if err != nil {
		log.Fatal("failed to parse size")
	}

	bytes, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal("failed to read")
	}
	input := strings.Split(strings.Trim(string(bytes), " \r\n\t"), "\n")
	grid := make([][]int, 0)
	for range size {
		r := make([]int, size)
		grid = append(grid, r)
	}
	for i, l := range input {
		sp := strings.Split(l, ",")
		x, _ := strconv.Atoi(sp[0])
		y, _ := strconv.Atoi(sp[1])
		grid[y][x] = i + 1
	}

	encode := func(x, y int) int { return y*size + x }
	vp := func(x, y int) bool {
		return x >= 0 && y >= 0 && x < size && y < size
	}

	findpath := func(sf int) (int, bool) {
		visited := make([]bool, size*size)

		queue := make([]Item, 1)
		queue[0] = Item{0, 0, 0}

		for len(queue) > 0 {
			item := queue[0]
			queue = queue[1:]

			ie := encode(item.x, item.y)
			if visited[ie] {
				continue
			}
			visited[ie] = true

			if item.y == size-1 && item.x == size-1 {
				return item.score, true
			}

			for _, d := range DIR {
				dx, dy := d[0], d[1]
				nx, ny := item.x+dx, item.y+dy
				if vp(nx, ny) && (grid[ny][nx] == 0 || (grid[ny][nx] > 0 && grid[ny][nx] > sf)) && !visited[encode(nx, ny)] {
					queue = append(queue, Item{nx, ny, item.score + 1})
				}
			}
		}
		return -1, false
	}
	p1, _ := findpath(simfirst)
	fmt.Println("part1: ", p1)

	i := simfirst + 1
	for {
		_, ok := findpath(i)
		if !ok {
			break
		}
		i++
	}
	fmt.Println("part2: ", input[i-1])
}
