use futures::Future;
use std::pin::Pin;
use crate::glib::MainContext;
use std::sync::Arc;

pub enum Command<T> {
    None,
    Defer(Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>>),
    DeferLocal(Pin<Box<dyn Future<Output = T> + 'static>>),
}

pub struct ComponentContainer<W: Component> {
    component: Box<W>,
    tx: Arc<dyn Fn(W::Msg) + Send + Sync>,
}

impl<W: Component> ComponentContainer<W> {
    pub fn new<T: MsgHandler<W::Msg> + Clone + Send>(sender: T, input: Option<W::Input>) -> ComponentContainer<W> {
        let component = W::create(sender.clone(), input);
        Self {
            component: Box::new(component),
            tx: Arc::new(sender.clone())
        }
    }

    pub fn get_mut(&mut self) -> &mut Box<W> {
        &mut self.component
    }

    pub fn get(&self) -> &Box<W> {
        &self.component
    }

    pub fn update(&mut self, msg: W::Msg) {
        let res = self.component.update(msg);
        match res {
            Command::Defer(f) => {
                let tx = self.tx.clone();
                tokio::task::spawn(async move {
                    let r = f.await;
                    tx(r)
                });
            }
            Command::DeferLocal(f) => {
                let tx = self.tx.clone();
                MainContext::ref_thread_default().spawn_local(async move {
                    let r = f.await;
                    tx(r)
                });
            }
            _ => ()
        }
    }

    pub fn view(&self) -> &W::View {
        self.component.view()
    }
}

pub trait MsgHandler<T>: 'static + Send + Sync + Fn(T) {}
impl<A, T> MsgHandler<T> for A where A: 'static + Send + Sync + Fn(T) {}

pub trait Component: Sized + 'static {
    type Msg: Clone;
    type View;
    type Input;
    fn create<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Option<Self::Input>) -> Self;
    fn new<T: MsgHandler<Self::Msg> + Clone>(sender: T) -> ComponentContainer<Self> {
        ComponentContainer::<Self>::new(sender, None)
    }

    fn new_with_data<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Self::Input) -> ComponentContainer<Self> {
        ComponentContainer::<Self>::new(sender, Some(input))
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;
    fn view(&self) -> &Self::View;
    fn run_async<T: Future<Output = Self::Msg> + 'static + Send + Sync>(&self, t: T) -> Command<Self::Msg> {
        Command::Defer(Box::pin(t))
    }
    fn run_async_local<T: Future<Output = Self::Msg> + 'static>(&self, t: T) -> Command<Self::Msg> {
        Command::DeferLocal(Box::pin(t))
    }
}