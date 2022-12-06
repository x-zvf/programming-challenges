(ns day06)
(require 'clojure.string)

(def test-input "mjqjpqmgbljsphdztnvjfqwrcgsmlb")
(def real-input (slurp "./inputs/day06.txt"))

(defn solve [n input]
  (->> input
       (partition n 1)
       (map #(= n (count (set %))))
       ((fn [x]
          (loop [idx 0 data x]
            (if (first data) idx
                (recur (+ 1 idx) (drop 1 data))))))
       (+ n)))

(defn part1 [input] (solve 4 input))
(defn part2 [input] (solve 4 input))

(part1 test-input);; => 7
(part1 real-input);; => 1140
(part2 test-input);; => 19
(part2 real-input);; => 3495

