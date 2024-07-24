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
use std::thread::sleep;
use std::time::Duration;

use crate::sync::prelude::*;

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

impl ApplicationEventListener<HelloEvent> for HelloEventListener {
    fn on_application_event(&self, event: &HelloEvent) {
        sleep(Duration::from_secs(1));
        println!(
            "sync: thread.current.id: {:?}, HelloEventListener: Received event with message: {}",
            thread::current().id(),
            event.message
        );
    }
}

impl ApplicationEventListener<HelloEvent> for GreetingEventListener {
    fn on_application_event(&self, event: &HelloEvent) {
        sleep(Duration::from_secs(3));
        println!(
            "sync: thread.current.id: {:?}, GreetingEventListener: Received event with message: {}",
            thread::current().id(),
            event.message
        );
    }
}

impl ApplicationEventListener<UpdateEvent> for UpdateEventListener {
    fn on_application_event(&self, event: &UpdateEvent) {
        sleep(Duration::from_secs(1));
        println!(
            "sync: thread.current.id: {:?}, UpdateEventListener: Received event with message: {}",
            thread::current().id(),
            event.message
        );
    }
}

// ----------------------------------------------------------------

#[test]
#[cfg(feature = "synchronous")]
fn test_eventbus_pub_sub() {
    let mut eventbus: Eventbus = Eventbus::builder() /* config or init | Unsupported now */
        .build();

    eventbus.register_listener(HelloEventListener);
    eventbus.register_listener(GreetingEventListener);
    eventbus.register_listener(UpdateEventListener);

    println!(
        "sync: prepare.sync.publish.event, thread.current.id: {:?}",
        thread::current().id()
    );

    println!("sync: --- prepare.sync.publish.HelloEvent ---");

    eventbus.publish_event(HelloEvent {
        message: String::from("Hello, Rust!"),
    });

    println!("sync: --- prepare.sync.publish.UpdateEvent ---");

    eventbus.publish_event(UpdateEvent {
        message: String::from("Hello, Rust!"),
    });

    println!("sync: --- post.sync.publish.UpdateEvent ---");
}
