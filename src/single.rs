use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

/// Returns the recv portion of the mpsc::channel
/// Spawns worker function to call
pub fn producer<T: Send + 'static> (generator: fn (Sender<T>)) -> Receiver<T> {
    let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();

    thread::spawn(  move || {
        generator(
            tx
        );
    });

    rx
}


#[cfg(test)]
mod test {

    use crate::single::producer;
    use std::sync::mpsc::{Sender};
    use std::io;
    use std::io::Write;

    fn fib_generator(send_channel: Sender<i128>) {
        let mut fib = (1, 1);

        send_channel.send(fib.0);
        send_channel.send(fib.1);

        loop {
            fib = (fib.1, fib.0 + fib.1);

            send_channel.send(fib.1);
        }
    }

    #[test]
    fn basics() {
        let out = producer(fib_generator);

        let mut first: i128 = 1;
        let mut second: i128 = 1;
        let mut temp;

        assert_eq!(out.recv().unwrap(), first); // 1
        assert_eq!(out.recv().unwrap(), second); // 1

        for i in 1..20 {
            temp = second;
            second = first + second;
            first = temp;

            assert_eq!(second, out.recv().unwrap());

        }

    }
}