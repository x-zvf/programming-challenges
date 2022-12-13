(ns day13)
(require 'clojure.string)

(def test-input "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]")

(def real-input (slurp "./inputs/day13.txt"))

(defn force-vec [x]
  (if (coll? x) x [x]))

(defn compare-lr [left right]
  (cond
    (and (number? left) (number? right))
    (cond (= left right) :next
          (< left right) :correct
          :else :wrong)
    (and (coll? left) (coll? right))
    (loop [[l & lt] left [r & rt] right]
      (cond (and (nil? l) (nil? r)) :next
            (nil? l) :correct
            (nil? r) :wrong
            :else (let [result (compare l r)]
                    (if (= result :next)
                      (recur lt rt)
                      result))))
    :else (compare (force-vec left) (force-vec right))))

(defn part1 [input]
  (->> input
       ((fn [x] (clojure.string/split x #"\n\n")))
       (map #(map read-string (clojure.string/split % #"\n")))
       (map (fn [[l r]] (compare-lr l r)))
       (map-indexed (fn [idx val] (if (= val :correct) (inc idx) 0)))
       (apply +)))

(def divider-packets #{[[2]] [[6]]})

(defn part2 [input]
  (->> input
       ((fn [x] (clojure.string/split x #"\n")))
       (filter seq) ; not empty?
       (map read-string)
       (concat divider-packets)
       (sort #(= (compare %1 %2) :correct))
       (map-indexed (fn [idx, packet]
                      (if (contains? divider-packets packet)
                        (inc idx)
                        1)))
       (apply *)))


(part1 test-input);; => 13
(part1 real-input);; => 6046
(part2 test-input);; => 140
(part2 real-input);; => 21423

