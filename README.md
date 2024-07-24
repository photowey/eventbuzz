# `eventbuzz`

A safe, fast, event publish/subscribe system, where asynchronous events are implemented based on `tokio`, and inspired
by `Spring` events.

## 1. `Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
eventbuzz = "0.1"

# And
# If necessary
tokio = "${version}"
async-trait = "${version}"
```

## 2. `APIs`

### 2.1. `Sync`

> `use eventbuzz::sync::prelude::*;`

#### 2.1.1. `Event`

```rust
use eventbuzz::sync::prelude::*;

struct HelloEvent {
    message: String,
}

// ...

// ----------------------------------------------------------------

impl ApplicationEvent for HelloEvent {
    fn topic() -> String {
        // default: io.github.eventbuzz.global.default.topic
        // Unused now.
        String::from("io.github.eventbuzz.global.hello.topic")
    }
}
```

#### 2.1.2. `Listener`

```rust
struct HelloEventListener;

// ----------------------------------------------------------------

// HelloEvent -> This target event of Listener.
impl ApplicationEventListener<HelloEvent> for HelloEventListener {
    fn on_application_event(&self, event: &HelloEvent) {
        // Handle event.
    }
}
```

#### 2.1.3. `Publish`

```rust

// 1.Build an instance of Eventbus
// -> Maybe -> Eventbus::new() | unsupported now.
let mut eventbus: Eventbus = Eventbus::builder()
/* config or init | Unsupported now */
.build();

// 2.Register
// -> Auto register unsupported now.
eventbus.register_listener(HelloEventListener);
eventbus.register_listener(GreetingEventListener);

// 3.Publish event.
eventbus.publish_event(HelloEvent {
message: String::from("Hello, HelloEvent!"),
});

eventbus.publish_event(GreetingEvent {
message: String::from("Hello, GreetingEvent!"),
});

```

### 2.2. `Async`

> `use eventbuzz::asynchronous::prelude::*;`

#### 2.2.1. `Event`

```rust
use eventbuzz::asynchronous::prelude::*;

struct HelloEvent {
    message: String,
}

// ...

// ----------------------------------------------------------------

impl ApplicationEvent for HelloEvent {
    fn topic() -> String {
        // default: io.github.eventbuzz.global.default.topic
        // Unused now.
        String::from("io.github.eventbuzz.global.hello.topic")
    }
}
```

#### 2.2.2. `Listener`

> use `#[async_trait]`

```rust
struct HelloEventListener;

// ----------------------------------------------------------------

// Notes: #[async_trait]
// HelloEvent -> This target event of Listener.

#[async_trait]
impl AsyncApplicationEventListener<HelloEvent> for HelloEventListener {
    async fn on_application_event(&self, event: &HelloEvent) {
        // Handle event.
    }
}
```

2.2.3. `Publish`

```rust
// #[tokio::test(flavor = "multi_thread")]


// 1.Build an instance of Eventbus
// -> Maybe -> Eventbus::new() | unsupported now.
let mut eventbus: AsyncEventbus = AsyncEventbus::builder()
/* config or init | Unsupported now */
.build();

// 2.Register
// -> Auto register unsupported now.
eventbus.register_listener(HelloEventListener).await;
eventbus.register_listener(GreetingEventListener).await;

// 3.Publish event.
eventbus.publish_event(HelloEvent {
message: String::from("Hello, HelloEvent!"),
}).await;

eventbus.publish_event(GreetingEvent {
message: String::from("Hello, GreetingEvent!"),
}).await;
```

#### 2.3.4. `tokio::spawn`

```rust
let mut eventbus: AsyncEventbus = AsyncEventbus::builder()
/* config or init | Unsupported now */
.build();

eventbus.register_listener(HelloEventListener).await;
eventbus.register_listener(GreetingEventListener).await;

// Spawn
tokio::spawn( async move {
eventbus.publish_event(HelloEvent {
message: String::from("Hello, tokio.HelloEvent!"),
}).await;
}).await.unwrap();
```

```rust
// Arc<AsyncEventbus>

let mut eventbus: AsyncEventbus = AsyncEventbus::builder()
/* config or init | Unsupported now */
.build();

eventbus.register_listener(HelloEventListener).await;
eventbus.register_listener(GreetingEventListener).await;

let eventbus_arc = Arc::new(eventbus);

let eventbus_wrapped_1 = Arc::clone( & eventbus_arc);
tokio::spawn( async move {
eventbus_wrapped_1.publish_event(HelloEvent {
message: String::from("Hello, multi.tokio.arc.1.HelloEvent!"),
}).await;
}).await.unwrap();

let eventbus_wrapped_2 = Arc::clone( & eventbus_arc);
tokio::spawn( async move {
eventbus_wrapped_2.publish_event(HelloEvent {
message: String::from("Hello, multi.tokio.arc.2.HelloEvent!"),
}).await;
}).await.unwrap();

```
