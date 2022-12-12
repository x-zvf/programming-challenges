(ns day12
(:require [clojure.string]
           [loom.graph]
           [loom.alg]))

(def test-input "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi")

(def real-input (slurp "./inputs/day12.txt"))

(defn get-navigable-neighbours [matrix height width [y x]]
  (->> [[(+ y 1) (+ x 0)]
        [(- y 1) (+ x 0)]
        [(+ y 0) (+ x 1)]
        [(+ y 0) (- x 1)]]
       (filter (fn [[y x]]
                 (and (>= x 0)
                      (>= y 0)
                      (< y height)
                      (< x width))))
       (map (fn [coord] [(:height (get-in matrix coord)) coord]))
       (filter (fn [[c coord]]
                 (<= c (+ 1 (:height (get-in matrix [y x]))))))
       (map (fn [[_ coord]] coord))))

(defn get-index [f matrix]
  (for [[y row] (map-indexed vector matrix)
               [x v] (map-indexed vector row)
               :when (f v)]
           [y x]))


(defn parse-input [input]
  (let [grid
        (as-> input z
          (clojure.string/split z #"\n")
          (mapv
           (fn [line]
             (->> line
                  seq
                  (mapv
                   (fn [c]
                     (cond (= c \S) {:height 0 :start true}
                           (= c \E) {:height 25 :end true}
                           :else {:height (- (int c) (int \a))})))))
           z))
        height (count grid)
        width (count (first grid))]
    {:graph (apply loom.graph/digraph (for [[y row] (map-indexed vector grid)
          [x point] (map-indexed vector row)]
      (let [neighbours (get-navigable-neighbours grid height width [y x])]
        {[y x] neighbours})))
     :start (first (get-index #(:start %) grid))
     :end (first (get-index #(:end %) grid))
     :lowest (get-index #(= 0 (:height %)) grid)
     }))


(defn part1 [input]
    (let [{:keys [start end graph]} (parse-input input)]
      (count (drop 1 (loom.alg/bf-path graph start end)))
      ))

(defn part2 [input]
  (let [{:keys [lowest end graph]} (parse-input input)
        paths (map (fn [start]
                            (loom.alg/bf-path graph start end))
                   lowest)]
    (-  (count (first (sort-by count (filter #(not (nil? %)) paths)))) 1)
    ))

(part1 test-input);; => 31
(part1 real-input);; => 520
(part2 test-input);; => 29
(part2 real-input) ;; => 508
