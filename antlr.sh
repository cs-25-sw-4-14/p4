cd src
mv lzy.g4 main.cpp ..
rm -rf *
mv ../lzy.g4 ../main.cpp .
antlr4 -Dlanguage=Cpp lzy.g4
cd ..
