use futures::Future;
use std::pin::Pin;
use crate::glib::MainContext;
use std::rc::Rc;

pub enum Command<T> {
    None,
    Defer(Pin<Box<dyn Future<Output = T> + 'static>>),
}

pub struct WidgetContainer<W: Widget> {
    widget: Box<W>,
    tx: Rc<dyn Fn(W::Msg)>,
}

impl<W: Widget> WidgetContainer<W> {
    pub fn new<T: 'static + Clone + Fn(W::Msg)>(sender: T) -> WidgetContainer<W> {
        Self {
            widget: Box::new(W::create(sender.clone())),
            tx: Rc::new(sender.clone())
        }
    }

    pub fn update(&mut self, msg: W::Msg) {
        let res = self.widget.update(msg);
        match res {
            Command::Defer(f) => {
                let tx = self.tx.clone();
                MainContext::ref_thread_default().spawn_local(async move {
                    tx(f.await);
                });
            }
            _ => ()
        }
    }

    pub fn view(&self) -> &gtk4::Box {
        self.widget.view()
    }
}

pub trait Widget: Sized + 'static {
    type Msg: Clone;
    fn create<T: 'static + Clone + Fn(Self::Msg)>(sender: T) -> Self;
    fn new<T: 'static + Clone + Fn(Self::Msg)>(sender: T) -> WidgetContainer<Self> {
        WidgetContainer::<Self>::new(sender)
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;
    fn view(&self) -> &gtk4::Box;
    fn run_async<T: Future<Output = Self::Msg> + 'static>(&self, t: T) -> Command<Self::Msg> {
        Command::Defer(Box::pin(t))
    }
}