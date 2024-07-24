/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// eventbuzz/tests

// ----------------------------------------------------------------

use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;

use crate::asynchronous::prelude::*;

// ----------------------------------------------------------------

struct HelloEvent {
    message: String,
}

struct UpdateEvent {
    message: String,
}

// ----------------------------------------------------------------

impl ApplicationEvent for HelloEvent {
    fn topic() -> String {
        String::from("io.github.eventbuzz.global.hello.topic")
    }
}

impl ApplicationEvent for UpdateEvent {
    fn topic() -> String {
        String::from("io.github.eventbuzz.global.hello.topic")
    }
}

// ----------------------------------------------------------------

struct HelloEventListener;

struct GreetingEventListener;

struct UpdateEventListener;

// ----------------------------------------------------------------
#[async_trait]
impl AsyncApplicationEventListener<HelloEvent> for HelloEventListener {
    async fn on_application_event(&self, event: &HelloEvent) {
        sleep(Duration::from_secs(1)).await;
        println!(
            "async: thread.current.id: {:?}, HelloEventListener: Received event with message: {}",
            thread::current().id(),
            event.message
        );
    }
}

#[async_trait]
impl AsyncApplicationEventListener<HelloEvent> for GreetingEventListener {
    async fn on_application_event(&self, event: &HelloEvent) {
        sleep(Duration::from_secs(3)).await;
        println!("async: thread.current.id: {:?}, GreetingEventListener: Received event with message: {}", thread::current().id(), event.message);
    }
}

#[async_trait]
impl AsyncApplicationEventListener<UpdateEvent> for UpdateEventListener {
    async fn on_application_event(&self, event: &UpdateEvent) {
        sleep(Duration::from_secs(1)).await;
        println!(
            "async: thread.current.id: {:?}, UpdateEventListener: Received event with message: {}",
            thread::current().id(),
            event.message
        );
    }
}

// ----------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn test_async_eventbus_pub_sub() {
    let mut eventbus: AsyncEventbus =
        AsyncEventbus::builder() /* config or init | Unsupported now */
            .build();

    eventbus.register_listener(HelloEventListener).await;
    eventbus.register_listener(GreetingEventListener).await;
    eventbus.register_listener(UpdateEventListener).await;

    println!(
        "async: prepare.sync.publish.event, thread.current.id: {:?}",
        thread::current().id()
    );

    println!("async: --- prepare.async.publish.HelloEvent ---");

    eventbus
        .publish_event(HelloEvent {
            message: String::from("Hello, Rust!"),
        })
        .await;

    println!("async: --- prepare.async.publish.UpdateEvent ---");

    eventbus
        .publish_event(UpdateEvent {
            message: String::from("Hello, Rust!"),
        })
        .await;

    println!("async: --- post.async.publish.UpdateEvent ---");
}
