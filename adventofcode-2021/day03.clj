(ns day03)

(def test-input "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")

(def real-input (slurp "./inputs/day03.txt"))

(defn parse-input [input]
  (-> input
      clojure.string/split-lines
      (->> (map seq))
      ))

(defn transpose [m]
  (apply mapv vector m))

(parse-input test-input)

(defn calc-rate [f in]
  (-> in
      transpose
      (->> (map frequencies)
           (map (partial apply f second))
           (map key)
           (apply str)
           )
      (Long/parseLong 2)
      ))

(defn part1 [in]
  (let [inp (parse-input in)]
    (* (calc-rate max-key inp) (calc-rate min-key inp))
    ))

(part1 test-input);; => 198
(part1 real-input);; => 3633500

(defn to-keep [in n x] ; most common?
  (let [freq (->> in (map (fn [a] (nth a n)))
                  frequencies
                  )]

    (if x
      (if (>= (get freq \1) (get freq \0)) \1 \0)
      (if (<= (get freq \0) (get freq \1)) \0 \1)
      )
    ))

(defn get-rating [input oxygen]
  (-> (loop [nums input idx 0]
        (if (= 1 (count nums))
          (first nums)
          (let [bit-to-keep (to-keep nums idx oxygen)]
            (recur (filter #(= (nth % idx) bit-to-keep) nums) (inc idx))
            )))
      (->> (apply str))
      (Long/parseLong 2)
      ))

(get-rating (parse-input test-input) true);; => 23
(get-rating (parse-input test-input) false);; => 10

(defn part2 [in]
  (let [input (parse-input in)]
    (* (get-rating input true) (get-rating input false))))

(part2 test-input);; => 230
(part2 real-input);; => 4550283
