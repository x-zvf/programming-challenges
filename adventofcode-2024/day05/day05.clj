#!/usr/bin/env bb
(require '[clojure.string :as s])

(def input
  (let [[ms,is] (-> *command-line-args*
                    first
                    slurp
                    (s/split #"\n\n")
                    (->> (map #(s/split % #"\n"))))
        p (fn [r f i]
            (-> i
                (->> (map #(s/split % r)))
                (->> (map #(map Integer/parseInt %)))
                f))
        ms (p #"\|" set ms)
        is (p #"," vec is)]
    [ms is]))

(defn solve [[mapping ms]]
  (let [s (map #(sort-by
                  identity
                  (fn [a,b] (cond (= a b) 0
                                  (contains? mapping (list a b)) -1
                                  :else 1)) %)
               ms)
        f (fn [inv] (apply +
                           (map (fn [o,s]
                                  (if (= (= o s) inv) 0
                                    (nth s (/ (- (count s) 1) 2)))) ms s)))]
    [(f false), (f true)]))

(println (solve input))
