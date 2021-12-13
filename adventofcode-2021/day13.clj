(ns day13)

(def test-input "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5")

(def real-input (slurp "./inputs/day13.txt"))

(defn parse-input [input]
  (let[[points folds] (clojure.string/split input #"\n\n")]
    (list (set (->> points
                    clojure.string/split-lines
                    (map (fn [l] (map read-string (clojure.string/split l #","))))
                    ))
          (->> folds
               clojure.string/split-lines
               (map (fn [l]
                      (let [[dir coord] (-> l
                                            (clojure.string/split #" ")
                                            (nth 2)
                                            (clojure.string/split #"=")
                                            )]
                        (list (keyword dir) (read-string coord))
                        )))
               ))))


(defn folded-coordj[fold-by to-fold]
  (if (> fold-by to-fold)
    to-fold
    (- (* 2 fold-by) to-fold)))

(defn fold-paper [points folds]
  (reduce (fn [paper [direction coord]]
            (set (map (fn [[x y]]
                        (if (= direction :y)
                          (list x (folded-coord coord y))
                          (list (folded-coord coord x) y)
                          )) paper)))
          points folds))

(defn part1 [input]
  (let [[points folds] (parse-input input)]
    (count (fold-paper points [(first folds)]))
    ))

(defn pprint [points]
  (let [max-x (apply max (map #(nth % 0) points))
        max-y (apply max (map #(nth % 1) points))]
    (clojure.string/join
     "\n"
     (for [y (range (+ 1 max-y))]
       (apply str (map (fn [x] (if (contains? points (list x y)) "#" ".")) (range (+ 1 max-x))))))))

(defn part2 [input]
  (let [[points folds] (parse-input input)]
    (pprint (fold-paper points folds))
    ))

(part1 test-input);; => 17
(part1 real-input);; => 712

(part2 test-input)
;; => "
;; #####
;; #...#
;; #...#
;; #...#
;; #####"

(part2 real-input);; => "
;; ###..#....#..#.####...##.###....##.####
;; #..#.#....#..#.#.......#.#..#....#.#...
;; ###..#....####.###.....#.#..#....#.###.
;; #..#.#....#..#.#.......#.###.....#.#...
;; #..#.#....#..#.#....#..#.#....#..#.#...
;; ###..####.#..#.#.....##..#.....##..#..."
;; BLHFJPJF
