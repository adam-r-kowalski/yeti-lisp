![Yeti Lisp Logo](assets/logo.png)

```clj
; this is a comment

5 ; this is an integer

3.14 ; this is a float

"Hello, World!" ; this is a string

true ; this is a boolean

false ; this is a boolean

nil ; this is nil, it represents nothing

[1 2 3] ; this is an array

{:first "John" :last "Doe"} ; this is a map

(+ 1 2) ; this is a function call it evaluates to 3

(def x 1) ; this is a variable declaration

(+ x 10) ; this evaluates to 11

(defn square [x] (* x x)) ; this is a function declaration

(square 5) ; this evaluates to 25

(if true "yes" "no") ; this evaluates to "yes"

(if false "yes" "no") ; this evaluates to "no"

(if nil "yes" "no") ; this evaluates to "no"

(def p {:first "John" :last "Doe"}) ; this is a map

(:first p) ; this evaluates to "John"

(:last p) ; this evaluates to "Doe"

(:middle p) ; this evaluates to nil

(def p2 (assoc p :middle "Q")) ; this is a new map

(:middle p2) ; this evaluates to "Q"

(:middle p) ; this evaluates to nil as p is unchanged

(def p3 (dissoc p :last)) ; this is a new map

(:last p3) ; this evaluates to nil

(:last p) ; this evaluates to "Doe" as p is unchanged

(def xs [1 4 9]) ; this is an array

(nth xs 0) ; this evaluates to 1

(nth xs 1) ; this evaluates to 4

(nth xs 2) ; this evaluates to 9

(+ x y) ; this is an error as y is not defined

'(+ x y) ; this is a quoted expression and evaluates to (+ x y)

(def ast '(+ x y)) ; this is a quoted expression and evaluates to (+ x y)

(eval ast) ; this is an error as y is not defined

(def y 2) ; this is a variable declaration

(eval ast) ; this evaluates to 3

(read-string "(+ x y)") ; this evaluates to (+ 1 2)

(eval (read-string "(+ x y)")) ; this evaluates to 3

(assert (= '(+ x y) (read-string "(+ x y)"))) ; this is true

(def a {:head 5 :tail nil}) ; this is a linked list

(def b {:head 7 :tail a}) ; we leverage structural sharing

(def c {:head 9 :tail b}) ; we leverage structural sharing

(:head c) ; this evaluates to 9

(:head (:tail c)) ; this evaluates to 7

(:head (:tail (:tail c))) ; this evaluates to 5

; here we define a function that sums the elements of a linked list
; using pattern matching
(defn list-sum
 ; this pattern matches a linked list with a head and a tail
 ([{:head h :tail t}] (+ h (list-sum t)))

 ; this pattern matches the empty list whose sum is 0
 ([nil] 0))


(list-sum c) ; this evaluates to 21


(let [d 10  ; this binds d to 10
      e 20] ; this binds e to 20
 (+ d e)) ; this evaluates to 30

d ; this is an error as d is not defined

e ; this is an error as e is not defined

; let bindings can also levearge pattern matching
(let [{:head h} c]
 h) ; this evaluates to 9

; there are several built in modules that help with common tasks
; first lets learn about sql

(def db (sql/connect)) ; this creats an in memory database

; this creates a table called people with three columns
(sql/execute! db
 {:create-table :people
  :with-columns [[:name :text]
                 [:age :integer]
                 [:job :text]]})

; this inserts four rows into the table
(sql/execute! db
 {:insert-into :people
  :columns [:name :age :job]
  :values [["John" 30 "Developer"]
           ["Jane" 25 "Designer"]
           ["Jack" 40 "Manager"]
           ["Jill" 35 "Engineer"]]})


; this evaluates to [{:name "John" :age 30 :job "Developer"}]
(sql/query db
 {:select :*
  :from :people
  :where [:= :name "John"]})


; this evaluates to [{:name "Jack" :age 40 :job "Manager"}
;                    {:name "Jill" :age 35 :job "Engineer"}]
(sql/query db
 {:select :*
  :from :people
  :where [:> :age 30]})


; notice how the entire api is built around data structures


; now lets learn about http servers

(def home
 [:html
  [:head
   [:title "Home"]
   [:style
    {:body {:font-family "sans-serif"}}]]
  [:body
   [:h1 "Home"]
   [:p "Welcome to my website!"]]])


(def app
 (server/start {:port 8080
                :routes {"/" home}}))
; this starts a server on port 8080 that serves the home page


(server/stop app) ; this stops the server


; now lets create a simple web app that uses the database we created earlier

(defn req->query
 ([{:query {:job job}}]
  {:select :*
   :from :people
   :where [:= :job job]})
 ([_] {:select :*
       :from :people}))


(sql/query db (req->query {:query {:job "Developer"}}))
; this evaluates to [{:name "John" :age 30 :job "Developer"}]

(sql/query db (req->query {}))
; this evaluates to [{:age 30, :job "Developer", :name "John"},
;                    {:age 25, :job "Designer", :name "Jane"},
;                    {:age 40, :job "Manager", :name "Jack"},
;                    {:age 35, :job "Engineer", :name "Jill"}]


(defn home [req]
 [:html
  [:head
   [:title "Home"]
   [:style
    {:body {:font-family "sans-serif"}}]]
  [:body
   [:ul
    (let [employees (sql/query db (req->query req))]
     (for [{:name name :age age :job job} employees]
      [:li [:strong name] ", " job ", " age " years old"]))]]])
 

(def app
 (server/start {:port 8080
                :routes {"/" home}}))

; if the request is made with no query parameters we render all employees
; if the request is made with a job query parameter we render only those employees
```
