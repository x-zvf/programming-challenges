(ns day01)

(def test-input "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")

(def real-input (slurp "./inputs/day01.txt"))

(defn part1 [input]
  (->> (clojure.string/split input #"\n\n")
       (parse-elve)
       (apply max)))

(defn parse-elve [elve]
  (map (fn [x]
         (->> (clojure.string/split x #"\n")
              (map #(Integer/parseInt %))
              (reduce +))) elve))

(part1 test-input)
(part1 real-input); => 69289

(defn part2 [input]
  (->> (clojure.string/split input #"\n\n")
       (parse-elve)
       (sort)
       (take-last 3)
       (reduce +)))

(part2 test-input); => 45000
(part2 real-input); => 205615