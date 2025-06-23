i64 char_to_i64(char c) {
    return (c - '0' as char) as i64;
}

void main() {
    String s = read_file("in.txt");
    Vec<char> v = s as Vec<char>;

    Vec<i64> l = {};
    Vec<i64> r = {};

    i64 i = 0;
    i64 cnt = 0;
    while(true) {
        if(v.get(i) == 0 as char) {
            break;
        }
        i64 nr = char_to_i64(v.get(i)) * 10000;
        nr = nr + char_to_i64(v.get(i + 1)) * 1000;
        nr = nr + char_to_i64(v.get(i + 2)) * 100;
        nr = nr + char_to_i64(v.get(i + 3)) * 10;
        nr = nr + char_to_i64(v.get(i + 4));
        l.insert(0, nr);

        i = i + 8;

        nr = char_to_i64(v.get(i)) * 10000;
        nr = nr + char_to_i64(v.get(i + 1)) * 1000;
        nr = nr + char_to_i64(v.get(i + 2)) * 100;
        nr = nr + char_to_i64(v.get(i + 3)) * 10;
        nr = nr + char_to_i64(v.get(i + 4));
        r.insert(0, nr);

        i = i + 6;

        cnt = cnt + 1;
    }

    i = 0;
    i64 j = 0;
    i64 answ1 = 0;
    i64 answ2 = 0;
    while(i < cnt) {
        i64 tmp = 0;
        j = 0;
        while(j < cnt) {
            if(l.get(i) == r.get(j)) {
                tmp = tmp + 1;
            }
            j = j + 1;
        }
        answ2 = answ2 + l.get(i) * tmp;
        i = i + 1;
    }

    Vec<i64> sorted_l = {};

    i = 0;
    while(i < cnt) {
        i64 highest_l = 0;
        i64 highest_l_idx = 0;

        j = 0;
        while(j < (cnt-i)) {
            if(l.get(j) > highest_l) {
                highest_l = l.get(j);
                highest_l_idx = j;
            }
            j = j + 1;
        }
        i = i + 1;
        sorted_l.insert(0, highest_l);
        l.remove(highest_l_idx);
    }

    Vec<i64> sorted_r = {};

    i = 0;
    while(i < cnt) {
        i64 highest_r = 0;
        i64 highest_r_idx = 0;

        j = 0;
        while(j < (cnt-i)) {
            if(r.get(j) > highest_r) {
                highest_r = r.get(j);
                highest_r_idx = j;
            }
            j = j + 1;
        }
        i = i + 1;
        sorted_r.insert(0, highest_r);
        r.remove(highest_r_idx);
    }

    i = 0;
    while(i < cnt) {
        i64 x = sorted_l.get(i);
        i64 y = sorted_r.get(i);
        if(x > y) {
            answ1 = answ1 + x - y;
                    } else {
            answ1 = answ1 + y - x;
        }
        i = i + 1;
    }

    debug("Part one: ");
    debug(answ1);
    debug("\n");

    debug("Part two: ");
    debug(answ2);
    debug("\n");
}
