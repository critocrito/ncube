(ns ncube.sources.views
  (:require [re-frame.core :as rf]
            [fork.core :as fork]
            [ncube.components :refer [panel desc-text overline tag btn-large btn-small text-input]]
            [ncube.data-table :refer [data-table]]
            [ncube.sources.events :as events]
            [ncube.sources.subscriptions :as subscriptions]
            [ncube.workspaces.subscriptions :as workspaces]))

(defn delete-source
  [slug id]
  [:span.trash
   {:on-click #(rf/dispatch [::events/delete-source slug id])}
   "D"])

(defn list-sources
  []
  (let [workspace @(rf/subscribe [::workspaces/workspace])
        sources @(rf/subscribe [::subscriptions/sources])
        sidebar? @(rf/subscribe [:sidebar?])]
    [panel
     {:workspace workspace
      :title "Sources"
      :description (:description workspace)
      :sidebar? sidebar?}
     [:div
      [:div.flex.justify-between.mb3
       [:div.w-50.flex
        [:div.mr2
         [btn-large {:label "Add new"
                     :on-click #(rf/dispatch [:unimplemented])
                     :style :secondary}]]
        [btn-large {:label "Upload csv"
                    :on-click #(rf/dispatch [:unimplemented])
                    :style :secondary}]]
       [:div.w-50.tr
        [btn-large {:label "Send to process"
                    :on-click #(rf/dispatch [:unimplemented])}]]]
      [data-table {:columns ["url" "type"]
                  :data sources
                  :row-fn (fn [{:keys [id type term]}]
                            [:tr {:key id}
                             [:td.text-medium
                              [delete-source (:slug workspace) id]
                              term]
                             [:td.text-medium type]])}]]]))


