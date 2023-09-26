#### rust_jobs website

The site is currently hosted at https://crajcan.dev/rust_jobs. This directory is served by github pages as a result of being a subdirectory of [personal_site](https://github.com/crajcan/crajcan.github.io)

This is a monorepo for managing the database, api layer and yew frontend. The Cargo project configured by the `Cargo.toml` in this directory is for the front end. The folders `api/` and `database/` describe how to manage the [dozer](https://getdozer.io) api layer and the postgres database.

### TODOs
- Make a spinner for when then yew app waits for the companies api to return
- Make a spinner for when the browser waits for the wasm and js to arrive
- Retry api requests when the api returns 0 companies
- Make a github action to ping the api every N minutes to keep it awake
- Put some jobs in the database
- Create an endpoint in the dozer config for the jobs table
- Add jobs to the app