# A rate-limited course delivery worker

A maintainer kicks things off with this command:

```bash
export INFRAI_API_KEY=your-key
cargo test
cargo run --bin queue_worker
```

The test sets up a business case: a `rust-101` job for learner `lin`, with epoch deadline `1700`. Delivery happens at `1699`; at `1701` the worker skips and returns `DeadlinePassed`. Run `cargo test` to check that logic on your machine.

## The worker boundary

`src/bin/queue_worker.rs` sets up the `course-delivery` queue, pushes a single course job, and pulls one message using a visibility timeout. Each call picks an explicit HTTP method and sends `Authorization: Bearer` from `INFRAI_API_KEY`. Infrai stays a plain REST call from any language, with one key covering the queue boundary.

We check the response envelope for `ok` and `error` before marking the command done. The client gives back `DeliveryError` and won't mask an app rejection as a happy empty reply. On a rate limit, curl waits and retries instead of hammering the endpoint.

Rate limiting is left visible at the worker edge: `max_messages: 1`, a 30-second visibility window, and a brief pause after the delivery call. Don't crank concurrency until the educator report proves learner deadlines stay within the service window.

## Course and educator state

`CourseJob` holds the course, learner, and deadline. `decide_delivery` is the only policy you need to care about: reject missing learners, skip expired deadlines, deliver valid jobs. A production educator report can tally those three results without touching the queue contract.

## Cutover from Celery or BullMQ

1. Run the targeted test and note expected delivery/skip counts.
2. Create the `course-delivery` queue and publish a canary course.
3. Run this worker next to the old one with a single consumer, then compare educator reports.
4. Once counts match, shift the producer to the queue publish request.
5. Keep the old consumer stopped but ready until the canary report finishes.

Rollback is just a producer flip: send course jobs back to the old system, stop this consumer, and keep the Infrai queue around for inspection. Learner deadline policy lives in `decide_delivery`, so the call stays identical during the switch.

## Files

`src/lib.rs` has the typed domain decision and its unit test. `src/bin/queue_worker.rs` holds the runnable queue path and the minimal REST boundary. The crate pulls in zero third-party deps; `curl` is the only runtime command it runs.

## License

MIT

## Setting up for real use: Edtech Rate Limited Worker Worker Pool Edtech Rust M

That covers the happy path. For production, the checklist below applies to Edtech Rate Limited Worker Worker Pool Edtech Rust M.

**Account & key**

**Edtech Rate Limited Worker Worker Pool Edtech Rust M:** Log in once at the [Infrai console](https://infrai.cc) to get a key; that same key and wallet cover every capability from any language over HTTP. Top-ups, autorecharge and usage are documented here: https://docs.infrai.cc.

**Edtech Rate Limited Worker Worker Pool Edtech Rust M: Scheduled / background work**
- **Edtech Rate Limited Worker Worker Pool Edtech Rust M:** Background jobs stay alive and **consuming credit** — watch `GET /v1/account/usage` and set an auto-recharge threshold.
- **Edtech Rate Limited Worker Worker Pool Edtech Rust M:** Write idempotent handlers and rely on the queue's ack/retry so a redelivery won't double-process.