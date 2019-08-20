use futures::{stream::Stream, Future};
use std::thread;
use tokio_udev::{Context, Event, EventType, MonitorBuilder};

macro_rules! ok_or_return {
    ($expression:expr) => {
        match $expression {
            Ok(value) => value,
            Err(_) => return,
        }
    };
}

/// Convenience function for an event loop which reacts to USB hotplug events.
pub fn usb_hotplug_event_loop<F: Fn() + Send + 'static>(func: F) {
    thread::spawn(move || {
        let context = ok_or_return!(Context::new());
        let mut builder = ok_or_return!(MonitorBuilder::new(&context));
        ok_or_return!(builder.match_subsystem_devtype("usb", "usb_device"));
        let monitor = ok_or_return!(builder.listen());

        let handler = move |e: Event| {
            match e.event_type() {
                EventType::Add | EventType::Remove => func(),
                _ => (),
            }
            Ok(())
        };

        tokio::run(monitor.for_each(handler).map_err(|_| ()));
    });
}
