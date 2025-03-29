cd src
mv lzy.g4 lzyLexer.g4 parserExpressions.g4 main.cpp ..
rm -rf *
mv ../lzy.g4 ../lzyLexer.g4 ../parserExpressions.g4 ../main.cpp .
java -jar ../antlr4-runtime/antlr-complete.jar -Dlanguage=Cpp lzyLexer.g4 lzy.g4 parserExpressions.g4
cd ..
