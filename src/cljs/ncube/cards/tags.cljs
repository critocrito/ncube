(ns ^:figwheel-hooks ncube.cards.tags
  (:require
   [reagent.core :as r]
   [devcards.core]
   [ncube.components :refer [tag]])
  (:require-macros
   [devcards.core :as dc :refer [defcard defcard-rg]]))

(defcard workspaces
  (r/as-element
   [:div {:class ["flex flex-column bg-buzzy-boop pa3"]}
    (tag {:label "Local Workspace" :style :local})
    (tag {:label "Remote Workspace" :style :remote})]))
