# Clojure

[full solution](https://github.com/x-zvf/programming-challenges/blob/master/adventofcode-2021/day04.clj)

Improvements and feedback is very much welcome, I am just a beginner in clojure.

## Parsing the input

This is done by first splitting by double newlines, to get the numbers list
and the boards seperated.
The numbers list is split by commas, and parsed into a list of numbers.

Each board is split into rows by newlines,
for each row, it is split by spaces, with each field being parsed into numbers,
resulting in a board being a list of lists,
therefore boards being a list of lists of lists.

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

## Determining the winning boards

Numbers are marked by replacing them with `-1`,
We iterate through all of the rows and columns of each
board and check if any column or row only consists
of `-1`. (NOTE: the reason it was done this way, was
because I thought that part 2 would require the values of the 
marked numbers, and I did not realize that `0` was a valid number.
Therefore originally I marked numbers by negating them)
If that is the case the board is a winning board.


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

## Playing Bingo

We iterate over each number, marking numbers by mapping over all items of all rows of all boards
and if the number matches the current number, substituting it with `-1`.

The boards get filtered by the function defined above, and if a winner is found
(and for part 2 if there's only one board left), we stop the iteration, because we have found the solution.

For part 2 this function takes `filter-on` which is used to filter out boards after each iteration.

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

## Scoring the winning solution

This is done by flattening the winner into a single dimensional list, removing all the marked (negative) numbers,
summing those up and multiplying them by the last number played.

    (defn score
      "Calculate the score of the winning board."
      [winner last-number]
      (->> winner
           (apply concat)
           (filter (fn [n] (< 0 n))) ; filter out all the marked numbers
           (apply +)
           (* last-number)
           ))


## Part 1

Now we have everything to solve part 1.

For part 1 we do not need to filter the boards after each round, so
we pass in a dummy function that just returns true for `filter-on` and
we do not need to have only one board remaining, so we pass in `false`.

    (defn part1 [in]
      (apply score
             (apply bingo
                    (fn [_ _] true)
                    false
                    (parse-input in))))

     (part1 test-input);; => 4512
     (part1 real-input);; => 65325

## Part 2

Since `bingo` is written such that it works for part 2, we only need a function for `filter-on`.

For this we need to determine if a colletion contains an item. Somehow I did not find the standard
library function for this (PLEASE REPLY, IF YOU KNOW!).
So I wrote this inefficient, stupid one:

    (defn collection-contains?
      "returns true if item is in collection, false otherwise. This really ought to be in the standard library."
      [coll item]
      (some (fn [x] (= item x)) coll))

Now we can bring all of this together for part 2:

    (defn part2 [in]
      (apply score
             (apply bingo
                    (fn [winners b]
                      (not (collection-contains? winners b))) ;; On each iteration, filter out all the winners
                    true
                    (parse-input in))))

    (part2 test-input);; => 1924
    (part2 real-input);; => 4624


## Any feedback or improvements are very much welcome

I am using AOC to learn clojure, therefore I am definitely not an expert and my code is probably crap.
