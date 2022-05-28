(ns ser-deser
  (:require [cheshire.core :refer :all]))

(def config (parse-stream (clojure.java.io/reader "../config/example.json")))

(def config (assoc config "age" 32 "weight" 82.1))

(def master {"degree" "MS", "major" "Mechanical Engineering"})
(def config (assoc config "education" (conj (get config "education") master)))

(def here_tech {"title" "Senior Software Engineer", "location" "Berlin, Germany"})
(def config (assoc-in config ["experience" "HERE Technologies"] here_tech))

(generate-stream config (clojure.java.io/writer "../config/clj-example.json") {:pretty true})
