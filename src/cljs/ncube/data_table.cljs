(ns ncube.data-table
  (:require [reagent.core :as r]))

(defn- header-row
  [columns]
  [:tr
   (doall (map-indexed (fn [])))])

(defn data-table
  []
  (let [state (r/atom 0)]
    (fn [{:keys [columns data row-fn]}]
      [:table.w-100.collapse
       [:thead
        [:tr.bg-background
         (for [column columns]
           [:th.ba.b--local-workspace.b.tl column])]]
       [:tbody
        (doall (map #(row-fn %) data))]])))
