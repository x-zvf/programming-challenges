package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func solve(s string) (sumign int, sumdo int) {
	re := regexp.MustCompile(`(mul\((?P<a1>\d{1,3}),(?P<a2>\d{1,3})\)|do\(\)|don't\(\))`)
	res := re.FindAllStringSubmatch(s, -1)
	sumdo = 0
	sumign = 0
	en := true
	for _, v := range res {
		if strings.HasPrefix(v[1], "mul") {
			l, e1 := strconv.Atoi(v[2])
			r, e2 := strconv.Atoi(v[3])
			if e1 != nil || e2 != nil {
				log.Fatal("Failed to parse: ", v[1], v[2])
			}
			if en {
				sumdo += l * r
			}
			sumign += l * r
		} else if strings.HasPrefix(v[1], "don't") {
			en = false
		} else { //do
			en = true
		}
	}
	return
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day02 <FILE>")
		os.Exit(1)
	}
	b, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	s := string(b)
	p1, p2 := solve(s)
	fmt.Println("part1: ", p1)
	fmt.Println("part2: ", p2)
}
