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
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::RwLock;

use crate::asynchronous::listener::AsyncApplicationEventListener;
use crate::event::ApplicationEvent;

// ----------------------------------------------------------------

type AsyncDynamicListener = dyn Any + Send + Sync;

// ----------------------------------------------------------------

#[async_trait]
pub trait AsyncApplicationEventPublisher {
    async fn register_listener<
        T: ApplicationEvent + 'static,
        L: AsyncApplicationEventListener<T> + 'static,
    >(
        &mut self,
        listener: L,
    );

    async fn publish_event<T: ApplicationEvent + 'static>(&self, event: T);
}

// ----------------------------------------------------------------

struct WrappedAsyncApplicationEventListener<T: ApplicationEvent> {
    listener: Arc<dyn AsyncApplicationEventListener<T>>,
}

impl<T: ApplicationEvent> WrappedAsyncApplicationEventListener<T> {
    fn new(listener: Arc<dyn AsyncApplicationEventListener<T>>) -> Self {
        Self { listener }
    }
}

// ----------------------------------------------------------------

pub struct BuiltinAsyncApplicationEventPublisher {
    listeners: Arc<RwLock<DashMap<TypeId, Vec<Box<AsyncDynamicListener>>>>>,
}

impl BuiltinAsyncApplicationEventPublisher {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(DashMap::new())),
        }
    }
}

#[async_trait]
impl AsyncApplicationEventPublisher for BuiltinAsyncApplicationEventPublisher {
    async fn register_listener<
        T: ApplicationEvent + 'static,
        L: AsyncApplicationEventListener<T> + 'static,
    >(
        &mut self,
        listener: L,
    ) {
        self.listeners
            .write()
            .await
            .entry(TypeId::of::<T>())
            .or_insert_with(Vec::new)
            .push(Box::new(WrappedAsyncApplicationEventListener::new(
                Arc::new(listener),
            )));
    }

    async fn publish_event<T: ApplicationEvent + 'static>(&self, event: T) {
        if let Some(listeners) = self
            .listeners
            .read()
            .await
            .get(&TypeId::of::<T>())
            .as_deref()
        {
            for listener in listeners {
                if let Some(wrapper) =
                    listener.downcast_ref::<WrappedAsyncApplicationEventListener<T>>()
                {
                    wrapper.listener.on_application_event(&event).await;
                }
            }
        }
    }
}
