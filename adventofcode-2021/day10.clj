(ns day10)

(def test-input "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]")

(def real-input (slurp "./inputs/day10.txt"))

(def open-p
  {\( \)
   \[ \]
   \{ \}
   \< \>})

(def points
  {
   \) 3
   \] 57
   \} 1197
   \> 25137
   })

(defn solve [line]
  (list line
        (reduce (fn [[p s] c]
                  (let [cls (get open-p c) top (first s)]
                    (cond
                      (not= p 0) (list p s)
                      (not= cls nil) (list 0 (conj s cls))
                      (= c top) (list 0 (rest s))
                      :else (list (get points c) s)
                      )
                    ))
                '(0 ()) (seq line))))


(defn part1 [input]
  (->> input
       clojure.string/split-lines
       (map solve)
       (map second)
       (map first)
       (apply +)
       ))

(part1 test-input);; => 26397
(part1 real-input);; => 339477

(def completion-points
  {
   \) 1
   \] 2
   \} 3
   \> 4
   })

(defn completion-score [s]
  (reduce (fn [p c]
            (+ (* 5 p) (get completion-points c)))
          0 s))

(defn get-middle [x]
  (nth x (/ (count x) 2)))

(defn part2 [input]
  (->> input
       clojure.string/split-lines
       (map solve)
       (filter (fn [[_ [p _]]] (= 0 p)))
       (map (fn [[_ [_ s]]] (completion-score s)))
       sort
       get-middle
       ))

(part2 test-input);; => 288957
(part2 real-input);; => 3049320156
