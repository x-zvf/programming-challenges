(ns day15)
(require 'clojure.string)

(def test-input "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3")

(def real-input (slurp "./inputs/day15.txt"))

(defn manhattan-distance [[ay ax] [by bx]]
  (+ (abs (- ax bx)) (abs (- ay by))))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map (partial re-seq #"(-?\d+)") x)
    (map (fn [matches]
           (->> matches
                (map first)
                flatten
                (map parse-long)
                ((fn [[px py bx by]]
                   {:position [py px]
                    :beacon [by bx]
                    :distance (manhattan-distance [py px] [by bx])})))) x)
    ))

(defn in-range? [sensors pos]
           (loop [[s & rest] sensors]
             (cond (nil? s) false
                   (<= (manhattan-distance (:position s) pos)
                       (:distance s)) true
                   :else (recur rest))))

(defn part1 [y input]
  (let [sensors (parse-input input)
        occupied (reduce (fn [s sensor]
                           (conj s
                                 (:position sensor)
                                 (:beacon sensor)))
                         #{}
                         sensors)
        max-sensor-range (apply max (map :distance sensors))
        min-x (apply min (map #(second (:position %)) sensors))
        max-x (apply max (map #(second (:position %)) sensors))
        from-x (- min-x max-sensor-range)
        to-x (+ max-x max-sensor-range)
        ]
    (loop [x from-x n 0]
      (cond (> x to-x) n
            (contains? occupied [y x]) (recur (inc x) n)
            (in-range? sensors [y x]) (recur (inc x) (inc n))
            :else (recur (inc x) n)))
    ))


(defn part2 [max-ord input]
  "brute force I am too embarrassed to share")
(part1 10 test-input);; => 26
(part1 2000000 real-input);; => 5108096
(part2 20 test-input) ;; => 56000011
(part2 4000000 real-input);; => 10553942650264 (Took 15min)
