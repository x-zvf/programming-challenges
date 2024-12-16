package main

import (
	"bufio"
	"fmt"
	"image"
	"image/png"
	"log"
	"os"
	"regexp"
	"strconv"
)

type Robot struct {
	px, py, vx, vy int
}

func mod(a, b int) int {
	return (a%b + b) % b
}

func main() {
	if len(os.Args) < 4 {
		fmt.Println("Usage: ./day14 <FILE> <w> <h>")
		os.Exit(1)
	}
	fd, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal("failed to open")
	}
	sc := bufio.NewScanner(fd)

	w, err := strconv.Atoi(os.Args[2])
	h, err2 := strconv.Atoi(os.Args[3])
	if err != nil || err2 != nil {
		fmt.Println("failed to parse w,h")
	}

	re := regexp.MustCompile(`p=(\d+),(\d+) v=([+\-0-9]+),([+\-0-9]+)`)
	robots := make([]Robot, 0)
	for sc.Scan() {
		line := sc.Text()

		res := re.FindStringSubmatch(line)
		s1, s2, s3, s4 := res[1], res[2], res[3], res[4]
		n1, err := strconv.Atoi(s1)
		n2, err2 := strconv.Atoi(s2)
		n3, err3 := strconv.Atoi(s3)
		n4, err4 := strconv.Atoi(s4)
		if err != nil || err2 != nil || err3 != nil || err4 != nil {
			fmt.Println("failed to parse ", s1, s2, s3, s4)
		}
		r := Robot{n1, n2, n3, n4}
		robots = append(robots, r)
	}

	quadcount := []int{0, 0, 0, 0}
	for _, r := range robots {
		fx := mod((r.px + 100*r.vx), w)
		fy := mod((r.py + 100*r.vy), h)
		if fx < (w/2) && fy < (h/2) {
			quadcount[0] += 1
		} else if fx > (w/2) && fy < (h/2) {
			quadcount[1] += 1
		} else if fx < (w/2) && fy > (h/2) {
			quadcount[2] += 1
		} else if fx > (w/2) && fy > (h/2) {
			quadcount[3] += 1
		}
	}
	fmt.Println(quadcount)
	part1 := quadcount[0] * quadcount[1] * quadcount[2] * quadcount[3]
	fmt.Println("part 1: ", part1)

	img := image.NewGray(image.Rect(0, 0, w, h))
	file, err := os.Create("easteregg.png")
	if err != nil {
		log.Fatal("failed to create image")
	}
	defer file.Close()
	i := 1
	for {
		grid := make([]uint8, w*h)
		valid := true
		for _, r := range robots {
			fx := mod((r.px + i*r.vx), w)
			fy := mod((r.py + i*r.vy), h)
			if grid[fy*w+fx] != 0 {
				valid = false
				break
			}
			grid[fy*w+fx] = 0xff
		}
		if valid {
			img.Pix = grid
			break
		}
		i++
	}
	fmt.Println("part 2: ", i)

	if err := png.Encode(file, img); err != nil {
		log.Fatal("Error encoding image", err)
	}
}
