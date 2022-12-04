(ns day04)
(require 'clojure.string)

(def test-input "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8")

(def real-input (slurp "./inputs/day04.txt"))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map #(re-matches #"(\d+)-(\d+),(\d+)-(\d+)" %) x)
    (map #(map read-string (take-last 4 %)) x)))

(defn fully-contains? [[s1 e1 s2 e2]]
  (or (and (<= s1 s2) (>= e1 e2))
      (and (<= s2 s1) (>= e2 e1))))

(defn overlaps? [[s1 e1 s2 e2]]
  (or (<= s1 s2 e1)
      (<= s2 s1 e2)))

(defn solve [f input]
  (->> input
       parse-input
       (map f)
       (filter identity)
       count))

(defn part1 [input]
  (solve fully-contains? input))

(defn part2 [input]
  (solve overlaps? input))

(part1 test-input);; => 2
(part1 real-input);; => 515
(part2 test-input);; => 4
(part2 real-input);; => 883
