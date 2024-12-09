package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func compact(fs []int) {
	freeidx := 0
	lastidx := len(fs) - 1
	for freeidx < lastidx && lastidx >= 0 {
		for fs[freeidx] != -1 {
			freeidx += 1
		}
		if freeidx >= lastidx {
			break
		}
		if fs[lastidx] != -1 {
			fs[freeidx], fs[lastidx] = fs[lastidx], -1
			freeidx -= 1
		}
		if lastidx >= 0 && fs[lastidx] == -1 {
			lastidx -= 1
		}
	}
}
func compact2(fs []int) {
	dataptr := len(fs) - 1
	datalen := 1
	for fs[dataptr+datalen-1] == fs[dataptr-1] && dataptr > 0 {
		datalen += 1
		dataptr -= 1
	}

	for dataptr >= 0 {
		freeptr := 0
		freelen := 0
		for freelen < datalen {
			freeptr += freelen
			freelen = 0
			for fs[freeptr] != -1 {
				freeptr += 1
			}
			for freeptr+freelen < len(fs) && fs[freeptr+freelen] == -1 {
				freelen += 1
			}
			if freeptr >= dataptr {
				break
			}
		}
		if freeptr < dataptr {
			for i := 0; i < datalen; i++ {
				fs[freeptr+i], fs[dataptr+i] = fs[dataptr+i], -1
			}
			freeptr += datalen
		}

		datalen = 1
		dataptr -= 1
		for dataptr >= 0 && fs[dataptr] == -1 {
			dataptr -= 1
		}
		for dataptr > 0 && fs[dataptr+datalen-1] == fs[dataptr-1] {
			datalen += 1
			dataptr -= 1
		}
	}

}

func checksum(fs []int) int {
	cs := 0
	for i, v := range fs {
		if v > 0 {
			cs += i * v
		}
	}
	return cs
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ./day09 <FILE>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	sc.Scan()
	line := sc.Text()
	fs := make([]int, 0)
	free := false
	fid := 0
	for _, c := range line {
		d, err := strconv.Atoi(string(c))
		if err != nil {
			log.Fatal("invalid digit")
		}
		for i := 0; i < d; i++ {
			x := -1
			if !free {
				x = fid
			}
			fs = append(fs, x)
		}
		if !free {
			fid++
		}
		free = !free
	}
	p1fs := make([]int, len(fs))
	copy(p1fs, fs)
	compact(p1fs)

	compact2(fs)

	fmt.Println("part1", checksum(p1fs))
	fmt.Println("part2", checksum(fs))
}
