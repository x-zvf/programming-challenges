package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

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

	w := len(grid[0])
	h := len(grid)

	encode := func(y, x int) int { return y*w + x }
	decode := func(c int) (int, int) {
		return c / w, c % w
	}

	antennas := make(map[rune][]int)

	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			c := grid[y][x]
			if c != '.' {
				antennas[c] = append(antennas[c], encode(y, x))
			}
		}
	}

	antinodes := make(map[int]bool)
	antinodes2 := make(map[int]bool)

	validCoord := func(y, x int) bool {
		return y < w && y >= 0 && x < h && x >= 0
	}
	for _, as := range antennas {
		for a := 0; a < len(as); a++ {
			for b := 0; b < len(as); b++ {
				if a == b {
					continue
				}
				ay, ax := decode(as[a])
				by, bx := decode(as[b])
				// the other direction is covered by switching a and b
				dy, dx := by-ay, bx-ax
				ry, rx := by+dy, bx+dx
				if validCoord(ry, rx) {
					antinodes[encode(ry, rx)] = true
				}

				antinodes2[encode(by, bx)] = true
				for validCoord(ry, rx) {
					antinodes2[encode(ry, rx)] = true
					ry, rx = ry+dy, rx+dx
				}
			}
		}
	}

	part1 := len(antinodes)
	fmt.Println("part1: ", part1)

	part2 := len(antinodes2)
	fmt.Println("part2: ", part2)
}
