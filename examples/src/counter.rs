use gtk4_helper::{
    prelude::*,
    gtk,
};

#[derive(Clone)]
pub enum CounterMsg {
    Inc,
    IncAsync,
    Dec
}

pub struct SimpleCounter {
    lbl: gtk::Label,
    container: gtk::Box,
    count: i32,
}

impl Component for SimpleCounter {
    type Msg = CounterMsg;
    type View = gtk::Box;
    type Input = i32;

    fn create<T: MsgHandler<Self::Msg> + Clone>(sender: T, input: Option<Self::Input>) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let start = input.unwrap_or(0);
        let lbl = gtk::Label::new(Some(&format!("Count: {}", start)));
        let btn = gtk::builders::ButtonBuilder::new()
            .label("Dec")
            .build();

        let tx = sender.clone();
        btn.connect_clicked(move |_| {
            tx(CounterMsg::Dec);
        });

        container.append(&btn);
        container.append(&lbl);

        let btn = gtk::builders::ButtonBuilder::new()
            .label("Inc")
            .build();

        let tx = sender.clone();
        btn.connect_clicked(move |_| {
            tx(CounterMsg::IncAsync);
        });
        container.append(&btn);

        Self {
            lbl,
            count: start,
            container,
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            CounterMsg::Inc => {
                self.count += 1;
                self.lbl.set_text(&format!("Count: {}", self.count));
            }
            CounterMsg::Dec => {
                self.count -= 1;
                self.lbl.set_text(&format!("Count: {}", self.count));
            }
            CounterMsg::IncAsync => {
                return self.run_async(inc_async());
            }
        }
        Command::None
    }

    fn view(&self) -> &Self::View {
        &self.container
    }
}

async fn inc_async() -> CounterMsg {
    CounterMsg::Inc
}