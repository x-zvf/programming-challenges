(ns day08)

(def test-input1 "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
(def test-input2 "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")

(def real-input (slurp "./inputs/day08.txt"))

;; this one requires some logic
;; if count = 2 ==> 1
;; if count = 4 ==> 4
;; if count = 3 ==> 7
;; if count = 6 ==> 0 || 6 || 9
;;   6  has 1 segment that matches 1
;;   0  has 3 segments that match with 4
;;   otherwise 9
;; if count = 5 ==> 2 || 3 || 5
;;   5  has 5 segments that match 9 ;; dependency!
;;   3  has 2 segments that 1
;;   otherwise 2

(defn parse-input [in]
  (->> in
       clojure.string/split-lines
       (map (fn [line]
              (map (fn [x]
                     (map set
                          (clojure.string/split x #" ")))
                   (clojure.string/split line #" \| ")))
            )))

(defn find-wires-with-length [n input-line]
  (filter #(= n (count %)) (first input-line)))

(defn filter-intersection-len [n s segments]
  (filter #(= n (count (clojure.set/intersection s %))) segments))

(defn determine-trivial [in-line]
  {
   1 (first (find-wires-with-length 2 in-line))
   4 (first (find-wires-with-length 4 in-line))
   7 (first (find-wires-with-length 3 in-line))
   8 (first (find-wires-with-length 7 in-line))
   :len5 (find-wires-with-length 5 in-line)
   :len6 (find-wires-with-length 6 in-line)
   })

(defn decuce-rest [m]
  (let [n6 (first (filter-intersection-len 1 (get m 1) (:len6 m)))
        n3 (first (filter-intersection-len 2 (get m 1) (:len5 m)))]
    (let [n0 (first (filter (partial not= n6) (filter-intersection-len 3 (get m 4) (:len6 m))))]
      (let [n9 (first (filter (partial not= n6) (filter (partial not= n0) (:len6 m))))]
        (let [n5 (first (filter (partial not= n3) (filter-intersection-len 5 n9 (:len5 m))))]
          (let [n2 (first (filter (partial not= n3) (filter (partial not= n5) (:len5 m))))]
            {(sort n0) 0
             (sort (get m 1)) 1
             (sort n2) 2
             (sort n3) 3
             (sort (get m 4)) 4
             (sort n5) 5
             (sort n6) 6
             (sort (get m 7)) 7
             (sort (get m 8)) 8
             (sort n9) 9}))))))

(defn get-mappings-for-line [in]
  (decuce-rest (determine-trivial in)))

(defn part1 [input]
  (apply + (map (fn [line]
                  (let [mappings (get-mappings-for-line line)]
                    (count
                     (filter #(or (= 1 %) (= 4 %) (= 7 %) (= 8 %))
                             (map #(get mappings (sort %))
                                  (second line))))))
                (parse-input input))))

(part1 test-input2);; => 26
(part1 real-input);; => 392

(defn line-to-number [in-line]
  (Integer/parseInt (apply str
                           (map #(get (get-mappings-for-line in-line) (sort %))
                                (second in-line)))))

(defn part2 [input]
  (apply + (map line-to-number (parse-input input))))

(part2 test-input2);; => 61229
(part2 real-input);; => 1004688
