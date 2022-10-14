use yew::prelude::*;

pub enum Msg {
    AddOne
}

pub struct CounterComponent {
    count: i64
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            count: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link().clone();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button type="button" class="btn btn-primary" onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

