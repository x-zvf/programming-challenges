package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day19 <FILE>")
		os.Exit(1)
	}
	bytes, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal("failed to read")
	}
	input := strings.Split(string(bytes), "\n")

	towels := strings.Split(input[0], ", ")

	memo := make(map[string]int, 0)
	var count func(s string) int
	count = func(s string) int {
		if s == "" {
			return 1
		}
		if c, ok := memo[s]; ok {
			return c
		}

		res := 0
		for _, t := range towels {
			if rest, match := strings.CutPrefix(s, t); match {
				res += count(rest)
			}
		}
		memo[s] = res
		return res
	}
	part1 := 0
	part2 := 0
	for _, line := range input[2:] {
		line = strings.Trim(line, " \r\n\t")
		if len(line) == 0 {
			continue
		}
		c := count(line)
		if c > 0 {
			part1 += 1
		}
		part2 += c
	}
	fmt.Println("part1: ", part1)
	fmt.Println("part2: ", part2)
}
