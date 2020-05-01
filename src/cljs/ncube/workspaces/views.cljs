(ns ncube.workspaces.views
  (:require [re-frame.core :as rf :refer [subscribe dispatch]]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [ajax.core :as ajax]
            [fork.core :as fork]
            [ncube.components :refer [desc-text overline tag btn-large btn-small text-input]]))

(rf/reg-event-fx
 ::create-workspace
 (fn-traced
  [_ _]
  {:navigate! :workspaces-create}))

(rf/reg-event-fx
 ::success
 [(fork/clean :form)]
 (fn-traced
  [_ [_ result]]
  {:dispatch [:fetch-workspaces]
   :navigate! :home}))


(rf/reg-event-fx
 ::failure
 (fn-traced
  [{db :db} [_ result]]
  (js/console.log result)
  {:db (-> db
           (fork/set-submitting :form false)
           (fork/set-status-code :form 500))}))

(rf/reg-event-fx
 ::submit-create-workspace-form
 [(fork/on-submit :form)]
 (fn-traced
  [{db :db} [_ {:keys [values]}]]
  (let [req-body {:name (values "name")
                  :kind "local"
                  :location "~/Ncubed/xxx"
                  :description (values "description")}]
    {:db (fork/set-submitting db :form false)
   :http-xhrio
   {:method :post
    :uri "http://127.0.0.1:40666/api/workspaces"
    :params req-body
    :format (ajax/json-request-format)
    :response-format (ajax/raw-response-format)
    :on-success [::success]
    :on-failure [::failure]}})))

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
        (btn-large {:label "Link to remote workspace"
                    :on-click #(dispatch [::create-workspace])
                    :style :secondary})
        (btn-large {:label "Create a new workspace"
                    :on-click #(dispatch [::create-workspace])})]]]]))

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
   (btn-large {:label "Create" :type :submit :disabled submitting?})
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
                :on-submit #(rf/dispatch [::submit-create-workspace-form %])}
     create-workspace-form]]])
