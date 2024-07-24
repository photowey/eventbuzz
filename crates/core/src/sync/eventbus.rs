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

// eventbus

// ----------------------------------------------------------------

use crate::event::ApplicationEvent;
use crate::sync::listener::ApplicationEventListener;
use crate::sync::publisher::{ApplicationEventPublisher, BuiltinSyncApplicationEventPublisher};

// ----------------------------------------------------------------

pub struct Eventbus {
    publisher: BuiltinSyncApplicationEventPublisher,
}

impl Eventbus {
    pub fn builder() -> EventbusBuilder {
        EventbusBuilder::new()
    }

    fn new() -> Self {
        Eventbus {
            publisher: BuiltinSyncApplicationEventPublisher::new(),
        }
    }
}

// ----------------------------------------------------------------

impl Eventbus {
    pub fn register_listener<T, L>(&mut self, listener: L)
    where
        T: ApplicationEvent + 'static,
        L: ApplicationEventListener<T> + 'static,
    {
        self.publisher.register_listener(listener);
    }

    pub fn publish_event<T>(&self, event: T)
    where
        T: ApplicationEvent + 'static,
    {
        self.publisher.publish_event(event);
    }
}

// ----------------------------------------------------------------

pub struct EventbusBuilder {}

impl EventbusBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> Eventbus {
        return Eventbus::new();
    }
}
