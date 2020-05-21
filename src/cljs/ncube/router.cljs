(ns ncube.router
  (:require
   [re-frame.core :refer [dispatch]]
   [reitit.core :as r]
   [reitit.frontend :as rf]
   [reitit.coercion.spec :as rss]
   [reitit.frontend.easy :as rfe]
   [ncube.views :refer [home-panel]]
   [ncube.onboarding.views :as onboarding]
   [ncube.workspaces.views :as workspaces]
   [ncube.sources.views :as sources]))

(def routes
  ["/"
   ["create" {:name :workspaces-create :view workspaces/create-workspaces}]
   ["w"
    ["/" {:name :home :view workspaces/list-workspaces}]
    ["/:slug"
     ["/" {:name :workspace-details :view workspaces/show-workspace}]
     ["/sources"
      ["/" {:name :sources-list :view sources/list-sources}]
      ["/create" {:name :sources-create :view sources/create-source}]]]]
   ["onboarding" {:name :onboarding :view onboarding/panel}]])

(defn on-navigate
  [new-match]
  (when new-match
    (dispatch [:navigated new-match])))

(def router
  (rf/router
   routes
   {:data {:coercion rss/coercion}}))

(defn init-routes!
  []
  (js/console.log "initializing routes")
  (rfe/start!
   router
   on-navigate
   {:use-fragment false}))
