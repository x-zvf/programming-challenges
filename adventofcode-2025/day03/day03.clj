(ns day03
  (:require [clojure.string :as string]))

(defn parse [f]
  (->> f
       slurp
       string/split-lines
       (mapv (fn [line] (mapv #(Integer/parseInt (str %)) line)))))

(defn to-jolt [xs]
  (reduce #(+ %2 (* 10 %1)) 0 xs))

(defn with-new-bat [xs b]
  (map #(vec (concat [b] (subvec xs 0 %) (subvec xs (inc %))))
       (range 0 (count xs))))

(def max-jolt
  (memoize
   (fn [c bs]
     (if (= c (count bs)) bs
         (let [[x & ts] bs
               mj (vec (max-jolt c ts))
               opt (cons mj (with-new-bat mj x))]
           (apply max-key to-jolt opt))))))

(defn solve [n inp]
  (->> inp
       (pmap (partial max-jolt n))
       (map to-jolt)
       (apply +)))

(def part1 (partial solve 2))
(def part2 (partial solve 12))

(def sample (parse "./sample.txt"))
(def input (parse "./input.txt"))

(println
 "part1 sample: "
 (part1 sample) ; 357
 "part1 input: "
 (part1 input) ; 17207
 "part2 sample: "
 (part2 sample) ; 3121910778619
 "part2 input: "
 (part2 input); 170997883706617
 )
