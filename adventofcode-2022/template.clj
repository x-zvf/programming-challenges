(ns dayXX)
(require 'clojure.string)

(def test-input "")

(def real-input (slurp "./inputs/dayXX.txt"))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    ))

(defn part1 [input]
  (->> input
       parse-input))

(defn part2 [input]
  (->> input
       parse-input))

(part1 test-input)
(part1 real-input)
(part2 test-input)
(part2 real-input)
