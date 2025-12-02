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
  (cond (= p 0)
        1

        (= p 1)
        b

        (even? p)
        (recur (* b b) (quot p 2))

        :else
        (* b (power b (dec p)))))

(defn palindrome-of [n]
  (let [d (count (str n))
        m (power 10 d)]
    (+ n (* m n))))

(defn minmaxp [x]
  (let [s (str x)
        sl (count s)
        hl (quot sl 2)
        [top bottom] (split-at hl s)
        tn (if (= 0 hl) 0 (Long/parseLong (apply str top)))
        bn (Long/parseLong (apply str bottom))]
    (cond
      (= 1 (mod sl 2)) [(power 10 hl)
                        (Long/parseLong (apply str \0 (repeat hl 9)))]
      (< tn bn) [(+ 1 tn) tn]
      (= tn bn) [tn tn]
      (> tn bn) [tn (- tn 1)])))

(defn get-pals [[a b]]
  (let [[smol _] (minmaxp a)
        [_ large] (minmaxp b)]
    (map palindrome-of (range smol (+ 1 large)))))

(defn solve1 [prob]
  (->> prob
       (mapcat get-pals)
       (apply +)))

(solve1 sample) ; 1227775554
(solve1 input) ; 43952536386

(defn pals [digit]
  (let [fac (power 10 (+ 1 (int (clojure.math/log10 digit))))]
    (drop 1 (reductions (fn [n d] (+ d (* n fac))) (repeat digit)))))

(defn palindromes-range [[minv maxv]]
  (let [[_ maxnum] (minmaxp maxv)]
    (set (mapcat (fn [x]
                   (->> x
                        pals
                        (drop-while #(< % minv))
                        (take-while #(<= % maxv))))
                 (range 1 (+ 1 maxnum))))))

(defn solve2 [prob]
  (->> prob
       (mapcat palindromes-range)
       (apply +)))

(solve2 sample) ; 4174379265
(solve2 input) ; 54486209192
