cd src
mv lzyLexer.g4 parserExpressions.g4 parserStatements.g4 lzy.g4 main.cpp ..
rm -rf *
mv  ../lzyLexer.g4 ../parserExpressions.g4 ../parserStatements.g4 ../lzy.g4 ../main.cpp .
java -jar ../antlr4-runtime/antlr-complete.jar -Dlanguage=Cpp lzyLexer.g4 lzy.g4
cd ..
