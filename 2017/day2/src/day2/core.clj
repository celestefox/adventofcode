(ns day2.core
  (:gen-class)
  (:require [clojure.math.combinatorics :as combo]))

(defn parseint
  [i]
  (Integer/parseInt i))

(defn parseinput
  "Takes the spreadsheet input and turns it into a list of lists."
  [s]
  (->> s
       clojure.string/split-lines
       (map clojure.string/trim)
       (map #(clojure.string/split % #"\p{Space}"))
       (map #(map clojure.string/trim %))
       (map #(map parseint %))))

(defn checksum-difference
  [ls]
  (- (apply max ls) (apply min ls)))

(defn checksum-divisible
  [ls]
  (->> (combo/cartesian-product ls ls)
       (map #(if (and (zero? (rem (first %) (second %))) (not= (first %) (second %))) (/ (first %) (second %)) nil))
       (remove nil?)
       distinct
       first))

(defn checksum
  "Calculates the \"Checksum\" of the input."
  [spreadsheet meth]
  (reduce + (map
             meth
             spreadsheet)))

(defn -main
  "Runs checksum from the command line."
  [& args]
  (let [file (if args (first args) "input")]
    (let [contents (slurp file)]
      (println (checksum (parseinput contents) checksum-difference))
      (println (checksum (parseinput contents) checksum-divisible)))))
