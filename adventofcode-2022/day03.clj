(ns day03)
(require 'clojure.string)
(require 'clojure.set)


(def test-input "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
")

(def real-input (slurp "./inputs/day03.txt"))

(defn parse-input [input]
  (clojure.string/split input  #"\n"))

(defn split-in-half [s]
    (let [sidelen (/ (count s) 2)]
      [(take sidelen s)
       (take-last sidelen s)]))

(defn priority [x]
  (let [a (int \a) A (int \A) v (int x) d (- v a) D (- v A)]
    (if (>= d 0) (+ 1 d)
        (+ 27 D))))

(defn duplicate [xs]
    (apply clojure.set/intersection
         (map set xs)))

(defn solve [f input]
    (->> input
       parse-input
       f
       (map duplicate)
       (map vec)
       flatten
       (map priority)
       (apply +)))

(defn part1 [input]
  (solve #(map split-in-half %) input))

(defn part2 [input]
  (solve #(partition 3 %) input))

(part1 test-input);; => 157
(part1 real-input);; => 8202
(part2 test-input);; => 70
(part2 real-input);; => 2864
