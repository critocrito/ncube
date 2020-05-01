(ns ncube.workspaces.views
  (:require [re-frame.core :refer [subscribe dispatch]]
            [ncube.components :refer [desc-text overline tag btn-large btn-small]]))

(defn list-workspaces-item
  [workspace]
  (let [style (keyword (:kind workspace))
        label (if (= style :remote)
                "Remote Workspace"
                "Local Workspace")]
    [:li {:class "bb b--back-to-reality" :key (:id workspace)}
     [:div {:class "flex items-center justify-between w-100"}
      [:h3 {:class "header3 nowrap back-to-reality"}
       (:name workspace)]
      (tag {:label label :style style})
      [:div {:class "flex items-center justify-between"}
       [:div {:class "flex items-center"}
        [:div {:class "bg-back-to-reality white br-100 h1 w1 ba mr2"} "Q"]
        [:div {:class "b text-medium ttu back-to-reality"}
         "Queries: 200"]]
       [:div {:class "flex items-center"}
        [:div {:class "bg-back-to-reality white br-100 h1 w1 ba mr2"} "Q"]
        [:div {:class "b text-medium ttu back-to-reality"}
         "Queries: 200"]]
       [:div {:class "flex items-center"}
        [:div {:class "bg-back-to-reality white br-100 h1 w1 ba mr2"} "Q"]
        [:div {:class "b text-medium ttu back-to-reality"}
         "Queries: 200"]]
       [:div {:class "flex items-center"}
        [:div {:class "bg-back-to-reality white br-100 h1 w1 ba mr2"} "Q"]
        [:div {:class "b text-medium ttu back-to-reality"}
         "Queries: 200"]]]
      [:div {:class "ml1"}
       (btn-small {:label "Open"})]]]))

(defn list-workspaces
  []
  (let [workspaces @(subscribe [:workspaces])]
    [:div {:class "mw8 center ph3-ns"}
     [:div {:class "cf ph2-ns mt5"}
      [:div {:class "fl w-100 pa2"}
       [:img {:src "/images/logo_big.svg" :alt "Ncube logo."}]]
      [:div {:class "fl w-50"}
       (desc-text "Cumque ve lcorporis cum animi rerum. Adipisci fuga voluptas temporibus maxime. Nihil praesentium accusantium et.")]
      [:div {:class "fl w-100 pa2"}
       (overline "Workspaces")
       (into [:ul {:class "list pl0 mt0 mb0"}]
             (map #(list-workspaces-item %) workspaces))]
      [:div {:class "fl w-100 pa2"}
       [:div {:class "flex items-end fr"}
        (btn-large {:label "Link to remote workspace" :style :secondary})
        (btn-large {:label "Create a new workspace"})]]]]))
