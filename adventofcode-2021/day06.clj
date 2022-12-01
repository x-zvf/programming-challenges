(ns day06)

(def test-input [3 4 3 1 2])

(def real-input [1 3 4 1 5 2 1 1 1 1 5 1 5 1 1 1 1 3 1 1 1 1 1 1 1 2 1 5 1 1 1 1 1 4 4 1 1 4 1 1 2 3 1 5 1 4 1 2 4 1 1 1 1 1 1 1 1 2 5 3 3 5 1 1 1 1 4 1 1 3 1 1 1 2 3 4 1 1 5 1 1 1 1 1 2 1 3 1 3 1 2 5 1 1 1 1 5 1 5 5 1 1 1 1 3 4 4 4 1 5 1 1 4 4 1 1 1 1 3 1 1 1 1 1 1 3 2 1 4 1 1 4 1 5 5 1 2 2 1 5 4 2 1 1 5 1 5 1 3 1 1 1 1 1 4 1 2 1 1 5 1 1 4 1 4 5 3 5 5 1 2 1 1 1 1 1 3 5 1 2 1 2 1 3 1 1 1 1 1 4 5 4 1 3 3 1 1 1 1 1 1 1 1 1 5 1 1 1 5 1 1 4 1 5 2 4 1 1 1 2 1 1 4 4 1 2 1 1 1 1 5 3 1 1 1 1 4 1 4 1 1 1 1 1 1 3 1 1 2 1 1 1 1 1 2 1 1 1 1 1 1 1 2 1 1 1 1 1 1 4 1 1 1 1 1 1 1 1 1 1 1 1 1 1 2 1 1 2 5 1 2 1 1 1 1 1 1 1 1 1])

(def count-fish
  (memoize
   (fn [days timer]
     (if (<= days timer)
       1
       (+ (count-fish (- days timer) 9)
          (count-fish (- days timer) 7))))))

(defn fish-count-after-days [days in]
  (reduce + (map (partial count-fish days) in)))

(fish-count-after-days 80 test-input);; => 5934
(fish-count-after-days 80 real-input);; => 386755
(fish-count-after-days 256 test-input);; => 26984457539
(fish-count-after-days 256 real-input);; => 1732731810807
