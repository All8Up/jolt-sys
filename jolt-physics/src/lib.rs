#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe { jolt_sys::register_types() };
    }
}
