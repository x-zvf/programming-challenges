(ns day01)
(require 'clojure.string)

(def test-input "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")

(def real-input (slurp "./inputs/day01.txt"))

(defn sum-elve [elve]
  (map (fn [x]
         (->> (clojure.string/split x #"\n")
              (map #(Integer/parseInt %))
              (reduce +))) elve))

(defn parse-input [input]
  (sum-elve (clojure.string/split input #"\n\n")))

(defn part1 [input]
  (->> input
       parse-input
       (apply max)))

(defn part2 [input]
  (->> input
       parse-input
       sort
       reverse
       (take 3)
       (reduce +)))

(part1 test-input);; => 24000
(part1 real-input);; => 69289

(part2 test-input);; => 45000
(part2 real-input);; => 205615