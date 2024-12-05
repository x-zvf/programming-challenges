package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

// hacky way to create a map key (12,34) -> 1234 as all a,b are <=99
func combine(a, b int) int {
	return a*100 + b
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

	order := make(map[int]bool)
	var inputs [][]int

	parsingRules := true
	for sc.Scan() {
		line := sc.Text()
		line = strings.Trim(line, " \n\r\t")

		if len(line) == 0 {
			parsingRules = false
		} else if parsingRules {
			l, r, f := strings.Cut(line, "|")
			if !f {
				log.Fatal("parse error in rules")
			}
			a, erra := strconv.Atoi(l)
			b, errb := strconv.Atoi(r)
			if erra != nil || errb != nil {
				log.Fatal("Failed to parse l or r")
			}
			order[combine(a, b)] = true
			order[combine(b, a)] = false
		} else {
			ss := strings.Split(line, ",")
			if len(ss)%2 == 0 {
				log.Fatal("input length must be odd")
			}
			var is []int
			for _, s := range ss {
				i, err := strconv.Atoi(s)
				if err != nil {
					log.Fatal("Failed to parse int")
				}
				if i > 99 {
					log.Fatal("combine only works with i <=99")
				}
				is = append(is, i)
			}
			inputs = append(inputs, is)
		}
	}

	cmpf := func(a, b int) int {
		if a == b {
			return 0
		}
		lt, ok := order[combine(a, b)]
		if !ok {
			log.Fatal("no rule for comparing ", a, b)
		} else if lt {
			return -1
		}
		return 1
	}
	se := 0
	sne := 0
	for _, is := range inputs {
		cpy := make([]int, 0)
		cpy = append(cpy, is...)

		slices.SortFunc(cpy, cmpf)
		mid := cpy[(len(cpy)-1)/2]

		eq := true
		for i := range is {
			if is[i] != cpy[i] {
				eq = false
				break
			}
		}
		if eq {
			se += mid
		} else {
			sne += mid
		}
	}
	fmt.Println("part1: ", se)
	fmt.Println("part2: ", sne)
}
