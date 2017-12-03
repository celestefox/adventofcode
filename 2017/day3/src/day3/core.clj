(ns day3.core
  (:gen-class))

(defn closestSquare
  "Finds the closest perfect square we care about."
  [i]
  (let [j (int (Math/sqrt i))]
    (if (odd? j) j (dec j))))

(defn gridDistance
  "Finds the distance on the grid to the center."
  [gridIndex]
  (let [closestCornerCIndex (closestSquare (dec gridIndex))]
    (let [spiralDistance (- gridIndex (Math/pow closestCornerCIndex 2))]
      (let [sidePosition (mod spiralDistance (inc closestCornerCIndex))
            centerSideDistance (int (/ (inc closestCornerCIndex) 2))]
        (+ centerSideDistance (Math/abs (- sidePosition centerSideDistance)))))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (gridDistance (Integer/parseInt (first args)))))
