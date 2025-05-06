void main() {
    String path = "readthis.ss";
    String content = read_file(path);
    debug(content);
    _ = write_file("Hey, this file will have some new text", path);
    String content2 = read_file(path);
    debug(content2);
}