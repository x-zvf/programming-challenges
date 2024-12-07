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

// solve recursively with dp, checking if the last element may be part of
// the solution.
func solveable(target int, vals []int, withconcat bool) bool {
	last := vals[len(vals)-1]

	if len(vals) == 1 {
		return last == target
	}

	rest := vals[:len(vals)-1]

	if withconcat {
		ndigit := int(math.Log10(float64(last))) + 1
		p10 := int(math.Pow10(ndigit))
		tdigits := target % p10

		if tdigits == last {
			if solveable(target/p10, rest, true) {
				return true
			}
		}
	}

	if target%last == 0 {
		if solveable(target/last, rest, withconcat) {
			return true
		}
	}
	return solveable(target-last, rest, withconcat)

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

	part1 := 0
	part2 := 0
	for sc.Scan() {
		line := sc.Text()
		targetStr, numbersStr, _ := strings.Cut(line, ": ")
		target, err := strconv.Atoi(targetStr)
		if err != nil {
			log.Fatal("failed to parse target")
		}
		numberStrs := strings.Split(numbersStr, " ")

		var nums []int
		for _, s := range numberStrs {
			n, err := strconv.Atoi(s)
			if err != nil {
				log.Fatal("failed to parse number ", s)
			}
			nums = append(nums, n)
		}
		if solveable(target, nums, false) {
			part1 += target
			part2 += target // no need to check twice
		} else if solveable(target, nums, true) {
			part2 += target
		}
	}
	fmt.Println("Part 1: ", part1)
	fmt.Println("Part 2: ", part2)

}
