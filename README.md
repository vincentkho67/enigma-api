to run

```
docker compose up -d
```

```
docker compose exec app diesel migration run
docker compose exec app cargo run
```

to init role
```
docker compose exec app cargo run -bin cli roles create {role_code} {role_name}
```

to init admin user after creating admin role:
```
docker compose exec app cargo run -bin cli users create {email} {password} {role_code}
```