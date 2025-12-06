(ns day06
  (:require [clojure.string :as string]))

(defn transpose [m]
  (apply mapv vector m))

(defn solve [file part2]
  (let [lines (->> file string/split-lines (mapv vec))
        number-lines (drop-last lines)
        ops-line (last lines)

        [split-idxs ops]
        (->> ops-line
             (keep-indexed #(when (not= \space %2) [%1 %2]))
             transpose)
        line-length (count (first number-lines))
        split-idxs (conj split-idxs line-length)
        ops (map {\+ + \* *} ops)

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
                 (map #(string/trim (apply str %)))
                 (filter #(not (empty? %)))
                 (map Integer/parseInt)
                 (cons op)
                 eval)))
         (apply +))))

(def sample (slurp "./sample.txt"))
(def input (slurp "./input.txt"))
(solve sample false) ; 4277556
(solve input false) ; 4405895212738
(solve sample true) ; 3263827
(solve input true)  ; 7450962489289
