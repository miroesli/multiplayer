pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Job = Box<FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub fn new_job<F>(f: F) -> Message
    where
        F: FnOnce() + Send + 'static
{
    let job = Box::new(f);

    Message::NewJob(job)
}