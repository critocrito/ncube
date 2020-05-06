(ns ncube.workspaces.subscriptions
  (:require [re-frame.core :as rf]))

(rf/reg-sub
 ::workspace
 (fn [db]
   (:current-workspace db)))

