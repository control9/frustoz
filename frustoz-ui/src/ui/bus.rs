use std::cell::RefCell;
use std::rc::Rc;

use frustoz_core::model::flame::Flame;

use crate::ui::preview::Preview;

use super::widgets::Widgets;

pub struct BusImpl {
    pub subscribers: Vec<Box< dyn Subscriber>>
}

#[derive(Debug)]
pub enum Event<'a> {
    Open(),

    PreBind(&'a Flame),
    Bind(&'a Flame),
    PostBind(),

    Edit(FlameUpdate),

    SuggestRender(),
    DoRender(&'a Flame),
    Redraw(Vec<u8>),
}

#[derive(Debug)]
pub enum FlameUpdate {
    ScaleX(f64),
    ScaleY(f64),
    OriginX(f64),
    OriginY(f64),
}

pub type Bus = Rc<RefCell<BusImpl>>;

pub fn process(bus: &Bus, event: Event) {
    match &event {
        Event::Redraw(raw) => println!("Processing Redraw with size {}", raw.len() ),
        e => println!("Processing {:?}", e)
    }
    let mut itself = (bus).borrow_mut();

    itself.subscribers.iter_mut().filter(|s| s.accepts(&event))
        .for_each(|s| s.process(&event));
}

pub fn subscribe(bus: &Bus, sub: Box<dyn Subscriber>) {
    bus.borrow_mut().subscribers.push(sub);
}

pub trait Subscriber  {
    fn accepts(&self, e: &Event) -> bool;
    fn process(&mut self, e: &Event);
}

pub fn new() -> Bus {
    let bus_impl = BusImpl {
        subscribers: vec![],
    };
    Rc::new(RefCell::new(bus_impl))
}