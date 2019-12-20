$ curl -F 'file=@test.csv;type=text/csv' http:/localhost:3000/csv
$ RUST_LOG=server=debug cargo run
$ curl -v -H 'Content-Type: application/json' -d'{"user_agent":"Mozilla","response_time":200}' http://localhost:3000/logs
$ curl http://localhost:3000/logs
$ curl http://localhost:3000/csv > res.csv
$ curl -H'Accept-Encoding:deflate,gzip' http://localhost:3000/csv > response.csv.gz

