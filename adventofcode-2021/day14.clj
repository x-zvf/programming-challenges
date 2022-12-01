(ns day14)

(def test-input "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C")

(def real-input (slurp "./inputs/day14.txt"))

(defn parse-input [in] (let [[template insertions] (clojure.string/split in #"\n\n")]
    (list template
          (->> insertions
               clojure.string/split-lines
               (mapcat #(let [[p n] (clojure.string/split % #" -> ")]
                           (list (vec p) (first n))))
               (apply assoc {})
               ))))


(parse-input test-input)

(defn pairwise [template]
  (frequencies (map vec (partition 2 1 template))))

(defn step [letters pairs insertions]
  (reduce
   (fn [[ls ps :as state] [pair n]]
     (if (< 0 n)
       (if-let [new (get insertions pair)]
         [(update ls new (fnil (partial + n) 0))
          (let [[a b] pair]
            (-> ps
                (update pair (fnil - 0) n)
                (update [a new] (fnil + 0) n)
                (update [new b] (fnil + 0) n)))]
         state)
       state))
   [letters pairs] pairs))

(defn solve [n input]
  (let [[template insertions] (parse-input input)
        letters (frequencies template)
        pairs (pairwise template)]
    (loop [i n l letters p pairs]
      (if (= 0 i)
        (- (apply max (vals l)) (apply min (vals l)))
        (let [[nl np] (step l p insertions)]
          (recur (dec i) nl np))))))

(solve 10 test-input);; => 1588
(solve 10 real-input);; => 2068

(solve 40 test-input);; => 2188189693529
(solve 40 real-input);; => 2158894777814

;; naive iteration
(comment
  (defn matching-points [in a b]
    (map (comp inc first)
         (filter (fn [[idx _]] (= b (nth in (inc idx) nil)))
                 (filter #(= a (second %)) (map-indexed vector in)))))

  (defn get-insertion-points [template pairmap]
    (map-indexed
     (fn [idx [i c]] [(+ i idx) c])
     (sort-by first
              (mapcat
               (fn [[[a b] to-insert]]
                 (map (fn [i] [i to-insert])
                      (matching-points template a b))
                 ) pairmap))))

  (defn insert-chars [template chars]
    (reduce (fn [s [i c]] (str (subs s 0 i) c (subs s i))) template chars))

  (defn step [template pairmap]
    (insert-chars
     template
     (get-insertion-points template pairmap)))

  (defn steps [n template pairmap]
    (loop [t template i n]
      (if (= 0 i)
        t
        (recur (step t pairmap) (dec i)))))

  (defn slow-solve [n input]
    (->> input
         parse-input
         (apply steps n)
         frequencies
         vals
         (#(-
            (apply max %)
            (apply min %)))
         ))

  (slow-solve 10 test-input);; => 1588
  (slow-solve 10 real-input);; => 2068
  )
