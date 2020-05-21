(ns ncube.sources.subscriptions
  (:require [re-frame.core :as rf]))

(rf/reg-sub
 ::sources
 (fn [db]
   (:sources db)))

