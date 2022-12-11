(ns day11)
(require 'clojure.string)

(def test-input "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1")

(def real-input (slurp "./inputs/day11.txt"))

(defn parse-monkey [lines]
  (let [[id-line items-line op-line test-line true-line false-line]
        (clojure.string/split lines #"\n")
        divisor (parse-long (first (re-seq #"\d+" test-line)))
        true-id (parse-long (first (re-seq #"\d+" true-line)))
        false-id (parse-long (first (re-seq #"\d+" false-line)))]
    {:id (parse-long (first (re-seq #"\d+" id-line)))
     :items (mapv parse-long (re-seq #"\d+" items-line))
     :op (let [[_ calc] (clojure.string/split op-line #"=")
               [_ a1 operator a2] (clojure.string/split calc #" ")]
           (eval (read-string (str "(fn [old] " "(" operator " " a1 " " a2"))"))))
     :next-monkey (fn [item]
                    (if (= 0 (mod item divisor)) true-id false-id))
     :divisor divisor
     :inspected 0
     }))

(defn reduce-worry [items]
  (quot items 3))

(defn run-monkey [reduce-worry-fn monkeys id]
  (let [{:keys [items op next-monkey]} (monkeys id)]
    (reduce (fn [res item]
              (let [new-item (reduce-worry-fn (op item))
                    to-monkey (next-monkey new-item)]
              (update-in res [to-monkey :items] conj new-item)))
            (-> monkeys
                (update-in [id :inspected] + (count items))
                (assoc-in [id :items] [])
                )
            items)
    ))

(defn run-monkeys [reduce-worry-fn monkeys]
  (reduce (partial run-monkey reduce-worry-fn) monkeys (range (count monkeys))))

(defn parse-input [input]
  (-> input
      (clojure.string/split #"\n\n")
      (->> (mapv parse-monkey))))

(defn get-reduce-worry-fn [default? monkeys]
  (if default?
    #(quot % 3)
    (let [common-multiple (reduce * (map :divisor monkeys))]
      #(mod % common-multiple)
      )))


(defn solve [default-reducer? iter input]
  (let [monkeys (parse-input input)
        reduce-worry-fn (get-reduce-worry-fn default-reducer? monkeys)]
    (->> monkeys
         (iterate (partial run-monkeys reduce-worry-fn))
         (#(nth % iter))
         (map :inspected)
         (sort >)
         (take 2)
         (apply *))))

(defn part1 [input]
  (solve true 20 input))

(defn part2 [input]
  (solve false 10000 input))

(part1 test-input);; => 10605
(part1 real-input);; => 54036
(part2 test-input);; => 2713310158
(part2 real-input);; => 13237873355


