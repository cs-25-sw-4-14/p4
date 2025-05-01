i64 main() {
    String a = "abc";

    borrow_2(a);
    borrow_1(&a);
    borrow_1(&a);

    return 0;
}

void borrow_1(&String in) {

}

void borrow_2(String in) {

}
