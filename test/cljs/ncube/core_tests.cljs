(ns ncube.core-tests
  (:require [ncube.core :as sut]
            [cljs.test :as t :include-macros true]))

(t/deftest should-pass
  (t/is (not= 1 20)))

(t/deftest should-not-pass
  (t/is (= 1 20)))

(t/run-tests)
