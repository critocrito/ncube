(ns ncube.workspaces.events
  (:require [re-frame.core :as rf]
            [re-frame.std-interceptors :refer [debug]]
            [day8.re-frame.tracing :refer-macros [fn-traced defn-traced]]
            [ajax.core :as ajax]
            [fork.core :as fork]))

(rf/reg-event-fx
 ::create-workspace
 (fn-traced
  [_ _]
  {:navigate! :workspaces-create}))

(rf/reg-event-fx
 ::link-workspace
 (fn-traced
  [_ _]
  {:navigate! :workspaces-link}))

(rf/reg-event-db
 ::workspaces-loaded
 (fn-traced
  [db [_ {:keys [data]}]]
  (assoc db :workspaces data)))

(rf/reg-event-fx
 ::workspace-fetched
 (fn-traced
  [{db :db} [_ {:keys [data]}]]
  {:navigate! [:workspace-details {:slug (:slug data)}]
   :db (assoc db :current-workspace data)}))

(rf/reg-event-fx
 ::success
 [(fork/clean :form)]
 (fn-traced
  [_ _]
  {:dispatch-n (list
                [::fetch-workspaces]
                [:navigate :home])}))

(rf/reg-event-fx
 ::failure
 (fn-traced
  [{db :db} [_ result]]
  {:db (-> db
           (fork/set-submitting :form false)
           (fork/set-status-code :form 500))}))

(rf/reg-event-fx
 ::submit-create-workspace-form
 [(fork/on-submit :form)]
 (fn-traced
  [{db :db} [_ {:keys [values]}]]
  (let [req-body {:workspace (values "workspace")
                  :kind "local"
                  :database "sqlite"
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
 ::submit-link-workspace-form
 [(fork/on-submit :form)]
 (fn-traced
  [{db :db} [_ {:keys [values]}]]
  (let [req-body {:name (values "name")
                  :description (values "description")
                  :workspace (values "workspace")
                  :kind "remote"
                  :endpoint (values "endpoint")
                  :database "http"
                  :account {:email (values "email")
                            :otp (values "otp")
                            :password (values "password")
                            :password_again (values "password-again")}}]
    {:db (fork/set-submitting db :form false)
     :http-xhrio
     {:method :post
      :uri "http://127.0.0.1:40666/api/workspaces"
      :params req-body
      :format (ajax/json-request-format)
      :response-format (ajax/json-response-format {:keywords? true})
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
