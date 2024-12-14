package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Machine struct {
	ax, ay, bx, by, tx, ty int
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day13 <FILE>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	re := regexp.MustCompile(`(\d+).+Y.(\d+)`)
	machines := make([]Machine, 0)
	i := 0
	m := Machine{}
	for sc.Scan() {
		line := sc.Text()
		line = strings.Trim(line, "\n\r\t ")
		if i == 3 {
			machines = append(machines, m)
			m = Machine{}
			i = 0
			continue
		}

		res := re.FindStringSubmatch(line)
		s1, s2 := res[1], res[2]
		n1, err := strconv.Atoi(s1)
		n2, err2 := strconv.Atoi(s2)
		if err != nil || err2 != nil {
			fmt.Println("failed to parse ", s1, s2)
		}
		switch i {
		case 0:
			m.ax = n1
			m.ay = n2
			break
		case 1:
			m.bx = n1
			m.by = n2
			break
		case 2:
			m.tx = n1
			m.ty = n2
			break
		}
		i += 1
	}

	calc := func(m Machine, o int) int {
		x, y := m.tx+o, m.ty+o
		ac := (m.bx*y - m.by*x) / (m.bx*m.ay - m.by*m.ax)
		bc := (x - ac*m.ax) / m.bx
		if (ac*m.ax+bc*m.bx == x) && (ac*m.ay+bc*m.by == y) {
			return ac*3 + bc
		}
		return 0
	}
	part1 := 0
	part2 := 0
	for _, m := range machines {
		part1 += calc(m, 0)
		part2 += calc(m, 10_000_000_000_000)
	}
	fmt.Println(part1)
	fmt.Println(part2)
}
