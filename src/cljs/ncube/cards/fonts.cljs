(ns ^:figwheel-hooks ncube.cards.fonts
  (:require
   [reagent.core :as r]
   [devcards.core])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))


(defcard header-fonts
  (r/as-element
   [:div {:class ["flex flex-column"]}
    [:h1 {:class ["fh1"]} "Header One"]
    [:h2 {:class ["fh2"]} "Header Two"]
    [:h3 {:class ["fh3"]} "Header Three"]
    [:h4 {:class ["fh4"]} "Header Four"]
    [:h5 {:class ["fh5"]} "Header Five"]]))

(defcard body-fonts
  (r/as-element
   [:div {:class ["flex flex-column"]}
    [:p {:class ["fb1"]} "A modular scale, like a musical scale, is a prearranged set of harmonious proportions."]
    [:p {:class ["fb2"]} "A modular scale, like a musical scale, is a prearranged set of harmonious proportions."]
    [:p {:class ["fb3"]} "A modular scale, like a musical scale, is a prearranged set of harmonious proportions."]]))
