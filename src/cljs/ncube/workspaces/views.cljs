(ns ncube.workspaces.views
  (:require [re-frame.core :as rf]
            [fork.core :as fork]
            [ncube.components :refer [panel desc-text overline tag btn-large btn-small text-input]]
            [ncube.workspaces.events :as events]
            [ncube.workspaces.subscriptions :as subscriptions]))

(defn workspace-stat
  [type value]
  (let [icon_url (str "/images/icon_" (name type) ".svg")
        text (cond
               (= type :data) "Units"
               (= type :query) "Queries"
               (= type :investigation) "Investigations"
               (= type :process) "Processes"
               :else "Don't know")]
    [:div {:class "ml2 mr2 flex items-center"}
     [:img {:src icon_url :class "mr1"}]
     [:div {:class "b text-medium ttu back-to-reality"}
      (str text ": " value)]]))

(defn list-workspaces-item
  [workspace]
  (let [id (:id workspace)
        slug (:slug workspace)
        style (keyword (:kind workspace))
        label (if (= style :remote)
                "Remote Workspace"
                "Local Workspace")]
    [:li {:class "bb b--back-to-reality" :key id}
     [:div {:class "flex items-center justify-between w-100"}

      [:h3 {:class "header3 nowrap back-to-reality"}
       (:name workspace)]
      (tag {:label label :style style})

      [:div {:class "flex items-center justify-between"}
       (workspace-stat :query "666")
       (workspace-stat :data "667")
       (workspace-stat :process "42")
       (workspace-stat :investigation "23")]

      [:div {:class "ml1"}
       (btn-small {:label "Open"
                   :on-click #(rf/dispatch [::events/show-workspace slug])})]]]))

(defn list-workspaces
  []
  (let [workspaces @(rf/subscribe [:workspaces])]
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
        (btn-large {:label "Link to remote workspace"
                    :on-click #(rf/dispatch [::events/create-workspace])
                    :style :secondary})
        (btn-large {:label "Create a new workspace"
                    :on-click #(rf/dispatch [::events/create-workspace])})]]]]))

(defn create-workspace-form
  [{:keys [values
           form-id
           submitting?
           on-submit-response
           handle-change
           handle-blur
           handle-submit]}]
  [:form {:id form-id :on-submit handle-submit}
   (text-input {:name "name"
                :label "Workspace Name"
                :value (values "name")
                :on-change handle-change
                :on-blur handle-blur})
   (text-input {:name "description"
                :label "Description "
                :value (values "description")
                :on-change handle-change
                :on-blur handle-blur})
   [btn-large {:label "Cancel"
               :on-click (fn [ev]
                           (.preventDefault ev)
                           (rf/dispatch [:navigate :home]))
               :disabled submitting?
               :style :secondary}]
   [btn-large {:label "Create"
               :disabled submitting?}]
   [:p on-submit-response]])

(defn create-workspaces
  []
  [:div {:class "mw8 center ph3-ns"}
   [:div {:class "fl w-100 pa2"}
    [:h1 {:class "fh1"} "Welcome to Ncube!"]
    [:p {:class "fb1"} "Before you can start, please fill in some basic configuration."]
    [fork/form {:path :form
                :form-id "workspace-create-form"
                :prevent-default? true
                :clean-on-unmount? true
                :on-submit-response {400 "client error"
                                     500 "server error"}
                :on-submit #(rf/dispatch [::events/submit-create-workspace-form %])}
     create-workspace-form]]])

(defn card
  [kind]
  (let [description "I'm some sort of description. What I will be, I don't know yet. But I'm convinced, it will be mganificient."
        [title icon label event]
        (cond
          (= kind :queries) ["Queries" "icon_query.svg" "Manage" :unimplemented]
          (= kind :data) ["Data" "icon_data.svg" "Explore" :unimplemented]
          (= kind :processes) ["Processes" "icon_process.svg" "Set Up" :unimplemented]
          (= kind :investigations) ["Investigations" "icon_investigation.svg" "Verify" :unimplemented])]
    [:div {:class "h4 bg-white shadow-1 flex items-center justify-between mb4"}
     [:div {:class "w-40 pa2"}
      [:div {:class "flex flex-column pl2"}
       [:div {:class "flex h3 items-center justify-between w-100"}
        [:div {:class "flex items-center"}
         [:img {:class "h2 w2"
                :src (str "http://localhost:9500/images/" icon)
                :alt (str "Icon for " title)}]
         [:h4 {:class "header4 pl2"} title]]
        ]
       [:p {:class "text-small"}
        description]]]
     [:div {:class "h3 pr2"} [btn-small {:label label :on-click #(rf/dispatch [event])}]]]))

(defn show-workspace
  []
  (let [workspace @(rf/subscribe [::subscriptions/workspace])
        sidebar? @(rf/subscribe [:sidebar?])]
    [panel
     {:workspace workspace
      :title (:name workspace)
      :description (:description workspace)
      :sidebar? sidebar?}
     [:div
      [card :queries]
      [card :data]
      [card :processes]
      [card :investigations]]]))
