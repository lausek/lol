(def main ()
    (serve "localhost:8080" "callback"))

(def callback (request)
    (let search_result (list "a" "b"))

    (let template (read_all (open_file "example/server-template.html")))
    (let page (format template search_result))

    (ret (list 200 "text/html" page)))
