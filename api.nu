curl -X POST http://localhost:8080/api/v1/products -H 'Content-Type: application/json' -d '{"name":"test2","price":60, "origin": "here", "cultivar": "some"}'

curl --cookie "id=" -X POST http://localhost:8080/api/v1/users -H 'Content-Type: application/json' -d '{"first_name":"a","last_name":"aa", "email": "some", "username": "x", "password":"x"}'

curl -X DELETE http://localhost:8080/api/v1/products/2

//fetch("http://localhost:8080/login", {method: "post", headers:{"content-type": "application/json"}, body: JSON.stringify({username: "foo", password: "hunter2"})})
18:33:59.578 
curl -X POST http://localhost:8080/login -v -H 'Content-Type: application/json' -d '{"username":"aa","password":"password"}'


; SameSite=Strict; Secure; Path=/

