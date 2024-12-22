package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func step(i int) int {
	i ^= (i << 6) & 0xFFFFFF
	i ^= (i >> 5) & 0xFFFFFF
	i ^= (i << 11) & 0xFFFFFF
	return i
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day22 <FILE>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	enc := func(x []int) int {
		return 1000000*x[0] + 10000*x[1] + 100*x[2] + x[3]
	}

	part1 := 0
	bananas := make(map[int]int, 0)
	for sc.Scan() {
		line := sc.Text()
		n, _ := strconv.Atoi(strings.Trim(line, " \r\n\t"))
		var ns [2001]int
		var ds [2001]int
		ns[0] = n
		for i := range 2000 {
			n = step(n)
			ns[i+1] = n
			ds[i] = (n % 10) - (ns[i] % 10)
		}
		part1 += n

		visited := make(map[int]bool, 0)
		for i := range 1997 {
			v := enc(ds[i : i+4])
			if visited[v] {
				continue
			}
			visited[v] = true
			bananas[v] += ns[i+4] % 10
		}
	}
	fmt.Println("part1: ", part1)

	m := -1
	for _, v := range bananas {
		if v > m {
			m = v
		}
	}
	fmt.Println("part2: ", m)
}
