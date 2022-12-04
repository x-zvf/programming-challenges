(ns day15
  (:require
            [clojure.data.priority-map :as data]
            [clojure.string]))

(def test-input "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581")


(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map seq x)
    (map (fn [line] (map #(read-string (str %)) line)) x)))

(parse-input test-input)

