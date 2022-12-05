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
  (->> (clojure.string/split input #"\n")
    (map #(str % " ")) ; add another space at the end of the line, so we can partition
    (map #(partition 4 %))
    (map #(map (fn [l] (first (filter (fn [c] (Character/isLetter c)) l))) %))
    (drop-last) ; remove the line that contains the indices
    (apply transpose)
    (map reverse)
    (map #(filter (fn [c] (not (nil? c))) %))
    (map vec)
    vec))

(defn parse-moves [input]
  (->> (clojure.string/split input #"\n")
       (map #(re-matches #"move (\d+) from (\d+) to (\d+)" %))
       (map (fn [[_ n f t]] [(read-string n)
                             (- (read-string f) 1) ; Use zero-based indexing
                             (- (read-string t) 1)]))))

(defn parse-input [input]
  (-> input
      (clojure.string/split #"\n\n")
      ((fn [[stack, moves]] (list (parse-stack stack) (parse-moves moves))))))

(defn step [at-a-time-fn [stack moves]]
  (loop [stack stack moves moves]
    (if (empty? moves) [stack, nil]
        (let [[n from to] (first moves)
              m (at-a-time-fn n)
              othermoves (drop 1 moves)
              val-to-move (take-last m (get stack from))]
          (if (= n 0) (recur stack othermoves)
              (recur
               (-> stack
                   (assoc from (vec (drop-last m (get stack from))))
                   (assoc to (apply conj (get stack to) val-to-move)))
               (conj othermoves (list (- n m) from to))))))))

(defn solve [at-a-time-fn input]
  (->> input
       parse-input
       (step at-a-time-fn)
       first
       (map last)
       (apply str)))

(defn part1 [input]
  (solve (fn [_] 1) input))

(defn part2 [input]
  (solve identity input))

(part1 test-input);; => "CMZ"
(part1 real-input);; => "VCTFTJQCG"
(part2 test-input);; => "MCD"
(part2 real-input);; => "GCFGLDNJZ"

