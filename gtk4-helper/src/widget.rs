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
    pub fn new<T: MsgHandler<W::Msg> + Clone>(sender: T, input: Option<W::Input>) -> WidgetContainer<W> {
        let mut widget = W::create(sender.clone(), input);
        widget.init(sender.clone());
        Self {
            widget: Box::new(widget),
            tx: Rc::new(sender.clone())
        }
    }

    pub fn get_mut(&mut self) -> &mut Box<W> {
        &mut self.widget
    }

    pub fn get(&self) -> &Box<W> {
        &self.widget
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

    pub fn view(&self) -> &W::View {
        self.widget.view()
    }
}

pub trait MsgHandler<T>: 'static + Send + Sync + Fn(T) {}
impl<A, T> MsgHandler<T> for A where A: 'static + Send + Sync + Fn(T) {}

pub trait Widget: Sized + 'static {
    type Msg: Clone;
    type View;
    type Input;
    fn create<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Option<Self::Input>) -> Self;
    fn new<T: MsgHandler<Self::Msg> + Clone>(sender: T) -> WidgetContainer<Self> {
        WidgetContainer::<Self>::new(sender, None)
    }

    fn new_with_data<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Self::Input) -> WidgetContainer<Self> {
        WidgetContainer::<Self>::new(sender, Some(input))
    }

    fn init<T: MsgHandler<Self::Msg> + Clone>(&mut self, _: T) {
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;
    fn view(&self) -> &Self::View;
    fn run_async<T: Future<Output = Self::Msg> + 'static>(&self, t: T) -> Command<Self::Msg> {
        Command::Defer(Box::pin(t))
    }
}