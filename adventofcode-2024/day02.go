package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func check(level []int, removeable bool) bool {
	if level[0] == level[1] {
		if removeable {
			return check(level[1:], false)
		} else {
			return false
		}
	}
	incr := level[0] < level[1]
	safe := true
	i := 0
	for i = 0; i < len(level)-1 && safe; i++ {
		l := level[i]
		r := level[i+1]
		safe = safe && ((incr && l < r) || (!incr && l > r)) && math.Abs(float64(l-r)) <= 3
		if removeable && !safe {
			if i == 1 && check(level[1:], false) {
				return true
			}
			var left []int
			left = append(left, level[:i]...)
			left = append(left, level[i+1:]...)
			if check(left, false) {
				return true
			}
			var right []int
			right = append(right, level[:i+1]...)
			right = append(right, level[i+2:]...)
			return check(right, false)
		}
	}
	return safe
}

func solve(levels [][]int, removeable bool) int {
	nsafe := 0
	for _, level := range levels {
		if check(level, removeable) {
			nsafe += 1
		}
	}
	return nsafe
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

	levels := make([][]int, 0)

	for sc.Scan() {
		line := sc.Text()
		parts := strings.Split(line, " ")
		if len(parts) < 2 {
			continue
		}
		level := make([]int, len(parts))
		for i, p := range parts {
			n, err := strconv.Atoi(p)
			if err != nil {
				log.Fatal("Failed to parse: ", p)
			}
			level[i] = n
		}
		levels = append(levels, level)
	}
	fmt.Println("part1: ", solve(levels, false))
	fmt.Println("part2: ", solve(levels, true))
}
