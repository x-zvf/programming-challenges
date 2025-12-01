(ns day01
  (:require [clojure.string]))

(defn get-rotations [file]
  (->> file
    slurp
    clojure.string/split-lines
    (mapv (fn [line]
        (let [sign (if (= \L(first line)) -1 1)
              value (Integer/parseInt (apply str (rest line)))]
          (* sign value)
          )))))

(def rotations (get-rotations (first *command-line-args*)))

(defn rotate [sv r]
  (mod (+ sv r) 100))

(defn part1 [rs]
  (->> rs
       (reductions rotate 50)
       (filter #(= % 0))
       count))


(defn rotate2 [[sv zc] r]
  (let [nv (mod (+ sv r) 100)
        full-rotations (quot (abs r) 100)
        signdelta (cond (and (< r 0) (> nv sv) (not= sv 0)) 1
                        (and (> r 0) (< nv sv)) 1
                        (and (not= sv 0) (= nv 0)) 1
                        :else 0)
        zerocrossings (+ full-rotations signdelta)]
    [nv (+ zc zerocrossings)]
  ))



(defn part2 [rs]
  (->> rs
       (reduce rotate2 [50 0])
       second))

(println "part1: " (part1 rotations))
(println "part2: " (part2 rotations))


