#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait TestScContract {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn test_1(&self) {
        let res = mx_plugin_demo::test1(7, 8);
        let res_2 = mx_plugin_demo::test2(12);

        mx_plugin_demo::test3();

        sc_panic!("test_1 result: {} test_2 result: {}", res, res_2);
    }
}
