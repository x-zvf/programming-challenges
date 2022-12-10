(ns day10)
(require 'clojure.string)

(def test-input "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop")

(def real-input (slurp "./inputs/day10.txt"))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map (fn [line]
           (let [[cmd v] (clojure.string/split line #" ")]
             (cond (= cmd "noop")
                   (list identity)
                   (= cmd "addx")
                   (list identity #(+ % (parse-long v)))
                   ))) x)
    (flatten x)
    ))

(defn run-input [input]
  (vec (conj (->> input
            parse-input
            (reductions (fn [x f] (f x)) 1)) 1)))

(defn part1 [input]
  (let [xs (run-input input)
        interesting (range 20 (+ 1 (count xs)) 40)]
    (->> interesting
         (map #(* % (nth xs %)))
         (apply +)
    )))

(defn visible? [xpos x]
  (or (= xpos x)
      (= xpos (+ x 1))
      (= xpos (+ x 2))))

;; FIXME: The last row is not printed..
(defn part2 [input]
  (let [xs (run-input input)
        pxlcount (* 40 6)]
    (loop [res [] line [] idx 0]
      (cond (= idx pxlcount)
            (->> line
                 (conj res)
                 (drop 1)
                 (map #(apply str %))
                 (clojure.string/join "\n"))
            (= 0 (mod idx 40))
            (recur (conj res line) [] (inc idx))
            :else
            (recur res
                   (conj line
                         (if (visible? (mod idx 40) (nth xs idx))
                           \#
                           \.))
                   (inc idx))))))


(part1 test-input)
;; => 13140

(part1 real-input)
;; => 15880

(println (part2 test-input))
;; ##..##..##..##..##..##..##..##..##..##.
;; ###...###...###...###...###...###...###
;; ####....####....####....####....####...
;; #####.....#####.....#####.....#####....
;; ######......######......######......###
;; #######.......#######.......#######....
(println (part2 real-input))
;; ###..#.....##..####.#..#..##..####..##.
;; #..#.#....#..#.#....#.#..#..#....#.#..#
;; #..#.#....#....###..##...#..#...#..#...
;; ###..#....#.##.#....#.#..####..#...#.##
;; #....#....#..#.#....#.#..#..#.#....#..#
;; #....####..###.#....#..#.#..#.####..###