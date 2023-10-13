# OTEL exampley thing

Note: restarting the Jaeger container will wipe its memory unless you set

## Startup

1. Start Jaeger to collect traces: `docker-compose up`
2. Run the example code: `cargo run`
3. Open Jaeger: <http://localhost:16686/>
4. Set the service to "example"
5. Click "Find Traces"

## Other filters

* You can filter for traces with errors in them by adding `error=true` in the tags box.
* Try setting "Max Duration" to 0.00004s
* Filter by specific instrumented operations, to show traces containing this operation.
  * `do_stuff`
  * `possibly_slow_thing`
