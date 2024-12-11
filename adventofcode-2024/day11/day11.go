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

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: ./day11 <FILE> <count>")
		os.Exit(1)
	}
	count, err := strconv.Atoi(os.Args[2])
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	sc.Scan()
	line := sc.Text()
	arr := strings.Split(line, " ")

	xs := make([]int, len(arr))
	for i, s := range arr {
		x, err := strconv.Atoi(s)
		if err != nil {
			log.Fatal("failed to parse ", s)
		}
		xs[i] = x
	}

	stonecounts := make(map[int]int, 0)
	for _, x := range xs {
		stonecounts[x] += 1
	}
	for range count {
		nsc := make(map[int]int, 0)
		for x, c := range stonecounts {
			if x == 0 {
				nsc[1] += c
			} else if ndigit := int(math.Log10(float64(x))) + 1; ndigit%2 == 0 {
				p10 := int(math.Pow10(ndigit / 2))
				left := x / p10
				right := x % p10
				nsc[left] += c
				nsc[right] += c
			} else {
				nsc[2024*x] += c
			}
		}
		stonecounts = nsc
	}
	stones := 0
	for _, c := range stonecounts {
		stones += c
	}
	fmt.Println(stones)
}
