(ns ncube.workspaces.events
  (:require [re-frame.core :as rf]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [ajax.core :as ajax]
            [fork.core :as fork]))

(rf/reg-event-fx
 ::create-workspace
 (fn-traced
  [_ _]
  {:navigate! :workspaces-create}))

(rf/reg-event-db
 ::workspaces-loaded
 (fn-traced
  [db [_ workspaces]]
  (assoc db :workspaces workspaces)))

(rf/reg-event-fx
 ::success
 [(fork/clean :form)]
 (fn-traced
  [_ [_ result]]
  {:dispatch [:fetch-workspaces]
   :navigate! :home}))

(rf/reg-event-fx
 ::workspace-fetched
 (fn-traced
  [{db :db} [_ workspace]]
  {:navigate! [:workspace-details {:slug (:slug workspace)}]
   :db (assoc db :current-workspace workspace)}))

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

(rf/reg-event-fx
 ::fetch-workspaces
 (fn-traced
  [_ _]
  {:http-xhrio {:method :get
                :uri "http://127.0.0.1:40666/api/workspaces"
                :response-format (ajax/json-response-format {:keywords? true})
                :on-success [::workspaces-loaded]
                :on-failure [:http-error]}}))

(rf/reg-event-fx
 ::show-workspace
 (fn-traced
  [_ [_ slug]]
  {:http-xhrio {:method :get
                :uri (str "http://127.0.0.1:40666/api/workspaces/" slug)
                :response-format (ajax/json-response-format {:keywords? true})
                :on-success [::workspace-fetched]
                :on-failure [:http-error]}}))
