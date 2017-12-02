(ns day1.core
  (:gen-class))

(defn zip
  "A quick zip/zipWith implementation"
  [& colls]
  (partition (count colls) (apply interleave colls)))

(defn matchsum
  "Finds the sum of all digits that match the next digit in a string.
  (It \"wraps\" around, so that the last digit must match the first to be added)"
  [x]
  (->> x
       seq
       (map (fn [^Character c] (Character/digit c 10)))
       ((fn [y] (zip y (next (cycle y)))))
       (map (fn [v] (if (= (first v) (second v)) (first v) 0)))
       ;; Add things that do the logic here
       (reduce +)))

(defn -main
  "Lets you run matchsum from the command line."
  [& args]
  (let [file (if args (first args) "input")]
    (println (matchsum (clojure.string/trim (slurp file))))))
