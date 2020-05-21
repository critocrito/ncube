(ns ncube.data-table
  (:require [reagent.core :as r]))

(defn- delete-entity
  [click-handler]
  [:span.trash
   {:on-click click-handler}
   "D"])

(defn- header-row
  [columns]
  [:tr
   (doall (map-indexed (fn [])))])

(defn- row
  [{:keys [cell-data row-fn delete-handler select-handler]}]
  (let [id (:id cell-data)
        prefix (if select-handler
                 [:td.text-medium.tc {:key (str "delete-" id)}
                  [delete-entity #(delete-handler id)]
                  [:input {:type "checkbox" :on-click #(select-handler id)}]]
                 [:td.text-medium.tc
                  [delete-entity #(delete-handler id)]])
        cells (row-fn cell-data)]
    [:tr {:key id} (concat [prefix] cells)]))

(defn data-table
  [{:keys [columns data on-select on-delete]} row-fn]
  (let [state (r/atom 0)]
    (fn [{:keys [columns data on-select on-delete]} row-fn]
      (let [table
            (doall (map #(row {:cell-data % :row-fn row-fn :delete-handler on-delete :select-handler on-select})
                        data))]
        [:table.w-100.collapse
         [:thead
          [:tr.bg-background
           [:th.ba.b--local-workspace.b.tl ""]
           (for [column columns]
             [:th.ba.b--local-workspace.b.tl {:key column} column])]]
         [:tbody table]]))))
