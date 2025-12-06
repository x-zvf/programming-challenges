(ns day06
  (:require [clojure.string :as string]
            [clojure.edn :as edn]))

(def sample (slurp "./sample.txt"))
(def input (slurp "./input.txt"))

(defn transpose [m]
  (apply mapv vector m))

(defn part1 [file]
  (->> file
       (string/split-lines)
       (mapv (fn [line]
               (->> line
                    (#(string/split % #" +"))
                    (filter #(not (empty? %)))
                    (mapv edn/read-string))))
       transpose
       (map reverse)
       (cons +)
       eval))

(part1 sample) ; 4277556
(part1 input) ; 4405895212738

(defn part2 [file]
  (let [lines (->> file string/split-lines (mapv vec))
        number-lines (drop-last lines)
        ops-line (last lines)

        [split-idxs ops]
        (->> ops-line
             (keep-indexed #(if (not= \space %2) [%1 %2] nil))
             transpose)
        ops (map #(edn/read-string (str %)) ops)
        split-idxs (conj split-idxs (count (first number-lines)))
        blocks (mapv (fn [line]
                       (mapv (fn [[f t]] (subvec line f t))
                             (partition 2 1 split-idxs)))
                     number-lines)]
    (->> ops
         (map-indexed
          (fn [i op]
            (->> blocks
                 (mapv #(nth % i))
                 transpose
                 (map #(edn/read-string (apply str %)))
                 (filter #(not (nil? %)))
                 (cons op)
                 eval)))
         (apply +))))

(part2 sample) ; 3263827
(part2 input)  ; 7450962489289
