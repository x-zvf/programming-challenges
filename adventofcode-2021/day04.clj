(ns day04)

(def test-input "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
")

(def real-input (slurp "./inputs/day04.txt"))

(defn parse-input
  "Parse Input into a list with (numbers boards) where numbers is a list of integers and boards is a list of lists of lists"
  [in]
  (let [[numbers & boards] (clojure.string/split in #"\n\n")]
    (list (map #(Long/parseLong % 10) (clojure.string/split numbers #","))
          (->> boards
               (map clojure.string/split-lines)
               (map (fn [b]        ; each board
                      (map (fn [r] ; each row of the board
                             (-> r
                                 clojure.string/trim
                                 (clojure.string/split #"\s+")
                                 (->> (map (fn [c] (Integer/parseInt c 10))))
                                 )
                             )
                           b)
                      ))
               )
          )
    )
  )

(defn transpose [m]
  (apply mapv vector m))

(defn filter-winners [boards]
  "return only winning boards, winning being where at least one row or column consists of only negative numbers"
  (filter (fn [b]
            (let [to-check (concat b (transpose b))]  ; we need to check both all rows and collumns
              (< 0 (count (filter
                           (fn [r]
                             (every? (fn [n]
                                       (< n 0))
                                     r))
                           to-check)))
              )
            )
          boards))

(defn bingo
  "Play until a winner is found and if `only-last-board` until only one board is left.
  Each iteration, the boards are filtered on `filter-on`."
  [filter-on only-last-board numbers boards]
  (loop [bs boards [current-number & ns] numbers last-number -1]
    (let [winners (filter-winners bs)] ; To optimize, this check should only be done after (count (first bs))
      (if (and (< 0 (count winners)) (if only-last-board (= 1 (count bs)) true))
        (list (first winners) last-number)
        (recur (map (fn [b] ; each board
                      (map (fn [r] ; each row
                             (map (fn [n] ; each number
                                    (if (= n current-number) ; If the number matches the number just called
                                      -1                     ; eliminate the number
                                      n)
                                    )
                                  r)
                             )
                           b)
                      )
                    (filter (partial filter-on winners) bs))
               ns
               current-number)
        )
      )
    ))

(defn score
  "Calculate the score of the winning board."
  [winner last-number]
  (->> winner
       (apply concat)
       (filter (fn [n] (< 0 n))) ; filter out all the marked numbers
       (apply +)
       (* last-number)
       ))

(defn part1 [in]
  (apply score
         (apply bingo
                (fn [_ _] true)
                false
                (parse-input in))))


(part1 test-input);; => 4512
(part1 real-input);; => 65325

(defn collection-contains?
  "returns true if item is in collection, false otherwise. This really ought to be in the standard library."
  [coll item]
  (some (fn [x] (= item x)) coll))

(defn part2 [in]
  (apply score
         (apply bingo
                (fn [winners b]
                  (not (collection-contains? winners b))) ;; On each iteration, filter out all the winners
                true
                (parse-input in))))

(part2 test-input);; => 1924
(part2 real-input);; => 4624
