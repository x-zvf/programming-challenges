# Clojure

[full solution](https://github.com/x-zvf/programming-challenges/blob/master/adventofcode-2021/day05.clj)

I really enjoyed today's task!

I'd like to think that today's solution is quite elegant, but as always, feedback and suggestions are very much welcome!

## Parsing the input

Using regex capture-groups, I extract the four numbers that make up each line,
parse them to numbers and then put them into a list lists, with `((x1 y1) (x2 y2))`


    (defn parse-input [in]
      (->> in
           clojure.string/split-lines
           (map (fn [s] (let [[x1 y1 x2 y2] (map read-string
                                                 (rest
                                                  (re-find #"(\d+),(\d+) \-> (\d+),(\d+)" s)))]
                          (list (list x1 y1) (list x2 y2))
                          )))))

## Drawing a line

To draw (interpolate all the integer coordinates of the points on the line) a line,
I first determine if the line is vertical, horizontal or diagonal.

To make things easier, I also determine the smaller and the larger x and y coordinates.

Horizontal and vertical lines are pretty easy to draw, they are simply all points with x or y constant respectively,
and x or y in the range \[sx, lx\] or \[sy, ly\].
For diagonal lines, the points are x1 + or - delta and y1 + or - delta, with delta being |x1-x2|

    (defn generate-line-coords
      "Generates all the points along the line (2nd argument). Ignores diagonals if with-diagonals
      is false"
      [with-diagonals [[x1 y1],[x2 y2]]]
      (let [sx (min x1 x2) lx (max x1 x2)  ; smaller and larger x coordinate
            sy (min y1 y2) ly (max y1 y2)] ; smaller and larger y coordinate
        (cond
          (= sx lx) (for [y (range sy (+ 1 ly))] (list sx y)) ; vertical
          (= sy ly) (for [x (range sx (+ 1 lx))] (list x sy)) ; horizontal
          :else (if with-diagonals
                  (if (not= (- lx sx) (- ly sy))
                    (throw (new IllegalArgumentException "Only 45 degree lines are supported"))
                    (for [delta (range 0 (+ 1 (- lx sx)))]
                      (list (if (< x1 x2) (+ x1 delta) (- x1 delta))
                            (if (< y1 y2) (+ y1 delta) (- y1 delta)))))
                  '())
          )))

## Drawing multiple lines

I use a map to contain store how many lines are on a specific point.
For each point on each line, we increment the value of the point.


    (defn draw-lines
      "Takes a list of lines and produces a map with the keys being coordinates and the
      values being the number of lines that cover that point"
      [with-diagonals lines]
      (reduce (fn [m line]
                (reduce (fn [sm coord]
                          (update-in sm [coord] #(inc (or % 0))))
                        m
                        (generate-line-coords with-diagonals line)))
              {}
              lines))

## Solution

Bringing this all together, we draw all the lines, then filter out only those that overlap (value > 1), then count them.

     (defn solve
       "for part 1 `with-diagonals` should be `false`, and `true` for part 2"
       [with-diagonals input]
       (->> input
            parse-input
            (draw-lines with-diagonals)
            (filter (fn [[_ v]] (< 1 v)))
            count))

### Part 1

    (solve false test-input);; => 5
    (solve false real-input);; => 3990

### Part 2

    (solve true test-input);; => 12
    (solve true real-input);; => 2130