parsing the input:

    (defn parse-input [input]
      (-> input
          clojure.string/split-lines
          (->> (map seq))
          ))

Haskell has a transpose function, which would come in handy.
In clojure we can implement it like so:

    (defn transpose [m]
      (apply mapv vector m))

Calculating the rates using `frequencies` using the function `f` to determine if to pick `0` or `1` for the digit
and finally converting the result to a number.

	(defn calc-rate [f in]
	  (-> in
	      transpose
	      (->> (map frequencies)
	           (map (partial apply f second))
	           (map key)
	           (apply str)
	           )
	      (Long/parseLong 2)
	      ))

All together, for part 1: Passing `max-keys` to `calc-rate` for the $\gamma$ and `min-keys` for the $\epsilon$ rate.

	(defn part1 [in]
	  (let [inp (parse-input in)]
	    (* (calc-rate max-key inp) (calc-rate min-key inp))
	    ))
     
	(part1 test-input);; => 198
	(part1 real-input);; => 3633500

Unfortunately for part2, I did not find a way to reeuse `max-keys` or `min-keys`.

Determine, if to keep the numbers (`in`) that have the `n`-th digit being the most-common (if `x` is true) or the least common digit (if `x` is false)

	(defn to-keep [in n x] ; most common?
	  (let [freq (->> in (map (fn [a] (nth a n)))
	                  frequencies
	                  )]
		    (if x
		      (if (>= (get freq \1) (get freq \0)) \1 \0)
		      (if (<= (get freq \0) (get freq \1)) \0 \1)
		      )
		    ))


To get the ratings we repeatedly filter against that helper, until only one binary string is left, which we then parse into a number.
To the filter we pass in true, if it's the oxygen rating, false if it is the CO2

	(defn get-rating [input oxygen]
	  (-> (loop [nums input idx 0]
	        (if (= 1 (count nums))
	          (first nums)
	          (let [bit-to-keep (to-keep nums idx oxygen)]
	            (recur (filter #(= (nth % idx) bit-to-keep) nums) (inc idx))
	            )))
	      (->> (apply str))
	      (Long/parseLong 2)
	      ))

	(get-rating (parse-input test-input) true);; => 23
	(get-rating (parse-input test-input) false);; => 10

Putting all of these together for part2

	(defn part2 [in]
	  (let [input (parse-input in)]
	    (* (get-rating input true) (get-rating input false))))

	(part2 test-input);; => 230
	(part2 real-input);; => 4550283