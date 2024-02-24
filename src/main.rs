fn main() {
    test_impl::run()
}

#[cfg(feature = "events")]
mod test_impl {
    use bhv::*;

    struct Tick(u32);

    struct Exit;

    impl EventType for Tick {}

    impl EventType for Exit {}

    pub fn run() {
        let tree = seq! {
            action(|i| println!("i: {}", i)),
            action(|i| *i += 1),
            action(|_| println!("Exiting...")).wait_for::<Exit>()
        };

        let event_queue: [&dyn Event; 6] = [
            &Tick(0),
            &Tick(1),
            &Tick(2),
            &Tick(3),
            &Tick(4),
            &Exit,
        ];

        let mut ctx = 0;

        tree.execute(event_queue, &mut ctx);

        assert_eq!(ctx, 6);
    }
}

#[cfg(not(feature = "events"))]
mod test_impl {
    use bhv::*;

    pub fn run() {
        let tree = sel! {
            seq! {
                action(|i| println!("i: {}", i)),
                action(|i| *i += 1),
            }.repeat_until(|i| *i == 5),
            action(|_| println!("Exiting...")),
        };

        let mut ctx = 0;
        tree.execute(&mut ctx);

        assert_eq!(ctx, 5);
    }
}