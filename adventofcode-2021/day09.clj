(ns day09)

(def test-input "2199943210
3987894921
9856789892
8767896789
9899965678")

(def real-input (slurp "./inputs/day09.txt"))

(defn parse-input [in]
  (map #(map (fn [c]
               (Integer/parseInt (str c) 10))
             (seq %))
       (clojure.string/split-lines in)))

(defn get-pos [l x y]
  (if (or (> 0 x) (> 0 y) (<= (count l) y) (<= (count (first l)) x))
    (Integer/MAX_VALUE)
    (nth (nth l y) x)))

(defn get-adj [l x y]
  (list (get-pos l (- x 1) y)
        (get-pos l (+ x 1) y)
        (get-pos l x (- y 1))
        (get-pos l x (+ y 1))))

(defn low? [l x y]
  (< (get-pos l x y) (apply min (get-adj l x y))))

(defn get-low-points [l]
  (filter
   (fn [[x y]] (low? l x y))
   (for [x (range (count (first l)))
         y (range (count l))]
     [x y])))

(defn part1 [input]
  (let [l (parse-input input)]
    (->> l
         get-low-points
         (map (fn [[x y]] (get-pos l x y)))
         (map (partial + 1))
         (apply +))))

(part1 test-input);; => 15
(part1 real-input);; => 594

(defn get-basin [l low-point]
  (loop [solution #{} points [low-point] visited #{}]
    (if (= 0 (count points))
      solution
      (let [[x y] (first points) r (rest points)]
        (if (or
             (>= (get-pos l x y) 9)
             (contains? visited (list x y)))
          (recur solution r visited)
          (recur (conj solution (list x y))
                 (conj r
                       (list x (- y 1))
                       (list x (+ y 1))
                       (list (- x 1) y)
                       (list (+ x 1) y))
                 (conj visited (list x y)))))
      )))

(defn part2 [input]
  (let [l (parse-input input)]
    (->> l
         get-low-points
         (map (partial get-basin l))
         (map count)
         sort
         (take-last 3)
         (apply *)
         )))

(part2 test-input);; => 1134
(part2 real-input);; => 858494
