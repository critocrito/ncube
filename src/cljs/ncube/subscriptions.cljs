(ns ncube.subscriptions
  (:require [re-frame.core :refer [reg-sub]]))

(reg-sub
 :counter
 (fn
  [{:keys [count]}]
  count))

