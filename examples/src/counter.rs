use gtk4_helper::{
    prelude::*,
    gtk4,
};

#[derive(Debug, Clone)]
pub enum CounterWidgetMsg {
    Inc,
    AsyncInc,
    Dec,
}

pub struct CounterWidget {
    counter: i32,
    lbl: gtk4::Label,
}

impl CustomWidget for CounterWidget {
    type Msg = CounterWidgetMsg;
    type Output = ();
    type Input = i32;

    fn init(data: Self::Input) -> Self {
        let lbl = gtk4::Label::new(Some(&format!("Count: {}", data)));
        Self {
            counter: data,
            lbl
        }
    }

    fn create(&self, container: &gtk4::Box, action_sender: Sender<Self::Msg>) {
        let btn = gtk4::ButtonBuilder::new()
            .label("Dec")
            .build();

        let tx = action_sender.clone();
        btn.connect_clicked(move |_| {
            tx.send(CounterWidgetMsg::Dec).expect("Could not send dec");
        });

        container.append(&btn);
        container.append(&self.lbl);

        let btn = gtk4::ButtonBuilder::new()
            .label("Inc")
            .build();


        let tx = action_sender.clone();
        btn.connect_clicked(move |_| {
            tx.send(CounterWidgetMsg::AsyncInc).expect("Could not send inc");
        });
        container.append(&btn);
    }

    fn update(&mut self, msg: CounterWidgetMsg) -> WidgetMsg<Self> {
        match msg {
            CounterWidgetMsg::Inc => {
                self.update_counter(true)
            }
            CounterWidgetMsg::Dec => {
                self.update_counter(false)
            }
            CounterWidgetMsg::AsyncInc => {
                self.run_async(inc_async())
            }
        }
    }
}

impl CounterWidget {
    fn update_counter(&mut self, inc: bool) -> WidgetMsg<Self> {
        self.counter = if inc { self.counter + 1 } else { self.counter -1 };
        self.lbl.set_text(&format!("Count: {}", self.counter));
        self.msg_none()
    }
}

async fn inc_async() -> CounterWidgetMsg {
    tokio::time::delay_for(std::time::Duration::from_millis(100)).await;
    CounterWidgetMsg::Inc
}