cd src
mv lzy.g4 main.cpp ..
rm -rf *
mv ../lzy.g4 ../main.cpp .
java -jar ../antlr4-runtime/antlr-complete.jar -Dlanguage=Cpp lzy.g4
cd ..
