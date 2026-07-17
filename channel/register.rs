pub fn register_front_api(reg: &mut crate::FrontApiRegistry) {
    reg.register(crate::end_turn::OP, crate::end_turn::handle);
    reg.register(crate::add_gold::OP, crate::add_gold::handle);
    reg.register(crate::spawn_unit::OP, crate::spawn_unit::handle);
    reg.register(crate::create_city::OP, crate::create_city::handle);
    reg.register(crate::request_snapshot::OP, crate::request_snapshot::handle);
}
