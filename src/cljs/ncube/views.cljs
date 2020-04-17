(ns ncube.views
  (:require [re-frame.core :refer [subscribe dispatch]]))

(defn ui
  []
  (let [counter @(subscribe [:counter])]
    [:div
     [:h1 "Hello Universe!"]
     [:p counter]
     [:button
      {:on-click #(dispatch [:inc-counter])}
      "Increment"]
     [:button
      {:on-click #(dispatch [:dec-counter])}
      "Decrement"]]))
