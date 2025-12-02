(ns day02
  (:require [clojure.string]
            [clojure.math]))

(defn parse [f]
  (-> f
      slurp
      (clojure.string/split #",")
      (->> (map
            (fn [pair]
              (mapv
               Long/parseLong
               (clojure.string/split (clojure.string/trim pair) #"-")))))))

(def sample (parse "./sample.txt"))
(def input (parse "./input.txt"))

(defn power [b p]
  (cond (= p 0) 1
        (= p 1) b
        (even? p) (recur (* b b) (quot p 2))
        :else (* b (power b (dec p)))))

(defn ndigits [x]
  (+ 1 (int (clojure.math/log10 x))))

(defn palindrome-of [n]
  (let [d (ndigits n)
        m (power 10 d)]
    (+ n (* m n))))

(defn nines [n]
  (reduce #(+ %2 (* 10 %1)) 0 (repeat n 9)))

(defn minmax-palindrome [x]
  (let [sl (ndigits x)
        hl (quot sl 2)
        dm (power 10 hl)
        tn (quot x dm)
        bn (mod x dm)]
    (cond
      (odd? sl) [dm (nines hl)]
      (< tn bn) [(+ 1 tn) tn]
      (= tn bn) [tn tn]
      (> tn bn) [tn (- tn 1)])))

(defn get-pals [[a b]]
  (let [[smol _] (minmax-palindrome a)
        [_ large] (minmax-palindrome b)]
    (map palindrome-of (range smol (+ 1 large)))))

(defn solve1 [prob]
  (->> prob
       (mapcat get-pals)
       (apply +)))

(solve1 sample) ; 1227775554
(solve1 input) ; 43952536386

(defn pals [x]
  (let [fac (power 10 (ndigits x))]
    (->> x
         repeat
         (reductions (fn [n d] (+ d (* n fac))))
         (drop 1))))

(defn palindromes-range [[minv maxv]]
  (let [[_ maxnum] (minmax-palindrome maxv)]
    (->> (range 1 (+ 1 maxnum))
         (mapcat
          (fn [x]
            (->> x
                 pals
                 (drop-while #(< % minv))
                 (take-while #(<= % maxv)))))
         set)))

(defn solve2 [prob]
  (->> prob
       (mapcat palindromes-range)
       (apply +)))

(solve2 sample) ; 4174379265
(solve2 input) ; 54486209192
