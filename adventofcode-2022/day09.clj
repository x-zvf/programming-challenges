(ns day09)
(require 'clojure.string)
(require 'clojure.math)


(def test-input "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")

(def real-input (slurp "./inputs/day09.txt"))

(def deltas
  {\U [0 -1]
   \D [0 1]
   \L [-1 0]
   \R [1 0]})



(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map (fn [l] (let [[c n] (clojure.string/split l #" ")]
                   (list (first c) (read-string n)))) x)
    (map (fn [[c n]] (repeat n c)) x)
    (flatten x)
    ))

(defn step-head [head move]
  (vec (map + (deltas move) head)))

(defn step-up [[lx ly] [mx my]]
  (let [dx (- lx mx)
        dy (- ly my)]
    (if (> 2 (max (abs dx) (abs dy)))
      [mx my]
      [(+ mx (clojure.math/signum dx))
       (+ my (clojure.math/signum dy))])))

(defn step-tail [head tail]
  (loop [[to-move & rest] tail
         res [head]]
    (if (empty? to-move) res
        (recur rest (conj res (step-up (peek res) to-move))))))

(defn step-rope [[head & tail] move]
  (step-tail (step-head head move) tail))

(defn solve [n moves]
  (loop [rope (vec (repeat n [0 0]))
        touched (set ())
         [move & rest] moves]
    (if
     (nil? move)
      (count (conj touched (peek rope)))
      (recur (step-rope rope move) (conj touched (peek rope)) rest))
    ))

(defn part1 [input]
  (->> input
       parse-input
       (solve 2)))

(defn part2 [input]
  (->> input
       parse-input
       (solve 10)))

(part1 test-input);; => 13
(part1 real-input);; => 6181
(part2 test-input);; => 1
(part2 real-input);; => 2386
