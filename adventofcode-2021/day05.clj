(ns day05)

(def test-input "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2")

(def real-input (slurp "./inputs/day05.txt"))

(defn parse-input [in]
  (->> in
       clojure.string/split-lines
       (map (fn [s] (let [[x1 y1 x2 y2] (map read-string
                                             (rest
                                              (re-find #"(\d+),(\d+) \-> (\d+),(\d+)" s)))]
                      (list (list x1 y1) (list x2 y2))
                      )))))

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

(defn solve
  "for part 1 `with-diagonals` should be `false`, and `true` for part 2"
  [with-diagonals input]
  (->> input
       parse-input
       (draw-lines with-diagonals)
       (filter (fn [[_ v]] (< 1 v)))
       count))

(solve false test-input);; => 5
(solve false real-input);; => 3990

(solve true test-input);; => 12
(solve true real-input);; => 21305
