curl -X POST http://localhost:8080/api/v1/products 
-H 'Content-Type: application/json' 
-d '{"name":"test2","price":60, "origin": "here", "cultivar": "some"}'

curl -X POST http://localhost:8080/api/v1/users 
-H 'Content-Type: application/json' 
-d '{"first_name":"ab","last_name":"bb", "email": "some", "username": "x", "password":"x"}'

curl -X DELETE http://localhost:8080/api/v1/products/2
curl -X DELETE http://localhost:8080/api/v1/users/1

#fetch("http://localhost:8080/login", {
#    method: "post", 
#    headers:{"content-type": "application/json"}, 
#    body: JSON.stringify({username: "foo", password: "hunter2"})
#})
18:33:59.578 
curl -X POST http://localhost:8080/login -v -H 'Content-Type: application/json' -d '{"username":"x","password":"x"}'
