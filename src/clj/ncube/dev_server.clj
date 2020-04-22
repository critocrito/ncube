(ns ncube.dev-server
  (:require
   [ring.middleware.resource :refer [wrap-resource]]))

(defn- wrap-default-index [handler]
  (fn [request]
    (let [response (handler request)]
      (if (not= (:status response) 404)
        response
        (handler (assoc request :uri "/index.html"))))))

(def handler
  (-> (fn [_] {:status 404 :body "index.html file found"})
      (wrap-resource "public")
      wrap-default-index))

