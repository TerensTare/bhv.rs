mod adapt;
mod bhv;
mod bhv_ext;

#[macro_use]
mod composite;
mod decor;

pub use adapt::*;
pub use bhv::*;
pub use composite::*;
pub use decor::*;

pub use bhv_ext::BhvExt as _;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adaptors_test() {
        struct Ctx(i32, String);

        let cond = cond(|v: &Ctx| v.0 == 42);
        let parse = action(|v: &mut Ctx| v.1 = format!("{}", v.0));
        let print = action(|v: &mut Ctx| println!("value: {}", v.1));

        let mut ctx = Ctx(42, String::new());
        let tree = seq! { cond, parse, print };

        assert!(tree.execute(&mut ctx) == true);
        assert_eq!(ctx.1, "42");
    }

    #[test]
    fn decorators_test() {
        let noop = |_: &mut i32| {};

        let ok = Pass(action(noop));
        assert!(ok.clone().execute(&mut 0) == true);
        assert!(ok.inv().execute(&mut 0) == false);

        let fail = Fail(action(noop));
        assert!(fail.clone().execute(&mut 0) == false);
        assert!(fail.inv().execute(&mut 0) == true);

        let print = action(|v: &mut i32| println!("Value is {}", *v));

        let repeat = print.clone().repeat(3);
        assert!(repeat.execute(&mut 10) == true);

        let decrease_ctx = action(|v: &mut i32| *v -= 1);
        let seq = seq! {print,decrease_ctx};
        let cond_repeat = seq.repeat_until(|v| *v == 0);

        assert!(cond_repeat.execute(&mut 2) == true);
    }

    #[test]
    fn composites_test() {
        let tree = seq! {
            action(|v| println!("Value is {}", *v)),
            action(|v| *v -=1),
            action(|v| println!("Value now is {}", *v)),
        };

        assert!(tree.execute(&mut 0) == true);

        let v_less_than_5 = seq! {
            cond(|v| (0..5).contains(v)),
            action(|_v| println!("v is less than 5")),
        };

        let v_in_5_25 = seq! {
            cond(|v| (5..25).contains(v)),
            action(|_v| println!("v is between 5 and 25")),
        };

        let v_greater = action(|_v| println!("v is greater than 24"));

        let tree = sel! {
            v_less_than_5,
            v_in_5_25,
            v_greater,
        };

        assert!(tree.execute(&mut 25) == true);
    }
}
