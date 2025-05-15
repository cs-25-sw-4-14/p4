i64 main() {
   
   Vec<i64> a = {1,2,3,4,5};
   

   debug(a.get(0));
   debug(a.get(1));
   a.remove(1);

   i64 b = 1;
   i64 c = 1;
   i64 d = b - c;

   debug(a.get(0));
   debug(a.get(1));
   debug(a.get(2));


   

   return 2; 
}
