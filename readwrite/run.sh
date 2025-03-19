gcc readwrite.s -o readwrite
./readwrite
echo "Exit code: $?"
echo "Input: $(cat input1.txt) & $(cat input2.txt)"
echo "Output: $(cat output.txt)"
