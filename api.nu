curl -X POST http://localhost:8080/api/v1/products -H 'Content-Type: application/json' -d '{"name":"test2","price":60, "origin": "here", "cultivar": "some"}'

curl -X DELETE http://localhost:8080/api/v1/products/2

curl -X POST http://localhost:8080/login -H 'Content-Type: application/json' -d '{"username":"aa","password":"hunter"}'