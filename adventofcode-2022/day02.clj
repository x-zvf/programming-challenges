(ns day02)
(require 'clojure.string)

(def test-input "A Y
B X
C Z
")

(def real-input (slurp "./inputs/day02.txt"))

(defn abcxyz-to-num [[a x]]
  [(cond (= a "A") 1
         (= a "B") 2
         :else 3)
   (cond (= x "X") 1
         (= x "Y") 2
         :else 3)])

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map #(clojure.string/split % #" ") x)
    (map abcxyz-to-num x)))

(defn beats? [a b]
  (or (and (= a 1) (= b 2))
      (and (= a 2) (= b 3))
      (and (= a 3) (= b 1))))

(defn result-score [a b]
(cond (= a b) 3
      (beats? a b) 6
      :else 0))

(defn round-score [[a b]]
  (+ b (result-score a b)))

(defn score [rounds]
  (reduce + (map round-score rounds)))

(defn part1 [input]
  (-> input
      parse-input
      score))

(defn choose-x [[a b]]
  (cond (= b 1) (cond (= a 1) 3
                      (= a 2) 1
                      (= a 3) 2)
        (= b 2) a
        (= b 3) (cond (= a 1) 2
                      (= a 2) 3
                      (= a 3) 1)))

(defn part2 [input]
  (->> input
       parse-input
       (map (fn [[a b]] [a (choose-x [a b])]))
       score))

(part1 test-input);; => 15
(part1 real-input);; => 13675
(part2 test-input);; => 12
(part2 real-input);; => 14184
