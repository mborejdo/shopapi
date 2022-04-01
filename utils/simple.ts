// const res = await fetch("http://localhost:8080/login", {
//   method: "post",
//   headers: { "content-type": "application/json" },
//   credentials: "include",
//   body: JSON.stringify({ username: "x", password: "x" }),
// });
// const data = res.headers.get("set-cookie")?.split(";")[0] || "";
// const resx = await fetch("http://localhost:8080/api/v1/products", {
//   method: "post",
//   headers: {
//     "content-type": "application/json",
//     "cookie": data,
//   },
//   credentials: "include",
//   body: JSON.stringify({
//     name: "test2",
//     price: 60,
//     origin: "here",
//     cultivar: "some",
//     images: "",
//   }),
// });
// console.log(await resx.text());
const res2 = await fetch("http://localhost:8080/api/v1/users", {
        method: "post",
        headers:{"content-type": "application/json"},
        body: JSON.stringify({
            first_name:"ab",
            last_name:"bb",
            email: "some",
            username: "x",
            password:"x"
        })
});

console.log(await res2.text())
