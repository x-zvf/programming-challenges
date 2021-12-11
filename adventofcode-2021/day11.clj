(ns day11)

(def test-input "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")

(def real-input "6617113584
6544218638
5457331488
1135675587
1221353216
1811124378
1387864368
4427637262
6778645486
3682146745")

(defn parse-input [input]
  (->> input
       clojure.string/split-lines
       (map (fn [l] (map (comp read-string str) (seq l))))
       ))

(defn at [l [x y]]
  (if (or (> 0 x) (> 0 y) (<= 10 x) (<= 10 y))
    nil
    (nth (nth l y) x)))

(defn neighbours [[x y]]
  (set [
        [(- x 1) (- y 1)]
        [x (- y 1)]
        [(+ 1 x) (- y 1)]
        [(- x 1) y]
        [(+ x 1) y]
        [(- x 1) (+ y 1)]
        [x (+ y 1)]
        [(+ x 1) (+ y 1)]]))

(defn inc-levels [i]
  (map (fn [l] (map inc l)) i))

(def all-coords
  (map (fn [n]
         (list (/ (- n (mod n 10)) 10) (mod n 10)))
       (range 100)))

(defn transpose [m]
  (apply mapv vector m))

(defn flash [i]
  (loop [state i n-flashed 0 has-flashed #{}]
    (let [to-flash (filter #(> (at state %) 9) all-coords)]
      (if (empty? to-flash)
        (list n-flashed state)
        (let [flashed (first to-flash) neigh (neighbours (first to-flash))]
          (recur
           (transpose
            (partition
             10
             (map (fn [p]
                    (cond (or (= p flashed) (contains? has-flashed p)) 0
                          (contains? neigh p) (inc (at state p))
                          :else (at state p)
                          ))
                  all-coords)))
           (inc n-flashed)
           (conj has-flashed flashed)
           ))))))

(defn flashes-after-steps [steps inp]
  (loop [n steps c 0 i inp]
    (if (= 0 n)
      c
      (let [[flashed state] (flash2 (inc-levels i))]
        (recur (- n 1) (+ c flashed) state)
        ))))

(defn part1 [input]
  (->> input
       parse-input
       (flashes-after-steps 100)))

(part1 test-input);; => 1656
(part1 real-input);; => 1599

(defn simultaneous [inp]
  (loop [n 0 i inp]
    (if (= #{0} (set (apply concat i)))
      n
      (let [[_ state] (flash2 (inc-levels i))]
        (recur (inc n) state)
        ))))

(defn part2 [input]
  (->> input
       parse-input
       simultaneous))

(part2 test-input);; => 195
(part2 real-input);; => 418
