(ns ncube.onboarding.views
  (:require [re-frame.core :as rf]
            [day8.re-frame.tracing :refer-macros [fn-traced]]
            [day8.re-frame.http-fx]
            [ajax.core :as ajax]
            [fork.core :as fork]
            [ncube.components :as c]))

(defn cfg->body
  [cfg]
  (->> cfg
       (into [])
       (map (fn [[n v]] {:name (name n) :value (str v)}))))

(rf/reg-event-fx
 ::success
 [(fork/clean :form)]
 (fn-traced
  [{db :db} [_ result]]
  {:db (assoc db :result result)
   :dispatch [:boot]}))


(rf/reg-event-fx
 ::failure
 (fn-traced
  [{db :db} [_ result]]
  (js/console.log result)
  {:db (-> db
           (fork/set-submitting :form false)
           (fork/set-status-code :form 500))}))

(rf/reg-event-fx
 ::submit-onboarding-form
 [(fork/on-submit :form)]
 (fn-traced
  [{db :db} [_ {:keys [values]}]]
  {:db (fork/set-submitting db :form false)
   :http-xhrio
   {:method :post
    :uri "http://127.0.0.1:40666/api"
    :params (cfg->body values)
    :format (ajax/json-request-format)
    :response-format (ajax/raw-response-format)
    :on-success [::success]
    :on-failure [::failure]}}))

(defn form
  [{:keys [values
           form-id
           submitting?
           on-submit-response
           handle-change
           handle-blur
           handle-submit]}]
  
  [:form {:id form-id :on-submit handle-submit}
   (c/text-input {:name "workspace_root"
                  :label "Workspace Root Directory"
                  :value (values "workspace_root")
                  :on-change handle-change
                  :on-blur handle-blur})
   (c/text-input {:name "name"
                  :label "What is your name?"
                  :value (values "name")
                  :on-change handle-change
                  :on-blur handle-blur})
   (c/text-input {:name "email"
                  :label "What is your email?"
                  :value (values "email")
                  :on-change handle-change
                  :on-blur handle-blur})
   [:button {:class "btn-primary" :type :submit :disabled submitting?} "Continue"]
   [:p on-submit-response]])

(defn panel
  []
  [:div {:class "mw8 center ph3-ns"}
   [:div {:class "fl w-100 pa2"}
    [:h1 {:class "fh1"} "Welcome to Ncube!"]
    [:p {:class "fb1"} "Before you can start, please fill in some basic configuration."]
    [fork/form {:path :form
                :form-id "onboarding-form"
                :prevent-default? true
                :clean-on-unmount? true
                :on-submit-response {400 "client error"
                                     500 "server error"}
                :on-submit #(rf/dispatch [::submit-onboarding-form %])
                :initial-values {"workspace_root" "~/Ncube"}} form]]])
