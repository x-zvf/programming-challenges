(ns day08)
(require 'clojure.string)

(def test-input "30373
25512
65332
33549
35390")

(def real-input (slurp "./inputs/day08.txt"))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map seq x)
    (map #(map (fn [x] (read-string (str x))) %) x)
    ))
(defn vectorify [matrix]
      (vec (map vec matrix)))

(defn transpose [xs]
  (apply map list xs))

(defn rotate [matrix]
  (map reverse (transpose matrix)))

(defn rotate-n [n matrix]
  (nth (iterate rotate matrix) n))

(defn rotations [matrix]
  (take 4 (iterate rotate matrix)))

(defn visible-in-row? [row]
  (first (reduce (fn [[res vheight] tree]
            (list
             (conj res (< vheight tree))
             (max vheight tree))) '([] -1) row)))

(defn visible-from-left? [matrix]
  (map visible-in-row? matrix))



(defn visible? [matrix]
  (->> matrix
       rotations
       (map visible-from-left?)
       (map-indexed #(rotate-n (- 4 %1) %2))
       (map flatten)
       (apply map (fn [& xs] (if (some identity xs) 1 0)))
       (apply +)))

(map (fn [& xs] (if (some identity xs) 1 0)) [true false true] [false false false] [false false false])

(defn part1 [input]
  (->> input
       parse-input
       visible?
       ))


(defn get-viewing-seqs [matrix tmatrix x y]
    (list (nth (nth matrix y) x)
          [(reverse (subvec (nth matrix y) 0 x))
           (subvec (nth matrix y) (+ 1 x))
           (reverse (subvec (nth tmatrix x) 0 y))
           (subvec (nth tmatrix x) (+ 1 y))]))

(get-viewing-seqs
 (vectorify (parse-input test-input))
 (vectorify (transpose (parse-input test-input)))
 3 2)


(defn get-views [in]
  (let [_lm in
        matrix (vectorify _lm)
        tmatrix (vectorify (transpose _lm))
        height (count matrix)
        width (count (first matrix))
        ]
    (for [x (range width) y (range height)]
      (get-viewing-seqs matrix tmatrix x y))))

(defn part2 [input]
  (->> input
       parse-input
       get-views
       (map (fn [[tree views]]
              (->> views
                   (map (fn [view]
                          (let [[vis hidden] (split-with #(< % tree) view)]
                            (conj vis (first hidden))
                            )))
                   (map (fn [s] (filter #(not (nil? %)) s)))
                   (map count)
                   (apply *)
                   )))
       (apply max)))

(part1 test-input);; => 21
(part1 real-input);; => 1776
(part2 test-input);; => 8
(part2 real-input);; => 234416
