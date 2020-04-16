(ns ^:figwheel-hooks ncube.cards.colors
  (:require
   [reagent.core :as r]
   [devcards.core])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))

(defcard reagent-no-help
  (r/as-element [:h1 "Reagent component example"]))
