(ns day02)
(require 'clojure.string)

(def test-input "A Y
B X
C Z
")

(def real-input (slurp "./inputs/day02.txt"))

(defn parse-input [input]
  (->>(clojure.string/split input #"\n")
      (map (fn [x] (clojure.string/split x #" ")))))

(defn choice-score [x]
  (cond (= x "A") 1
        (= x "B") 2
        (= x "C") 3))

(defn beats? [a b]
  (cond (and (= a "A") (= b "B")) true
        (and (= a "B") (= b "C")) true
        (and (= a "C") (= b "A")) true
        :else false))

(defn to-a [x]
  (cond (= x "X") "A"
        (= x "Y") "B"
        (= x "Z") "C"))

(defn round-score [[a b]]
(cond (= a b) 3
      (beats? a b) 6
      :else 0))

(defn score [ilist]
  (apply + (map (fn [[a b]] (+ (round-score [a b])
                      (choice-score b)))
       (map (fn [[a b]] [a (to-a b)]) ilist))))

(defn part1 [input]
  (-> input
      parse-input
      score))

(defn choose-x [[a b]]
  (cond (= b "X") (cond (= a "A") "Z"
                        (= a "B") "X"
                        (= a "C") "Y")
        (= b "Y") (cond (= a "A") "X"
                        (= a "B") "Y"
                        (= a "C") "Z")
        (= b "Z") (cond (= a "A") "Y"
                        (= a "B") "Z"
                        (= a "C") "X")))

(defn part2 [input]
  (->> input
       parse-input
       (map (fn [[a b]] [a (choose-x [a b])]))
       score))

(part1 test-input);; => 15
(part1 real-input);; => 13675
(part2 test-input);; => 12
(part2 real-input);; => 14184

