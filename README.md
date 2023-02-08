# Building
Execute *make* in the root directory

# Running
1. Start wasmcloud.azurecr.io/httpserver:0.17.0 and wasmcloud.azurecr.io/kvredis:0.16.3 providers
2. Start the current actor from file with 50 instances 
3. Link HTTP Server -> address=0.0.0.0:8080
4. Link Redis providers -> URL=redis://localhost:6379

# Testing 
1. ab -c 50 -t 30 http://localhost:8080/api/actor
2. ab -c 50 -t 30 http://localhost:8080/api/actor_log
3. ab -c 50 -t 30 http://localhost:8080/api/counter/one
4. ab -c 50 -t 30 http://localhost:8080/api/counter_log/two
5. ab -c 50 -t 30 http://localhost:8080/api/counter_twice/three
6. ab -c 50 -t 30 http://localhost:8080/api/counter_twice_log/four


