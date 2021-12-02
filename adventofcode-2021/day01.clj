(ns day01)

(def test-input [199
                 200
                 208
                 210
                 200
                 207
                 240
                 269
                 260
                 263])

(def real-input
  (map #(Integer/parseInt %) (clojure.string/split (slurp "./inputs/day01.txt") #"\n")))

(defn part1 [input]
  (apply + (map #(if (< (nth % 0) (nth % 1)) 1 0)
                (partition 2 1 input))))

(part1 test-input);; => 7
(part1 real-input);; => 1616

(defn part2 [input]
    (part1 (map #(apply + %) (partition 3 1 input))))

(part2 test-input);; => 5
(part2 real-input);; => 1645
