(ns ncube.utils
  (:require
   [reitit.frontend.easy :as rfe]))

(defn href
  "Return the relative url for a given route. The url can be used in html links."
  ([k]
   (href k nil nil))
  ([k params]
   (href k params nil))
  ([k params query]
   (rfe/href k params query)))
