### Connect to supabase postgres instance: 

```
psql -h db.iniepalupghmmajzivqu.supabase.co -p 5432 -d postgres -U postgres
```

and enter the password from  ../api/.supabase_password

### Clearing walsenders from database: 

```
rust_jobs=> SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND datname = 'rust_jobs';
```
