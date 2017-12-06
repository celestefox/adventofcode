(ns day5.core
  (:gen-class))

(defn wackyJumps
  "If the offset was 3+ dec it instead."
  [jump]
  (if (>= jump 3) (dec jump) (inc jump)))

(defn jmpEval
  "Finds when the jumps in jmps take you outside it."
  [jmps jumpChangeFn]
  (let [size (count jmps)]
    (loop [steps 0 position 0 vect jmps]
      (if (>= position size)
        steps
        (recur
         (inc steps)
         (+ position (vect position))
         (update vect position jumpChangeFn))))))

(defn parseJumpsStr
  "Parses the jumps contained in the passed string into a vector for jmpEval."
  [jumpStr]
  (->> jumpStr
       (clojure.string/split-lines)
       (map #(Integer/parseInt %))
       (vec)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [input (slurp (if args (first args) "input"))]
    (println (jmpEval (parseJumpsStr input) inc))
    (println (jmpEval (parseJumpsStr input) wackyJumps))))
