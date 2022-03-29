# $ cargo install sqlx-cli --no-default-features --features native-tls,postgres
DATABASE_URL=postgres://wwwdata:wwwdata@em-chef-appserver.em.lan:5432/mibtest sqlx database create
sqlx migrate run


sqlx database create --database-url postgres://wwwdata:wwwdata@em-chef-appserver.em.lan:5432/mibtest
sqlx migrate run --database-url postgres://wwwdata:wwwdata@em-chef-appserver.em.lan:5432/mibtest