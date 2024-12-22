package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// as maths profs would say: "durch scharfes hinsehen"
var LUT = map[string]string{
	"00": "A",
	"01": "^<A",
	"02": "^A",
	"03": "^>A",
	"04": "^<^A",
	"05": "^^A",
	"06": "^^>A",
	"07": "^^^<A",
	"08": "^^^A",
	"09": "^^^>A",
	"0A": ">A",
	"10": ">vA",
	"11": "A",
	"12": ">A",
	"13": ">>A",
	"14": "^A",
	"15": "^>A",
	"16": "^>>A",
	"17": "^^A",
	"18": "^^>A",
	"19": "^^>>A",
	"1A": ">>vA",
	"20": "vA",
	"21": "<A",
	"22": "A",
	"23": ">A",
	"24": "<^A",
	"25": "^A",
	"26": "^>A",
	"27": "<^^A",
	"28": "^^A",
	"29": "^^>A",
	"2A": "v>A",
	"30": "<vA",
	"31": "<<A",
	"32": "<A",
	"33": "A",
	"34": "<<^A",
	"35": "<^A",
	"36": "^A",
	"37": "<<^^A",
	"38": "<^^A",
	"39": "^^A",
	"3A": "vA",
	"40": ">vvA",
	"41": "vA",
	"42": "v>A",
	"43": "v>>A",
	"44": "A",
	"45": ">A",
	"46": ">>A",
	"47": "^A",
	"48": "^>A",
	"49": "^>>A",
	"4A": ">>vvA",
	"50": "vvA",
	"51": "<vA",
	"52": "vA",
	"53": "v>A",
	"54": "<A",
	"55": "A",
	"56": ">A",
	"57": "<^A",
	"58": "^A",
	"59": "^>A",
	"5A": "vv>A",
	"60": "<vvA",
	"61": "<<vA",
	"62": "<vA",
	"63": "vA",
	"64": "<<A",
	"65": "<A",
	"66": "A",
	"67": "<<^A",
	"68": "<^A",
	"69": "^A",
	"6A": "vvA",
	"70": ">vvvA",
	"71": "vvA",
	"72": "vv>A",
	"73": "vv>>A",
	"74": "vA",
	"75": "v>A",
	"76": "v>>A",
	"77": "A",
	"78": ">A",
	"79": ">>A",
	"7A": ">>vvvA",
	"80": "vvvA",
	"81": "<vvA",
	"82": "vvA",
	"83": "vv>A",
	"84": "<vA",
	"85": "vA",
	"86": "v>A",
	"87": "<A",
	"88": "A",
	"89": ">A",
	"8A": "vvv>A",
	"90": "<vvvA",
	"91": "<<vvA",
	"92": "<vvA",
	"93": "vvA",
	"94": "<<vA",
	"95": "<vA",
	"96": "vA",
	"97": "<<A",
	"98": "<A",
	"99": "A",
	"9A": "vvvA",
	"<<": "A",
	"<>": ">>A",
	"<A": ">>^A",
	"<^": ">^A",
	"<v": ">A",
	"><": "<<A",
	">>": "A",
	">A": "^A",
	">^": "<^A",
	">v": "<A",
	"A0": "<A",
	"A1": "^<<A",
	"A2": "<^A",
	"A3": "^A",
	"A4": "^^<<A",
	"A5": "<^^A",
	"A6": "^^A",
	"A7": "^^^<<A",
	"A8": "<^^^A",
	"A9": "^^^A",
	"A<": "v<<A",
	"A>": "vA",
	"AA": "A",
	"A^": "<A",
	"Av": "<vA",
	"^<": "v<A",
	"^>": "v>A",
	"^A": ">A",
	"^^": "A",
	"^v": "vA",
	"v<": "<A",
	"v>": ">A",
	"vA": "^>A",
	"v^": "^A",
	"vv": "A",
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day21 <FILE>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	memo := make([]map[string]int, 0)
	for range 26 {
		memo = append(memo, make(map[string]int, 0))
	}
	var calc func(s string, level int) int
	calc = func(s string, level int) int {
		if level == 0 {
			return len(s)
		}

		if v, ok := memo[level-1][s]; ok {
			return v
		}

		res := 0
		p := 'A'
		for _, c := range s {
			v := LUT[string(p)+string(c)]
			res += calc(v, level-1)
			p = c
		}

		memo[level-1][s] = res
		return res
	}
	part1 := 0
	part2 := 0
	for sc.Scan() {
		line := sc.Text()
		code := strings.Trim(line, " \r\n\t")
		num, _ := strconv.Atoi(strings.Trim(line, "A \r\n\t"))
		res1 := calc(code, 3)
		res2 := calc(code, 26)
		part1 += res1 * num
		part2 += res2 * num

	}
	fmt.Println("part1: ", part1)
	fmt.Println("part2: ", part2)
}
