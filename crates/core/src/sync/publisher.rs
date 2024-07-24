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

// publisher

// ----------------------------------------------------------------

use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex};

use dashmap::DashMap;

use crate::sync::prelude::*;

// ----------------------------------------------------------------

type DynamicListener = dyn Any + Send + Sync;

// ----------------------------------------------------------------

pub trait ApplicationEventPublisher {
    fn register_listener<T: ApplicationEvent + 'static, L: ApplicationEventListener<T> + 'static>(
        &mut self,
        listener: L,
    );

    fn publish_event<T: ApplicationEvent + 'static>(&self, event: T);
}

// ----------------------------------------------------------------

struct WrappedApplicationEventListener<T: ApplicationEvent> {
    listener: Arc<dyn ApplicationEventListener<T>>,
}

impl<T: ApplicationEvent> WrappedApplicationEventListener<T> {
    fn new(listener: Arc<dyn ApplicationEventListener<T>>) -> Self {
        Self { listener }
    }
}

// ----------------------------------------------------------------

pub struct BuiltinSyncApplicationEventPublisher {
    listeners: Arc<Mutex<DashMap<TypeId, Vec<Box<DynamicListener>>>>>,
}

impl BuiltinSyncApplicationEventPublisher {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(DashMap::new())),
        }
    }
}

impl ApplicationEventPublisher for BuiltinSyncApplicationEventPublisher {
    fn register_listener<
        T: ApplicationEvent + 'static,
        L: ApplicationEventListener<T> + 'static,
    >(
        &mut self,
        listener: L,
    ) {
        self.listeners
            .lock()
            .unwrap()
            .entry(TypeId::of::<T>())
            .or_insert_with(Vec::new)
            .push(Box::new(WrappedApplicationEventListener::new(Arc::new(
                listener,
            ))));
    }

    fn publish_event<T: ApplicationEvent + 'static>(&self, event: T) {
        if let Some(listeners) = self
            .listeners
            .lock()
            .unwrap()
            .get(&TypeId::of::<T>())
            .as_deref()
        {
            for listener in listeners {
                if let Some(wrapper) = listener.downcast_ref::<WrappedApplicationEventListener<T>>()
                {
                    wrapper.listener.on_application_event(&event);
                }
            }
        }
    }
}
