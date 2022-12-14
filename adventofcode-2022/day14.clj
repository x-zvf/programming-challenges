(ns day14)
(require 'clojure.string)

(def test-input "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9")

(def real-input (slurp "./inputs/day14.txt"))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map #(clojure.string/split % #" -> ") x)
    (map #(partition 2 1 %) x)
    (apply concat x)
    (map (fn [[start end]]
           (let [[x1 y1] (map parse-long (clojure.string/split start #","))
                 [x2 y2] (map parse-long (clojure.string/split end #","))
                 [sx ex] (sort [x1 x2])
                 [sy ey] (sort [y1 y2])
                 ]
             (if (= sx ex)
               (for [y (range sy (inc ey))] [y sx])
               (for [x (range sx (inc ex))] [sy x])))) x)
    (apply concat x)
    (set x)))

(defn deposit [pointset floor-level [y x]]
  (cond
    (and (= floor-level ##Inf) (empty? (filter (fn [[py px]] (and (= px x) (> py y))) pointset)))
    nil
    (and (< (inc y) floor-level) (not (contains? pointset [(inc y) x])))
    (deposit pointset floor-level [(inc y) x])
    (and (< (inc y) floor-level)  (not (contains? pointset [(inc y) (dec x)])))
    (deposit pointset floor-level [(inc y) (dec x)])
    (and (< (inc y) floor-level)  (not (contains? pointset [(inc y) (inc x)])))
    (deposit pointset floor-level [(inc y) (inc x)])
    :else [y x]))

(defn part1 [input]
  (let [pointset (parse-input input)]
    (loop [points pointset n 0]
      (if-let [deposited-at (deposit points ##Inf [0 500])]
        (recur (conj points deposited-at) (inc n))
        n))))

(defn part2 [input]
  (let [pointset (parse-input input)
        floor-level (+ 2 (apply max (map first pointset)))]
    (loop [points pointset n 0]
      (let [deposited-at (deposit points floor-level [0 500])]
        (if (= deposited-at [0 500])
          (inc n)
          (recur (conj points deposited-at) (inc n)))))))

(part1 test-input);; => 24
(part1 real-input);; => 692
(part2 test-input);; => 93
(part2 real-input);; => 31706
