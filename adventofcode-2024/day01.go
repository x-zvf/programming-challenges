package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day01 <FILE>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)
	var left, right []int
	for sc.Scan() {
		line := sc.Text()
		parts := strings.Split(line, "   ")
		if len(parts) != 2 {
			continue
		}
		li, errl := strconv.Atoi(parts[0])
		ri, errr := strconv.Atoi(parts[1])
		if errl != nil || errr != nil {
			log.Fatalf("failed to parse")
		}
		left = append(left, li)
		right = append(right, ri)
	}
	sort.Ints(left)
	sort.Ints(right)

	sum := 0
	for i := range left {
		l := left[i]
		r := right[i]
		delta := max(l, r) - min(l, r)
		sum += delta
	}
	fmt.Println("part1: ", sum)

	freq := make(map[int]int)
	for _, v := range right {
		freq[v] += 1
	}

	similarity := 0
	for _, v := range left {
		similarity += v * freq[v]
	}
	fmt.Println("part2: ", similarity)
}
