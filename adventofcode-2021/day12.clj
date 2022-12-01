(ns day12
  (:require [clojure.set :as set]))

(def test-input "start-A
start-b
A-c
A-b
b-d
A-end
b-end")

(def test-input2 "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW")

(def real-input (slurp "./inputs/day12.txt"))

(defn parse-input [input]
  (->> input
       clojure.string/split-lines
       (map (fn [line] (clojure.string/split line #"-")))
       (mapcat (fn [[from to]] [[from to] [to from]]))
       ))

(defn get-children [graph node]
  (set (keep #(when (= (first %) node) (second %)) graph)))

(defn small-cave? [c]
  (= c (clojure.string/lower-case c)))

(defn paths [f path graph]
  (if (= (first path) "end")
    (list path)
    (mapcat (fn [next] (paths f (conj path next) graph)) (f graph (first path) path))))

(defn no-revisits [graph node path]
  (set/difference (get-children graph node) (set (filter small-cave? path))))

(defn one-revisit [graph node path]
  (if ((set (vals (frequencies (filter small-cave? path)))) 2)
    (no-revisits graph node path)
    (disj (get-children graph node) "start")))

(defn part1 [input]
  (->> input
       pi
       (paths no-revisits '("start"))
       count
   ))
(defn part2 [input]
  (->> input
       parse-input
       (paths one-revisit '("start"))
       count
   ))

(part1 test-input);; => 10
(part1 test-input2);; => 226
(part1 real-input);; => 5333

(part2 test-input);; => 36
(part2 test-input2);; => 3509
(part2 real-input);; => 146553
