fn main() {
    cc::Build::new().file("ffi/va_list.c").compile("va_list");
}
