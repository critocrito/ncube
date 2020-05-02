(ns ncube.onboarding.views
  (:require [re-frame.core :as rf]
            [day8.re-frame.tracing :refer-macros [fn-traced]]
            [day8.re-frame.http-fx]
            [ajax.core :as ajax]
            [fork.core :as fork]
            [ncube.components :refer [text-input btn-large]]
            [ncube.onboarding.events :as events]))

(defn form
  [{:keys [values
           form-id
           submitting?
           on-submit-response
           handle-change
           handle-blur
           handle-submit]}]
  
  [:form {:id form-id :on-submit handle-submit}
   (text-input {:name "workspace_root"
                  :label "Workspace Root Directory"
                  :value (values "workspace_root")
                  :on-change handle-change
                  :on-blur handle-blur})
   (text-input {:name "name"
                  :label "What is your name?"
                  :value (values "name")
                  :on-change handle-change
                  :on-blur handle-blur})
   (text-input {:name "email"
                  :label "What is your email?"
                  :value (values "email")
                  :on-change handle-change
                :on-blur handle-blur})
   (btn-large {:label "Continue" :type :submit :disabled submitting?})
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
                :on-submit #(rf/dispatch [::events/submit-onboarding-form %])
                :initial-values {"workspace_root" "~/Ncube"}} form]]])
