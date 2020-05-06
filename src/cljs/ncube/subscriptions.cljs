(ns ncube.subscriptions
  (:require [re-frame.core :as rf]))

(rf/reg-sub
 :current-route
 (fn [db]
   (:current-route db)))

(rf/reg-sub
 :workspaces
 (fn
   [db]
   (:workspaces db)))

(rf/reg-sub
 :sidebar?
 (fn
   [db]
   (:sidebar? db)))
