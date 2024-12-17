package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func run(prog []int8, A, B, C int) (output []int8) {
	IP := 0
	output = make([]int8, 0)
	for IP >= 0 && IP < len(prog) {
		opcode := prog[IP]
		literal := int(prog[IP+1])
		combo := literal
		switch combo {
		case 4:
			combo = A
			break
		case 5:
			combo = B
			break
		case 6:
			combo = C
			break
		}

		IP += 2
		switch opcode {
		case 0: //adv
			A = A / (1 << combo)
			break
		case 1: //bxl
			B = B ^ literal
			break
		case 2: //bst
			B = combo % 8
			break
		case 3: //jnz
			if A != 0 {
				IP = literal
			}
			break
		case 4: //bxc
			B = B ^ C
			break
		case 5: //out
			output = append(output, int8(combo%8))
			break
		case 6: //bdv
			B = A / (1 << combo)
			break
		case 7: //cdv
			C = A / (1 << combo)
			break
		}

	}
	return
}
func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day17 <FILE>")
		os.Exit(1)
	}
	bytes, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal("failed to read")
	}
	input := strings.Split(string(bytes), "\n")
	parseReg := func(str string) (v int) {
		v, err := strconv.Atoi(strings.Trim(str, "Register ABC:"))
		if err != nil {
			log.Fatal("failed to parse register")
		}
		return
	}
	A, B, C := parseReg(input[0]), parseReg(input[1]), parseReg(input[2])
	progstr := strings.Split(strings.Trim(input[4], "Program: \t\r\n"), ",")

	prog := make([]int8, len(progstr))
	for i, s := range progstr {
		v, _ := strconv.Atoi(s)
		prog[i] = int8(v)
	}

	p1o := run(prog, A, B, C)
	part1 := ""
	for i, v := range p1o {
		part1 += fmt.Sprint(v)
		if i != len(p1o)-1 {
			part1 += ","
		}
	}
	fmt.Println("part1: ", part1)

	/*
		// find length
		i := 1
		l := 0
		for {
			i = 1 << l
			o := run(prog, i, 0, 0)
			if len(o) == len(prog) {
				test := run(prog, i-1, 0, 0)
				if len(test) >= len(o) {
					log.Fatal("len does not increase at power of 2 boundary")
				}
				break
			}
			l += 1
		}
		fmt.Println("found min for len at 2 **", l, "=", i)

		//prefixmatchn := 6
		maxprefixmatch := 0
		firstmaxpref := 0
		lastmatch := 0
		prevdelta := 0
		step := 8
		nmatching := 0
		for {
			o := run(prog, i, 0, 0)
			found := true
			x := 0
			for ; found && x < len(prog); x++ {
				found = found && (prog[x] == o[x])
			}
			x -= 1
			if x > maxprefixmatch {
				maxprefixmatch = x
				firstmaxpref = i
				fmt.Println("matching prefix len =", x, ":", o, "i =", i, "delta:", i-firstmaxpref, i-lastmatch)
			}
			if x >= maxprefixmatch {
				if i-lastmatch == prevdelta {
					nmatching += 1
				}

				if prevdelta > step && nmatching > 5 {
					fmt.Println("five matches in a row; increasing step to:", prevdelta)
					step = prevdelta
					nmatching = 0
				}
				prevdelta = i - lastmatch
				lastmatch = i
			}
			i += step
		}
	*/
}
