(ns day07)
(require 'clojure.string)

(def test-input "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k")

(def real-input (slurp "./inputs/day07.txt"))

(defn parse-input [input]
  (as-> input x
    (clojure.string/split x #"\$ ")
    (drop 1 x)
    (map #(clojure.string/split % #"\n") x)
    ))

(defn run-command [state in]
  (let [[cmd par] (clojure.string/split (first in) #" ") res (drop 1 in)]
    (cond (= cmd "cd")
          (assoc state :pwd ((fn [cwd s]
                         (cond (= s "/") []
                               (= s "..") (vec (drop-last 1 cwd))
                               :else (conj (vec cwd) (keyword s))))
                       (:pwd state) par))
          (= cmd "ls")
          (reduce (fn [state line]
                    ;(println "reduce =>" state " | " line)
                    (let [[_size name] (clojure.string/split line #" ")
                          size (read-string _size)]
                      (assoc-in state (into [:dirs] (conj (:pwd state) (keyword name))) (if (number? size) size {}))
                      )) state res)
          :else (println "Unknown command" cmd))))

(defn sum-part1 [t]
  (let [legal-sizes (atom [])]
  (defn sum-sizes-p1 [tree]
    (let [sizes
          (map (fn [key]
                 (let [size (if (number? (key tree)) (key tree)
                                (sum-sizes-p1 (key tree)))]
                   size)) (keys tree))
          total (apply + sizes)
          legal (<= total 100000)]
        (if legal
          (swap! legal-sizes conj total))
        total))
  (sum-sizes-p1 t)
  (apply + (deref legal-sizes))))

(defn sum-sizes [tree]
  (let [subdirs (filter (fn [key] (map? (key tree))) (keys tree))
        subdir-rv (map #(first (sum-sizes (% tree))) subdirs)
        subdir-sizes (map #(second (sum-sizes (% tree))) subdirs)
        subdir-total (apply + subdir-sizes)
        files (filter (fn [key] (number? (key tree))) (keys tree))
        file-sizes (map #(% tree) files)
        file-total (apply + 0 file-sizes)
        total (+ subdir-total file-total)]
    (list (conj (flatten subdir-rv) total) total)))

(defn part1 [input]
  (->> input
       parse-input
       (reduce run-command {})
       (#(sum-part1 (:dirs %)))
       ))

(sum-sizes {:a {:b {:c 10} :d 20}})

(conj '(30 10) 30)

(flatten [[1 2] [3 4] [5 6]])

(defn part2 [input]
  (->> input
       parse-input
       (reduce run-command {})
       (#(sum-sizes (:dirs %)))
       ((fn [[poss used]] (filter #(>= (+ % (- 70000000 used)) 30000000) poss)))
       (apply min)))

(part1 test-input)
(part1 real-input)
(part2 test-input)
(part2 real-input)