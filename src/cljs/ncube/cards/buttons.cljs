(ns ^:figwheel-hooks ncube.cards.buttons
  (:require
   [reagent.core :as r]
   [devcards.core])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))


(defcard primary-button
  (r/as-element
   [:div {:class ["flex flex-column"]}
    [:a {:class ["btn-primary"]} "Primary Button"]
    [:a {:class ["btn-secondary"]} "Secondary Button"]
    [:a {:class ["btn-caution"]} "Caution Button"]
    [:a {:class ["btn-disabled"] :disabled true} "Disabled Button"]]))
