(ns day4.core
  (:gen-class))

(defn splitWhitespace
  "Splits on unicode whitespace."
  [string]
  (clojure.string/split string #"\p{Space}"))

(defn passphrase?
  "Checks if the given passphrase is valid."
  [passphrase]
  (let [splitPassphrase (seq (splitWhitespace passphrase))]
    (= splitPassphrase (distinct splitPassphrase))))

(defn trueToNum
  "Translates true to 1 and false to 0."
  [maybeTrue]
  (if maybeTrue 1 0))

(defn validPassphrases
  "Returns how many passphrases of sequence are valid."
  [sequence]
  (->> sequence
       (clojure.string/split-lines)
       (map clojure.string/trim)
       (map passphrase?)
       (map trueToNum)
       (reduce +)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [filename (if args (first args) "input")]
    (let [contents (clojure.string/trim (slurp filename))]
      (println (validPassphrases contents)))))
