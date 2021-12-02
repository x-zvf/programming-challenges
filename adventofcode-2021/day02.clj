(ns day02)

(def test-input "forward 5
down 5
forward 8
up 3
down 8
forward 2")

(def real-input (slurp "./inputs/day02.txt"))

(defn parse-input [istr]
  (map #(let [[direction value] (clojure.string/split % #" ")]
          (list (keyword direction) (Integer/parseInt value)))
       (clojure.string/split istr #"\n")))

(parse-input test-input)

(defn delta [[direction amount]]
  (cond
    (= direction :forward) (list amount 0)
    (= direction :up) (list  0 (* -1 amount))
    (= direction :down) (list 0 amount)
    :else (println "unknown direction")
    ))

(delta '(:forward 100));; => (100 0)

(defn part1 [input]
  (apply * (reduce #(list (+ (nth %1 0) (nth %2 0)) (+ (nth %1 1) (nth %2 1)))
                   '(0 0)
                    (map delta (parse-input input)))))

(part1 test-input);; => 150
(part1 real-input);; => 1989265


(defn step [[x y aim][direction amount]]
  (cond
    (= direction :forward) (list (+ x amount) (+ y (* aim amount)) aim)
    (= direction :up) (list  x y (- aim amount))
    (= direction :down) (list x y (+ aim amount))
    :else (println "unknown direction")
    ))

(defn steps [ops]
  (loop [state '(0 0 0) i 0]
    (if (< i (count ops))
      (recur (step state (nth ops i)) (inc i))
      (* (nth state 0) (nth state 1))
      )))

(defn part2 [input]
  (steps (parse-input input)))

(part2 test-input);; => 900
(part2 real-input);; => 2089174012
