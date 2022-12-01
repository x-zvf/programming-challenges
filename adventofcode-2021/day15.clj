(ns day15
  (:require [com.rpl.specter :as S]
            [clojure.data.priority-map :as data]))

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

(defn find-path [start-coord neighbours]
  (loop [m {} Q (data/priority-map start-coord 0)]
    (if-let [[coord dist] (peek Q)]
      (let [dists (->> (neighbours coord)
                       )])
      m)
    ))
