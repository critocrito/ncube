(ns ncube.sources.views
  (:require [reagent.core :as r]
            [re-frame.core :as rf]
            [fork.core :as fork]
            [ncube.components :refer [panel desc-text overline tag btn-large btn-small text-input]]
            [ncube.data-table :refer [data-table]]
            [ncube.sources.events :as events]
            [ncube.sources.subscriptions :as subscriptions]
            [ncube.workspaces.subscriptions :as workspaces]))

(defn create-source-form
  [slug {:keys [values
                form-id
                submitting?
                on-submit-response
                handle-change
                handle-blur
                handle-submit]}]
  [:form {:id form-id :on-submit handle-submit}
   [text-input {:name "type"
                :label "Source Type"
                :value (values "type")
                :on-change handle-change
                :on-blur handle-blur}]
   [:div.mt3
    [text-input {:name "term"
                 :label "Term "
                 :value (values "term")
                 :on-change handle-change
                 :on-blur handle-blur}]]
   [:div.flex.mt3
    [:div.mr3
     [btn-large {:label "Cancel"
               :on-click (fn [ev]
                           (.preventDefault ev)
                           (rf/dispatch [::events/list-sources slug]))
               :disabled submitting?
               :style :secondary}]]
    [btn-large {:label "Create"
                :disabled submitting?}]]
   [:p on-submit-response]])

(defn list-sources
  []
  (let [selected (r/atom [])
        workspace @(rf/subscribe [::workspaces/workspace])
        sources @(rf/subscribe [::subscriptions/sources])
        sidebar? @(rf/subscribe [:sidebar?])
        slug (:slug workspace)]
    (fn []
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
                         :on-click #(rf/dispatch [::events/sources-create slug])
                         :style :secondary}]]
            [btn-large {:label "Upload csv"
                        :on-click #(rf/dispatch [:unimplemented])
                        :style :secondary}]]
           [:div.w-50.flex.items-center
            [:div.ml-auto.mr2.b
              (if (= (count @selected) 0)
                ""
                (str (count @selected) " selected"))]
            [btn-large {:label "Send to process"
                         :on-click #(rf/dispatch [:unimplemented @selected])}]]]
          [data-table
           {:columns ["url" "type"]
            :on-select (fn [id]
                         (let [new-selected (if (some #(= id %) @selected)
                                              (do (remove #(= id %) @selected))
                                              (conj @selected id))]
                           (reset! selected new-selected)))
            :on-delete (fn [id] (rf/dispatch [::events/delete-source slug id]))
            :data sources}
           (fn [{:keys [id type term]}]
             [[:td.text-medium {:key (str "term-" id)} term]
              [:td.text-medium {:key (str "type-" id)} type]])]]])))

(defn create-source
  []
  (let [workspace @(rf/subscribe [::workspaces/workspace])
        sidebar? @(rf/subscribe [:sidebar?])]
    [panel
     {:workspace workspace
      :title "Create source"
      :description ""
      :sidebar? sidebar?}
     [fork/form {:path :form
                 :form-id "source-create-form"
                 :prevent-default? true
                 :clean-on-unmount? true
                 :on-submit-response {400 "client error"
                                      500 "server error"}
                 :on-submit #(rf/dispatch [::events/create-source (:slug workspace) %])}
      #(create-source-form (:slug workspace) %)]]))
