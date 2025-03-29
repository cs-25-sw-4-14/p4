cd src
mv lzy.g4 lzyLex.g4 main.cpp ..
rm -rf *
mv ../lzy.g4 ../lzyLex.g4 ../main.cpp .
java -jar ../antlr4-runtime/antlr-complete.jar -Dlanguage=Cpp lzyLex.g4 lzy.g4
cd ..
