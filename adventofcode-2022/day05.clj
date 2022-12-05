(ns day05)
(require 'clojure.string)

(def test-input "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2")

(def real-input (slurp "./inputs/day05.txt"))

(defn transpose [& xs]
  (apply map list xs))

(defn parse-stack [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map #(str % " ") x)
    (map #(partition 4 %) x)
    (map #(map (fn [l] (first (filter (fn [c] (Character/isLetter c)) l))) %) x)
    (drop-last x)
    (apply transpose x)
    (map reverse x)
    (map #(filter (fn [c](not (nil? c))) %) x)
    (map vec x)
    (vec x)
  ))

(defn parse-moves [input]
  (as-> input x
    (clojure.string/split x #"\n")
    (map #(re-matches #"move (\d+) from (\d+) to (\d+)" %) x)
    (map (fn [[_ n f t]] [(read-string n)
                               (- (read-string f) 1)
                               (- (read-string t) 1)]) x)))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\n\n")
    ((fn [[stack, moves]] (list (parse-stack stack) (parse-moves moves))) x)
    ))

(defn step [at-a-time-fn [istack imoves]]
  (loop [stack istack moves imoves]
    (if (empty? moves) [stack, nil]
        (let [[n f t] (first moves)
              m (at-a-time-fn n)
              othermoves (drop 1 moves)
              val-to-move (take-last m (get stack f))]
          (if (= n 0) (recur stack othermoves)
              (recur
               (-> stack
                   (assoc f (vec (drop-last m (get stack f))))
                   (assoc t (apply conj (get stack t) val-to-move)))
               (conj othermoves (list (- n m) f t))))))))

(defn solve [stepfn input]
  (->> input
       parse-input
       (step stepfn)
       first
       (map last)
       (apply str)))

(defn part1 [input]
  (solve (fn [_] 1) input))

(defn part2 [input]
  (solve identity input))


(part1 test-input)
(part1 real-input)
(part2 test-input)
(part2 real-input)
