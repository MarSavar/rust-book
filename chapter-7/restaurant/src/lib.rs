mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn serve_order() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::hosting::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}