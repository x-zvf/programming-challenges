(ns day06
  (:require [clojure.string :as string]
            [clojure.edn :as edn]))

(defn transpose [m]
  (apply mapv vector m))

(defn solve [file part2]
  (let [lines (->> file string/split-lines (mapv vec))
        number-lines (drop-last lines)
        ops-line (last lines)

        [split-idxs ops]
        (->> ops-line
             (keep-indexed #(if (not= \space %2) [%1 %2] nil))
             transpose)
        split-idxs (conj split-idxs (count (first number-lines)))
        ops (map #(edn/read-string (str %)) ops)

        blocks (mapv (fn [line]
                       (mapv (fn [[f t]] (subvec line f t))
                             (partition 2 1 split-idxs)))
                     number-lines)]
    (->> ops
         (map-indexed
          (fn [i op]
            (->> blocks
                 (mapv #(nth % i))
                 ((if part2 transpose identity))
                 (map #(edn/read-string (apply str %)))
                 (filter #(not (nil? %)))
                 (cons op)
                 eval)))
         (apply +))))

(def sample (slurp "./sample.txt"))
(def input (slurp "./input.txt"))
(solve sample false) ; 4277556
(solve input false) ; 4405895212738
(solve sample true) ; 3263827
(solve input true)  ; 7450962489289
