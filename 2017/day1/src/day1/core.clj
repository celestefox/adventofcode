(ns day1.core
  (:gen-class))

(defn zip
  "A quick zip/zipWith implementation"
  [& colls]
  (partition (count colls) (apply interleave colls)))

(defn matchsum
  "Finds the sum of all digits that match the digit distance away, wrapping around."
  [x distance]
  (->> x
       seq
       (map (fn [^Character c] (Character/digit c 10)))
       ((fn [y] (zip y (nthnext (cycle y) distance))))
       (map (fn [v] (if (= (first v) (second v)) (first v) 0)))
       ;; Add things that do the logic here
       (reduce +)))

(defn -main
  "Lets you run matchsum from the command line."
  [& args]
  (let [file (if args (first args) "input")]
    (let [contents (clojure.string/trim (slurp file))]
      (println (matchsum contents 1))
      (println (matchsum contents (/ (count contents) 2))))))
