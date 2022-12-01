# Clojure

[full solution](https://github.com/x-zvf/programming-challenges/blob/master/adventofcode-2021/day09.clj)

As always feedback is appreciated, I am just learning clojure.

Parsing input

    (defn parse-input [in]
      (map #(map (fn [c]
                   (Integer/parseInt (str c) 10))
                 (seq %))
           (clojure.string/split-lines in)))

Helpers

Get the digit at the x y position (with invalid positions being a large number)

    (defn get-pos [l x y]
      (if (or (> 0 x) (> 0 y) (<= (count l) y) (<= (count (first l)) x))
        (Integer/MAX_VALUE)
        (nth (nth l y) x)))
    
Get the coordinates of all 4 adjacent points

    (defn get-adj [l x y]
      (list (get-pos l (- x 1) y)
            (get-pos l (+ x 1) y)
            (get-pos l x (- y 1))
            (get-pos l x (+ y 1))))

Is a point a low point?

    (defn low? [l x y]
      (< (get-pos l x y) (apply min (get-adj l x y))))
  
Find all the low points by filtering all possible coordinates

    (defn get-low-points [l]
      (filter
       (fn [[x y]] (low? l x y))
       (for [x (range (count (first l)))
             y (range (count l))]
         [x y])))


Finally, bringing it all together for part 1

    (defn part1 [input]
      (let [l (parse-input input)]
        (->> l
             get-low-points
             (map (fn [[x y]] (get-pos l x y)))
             (map (partial + 1))
             (apply +))))

    (part1 test-input);; => 15
    (part1 real-input);; => 594


For part 2: Get the basin of a low point,
by recursively checking all its neighbours (and their neighbours and so on),
until it is either a 9 or an invalid coordinate.

    (defn get-basin [l low-point]
      (loop [solution #{} points [low-point] visited #{}]
        (if (= 0 (count points))
          solution
          (let [[x y] (first points) r (rest points)]
            (if (or
                 (>= (get-pos l x y) 9)
                 (contains? visited (list x y)))
              (recur solution r visited)
              (recur (conj solution (list x y))
                     (conj r
                           (list x (- y 1))
                           (list x (+ y 1))
                           (list (- x 1) y)
                           (list (+ x 1) y))
                     (conj visited (list x y)))))
          )))

Then we can just use the existing functions for part 2:


    (defn part2 [input]
      (let [l (parse-input input)]
        (->> l
             get-low-points
             (map (partial get-basin l))
             (map count)
             sort
             (take-last 3)
             (apply *)
             )))
    
    (part2 test-input);; => 1134
    (part2 real-input);; => 858494
