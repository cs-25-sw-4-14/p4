void double_vec(&Vec<i64> v, i64 l) {
    i64 i = 0;
    while(i < l) {
        Vec<i64> v2 = *v;
         v2.insert(i, v2.get(i) * 2);
         v2.remove(i+1);
    }
}
    

void main() {
    Vec<i64> v = {1,2,3};
    double_vec(&v, 3);
}
