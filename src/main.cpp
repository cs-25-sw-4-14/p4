#include <iostream>
#include <fstream>
#include "lzyLexer.h"
#include "lzyParser.h"

using namespace antlr4;

int main(int argc, const char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input file>" << std::endl;
        return 1;
    }
    
    std::ifstream stream(argv[1]);
    if (!stream) {
        std::cerr << "Error opening file: " << argv[1] << std::endl;
        return 1;
    }
    
    ANTLRInputStream input(stream);
    lzyLexer lexer(&input);
    CommonTokenStream tokens(&lexer);
    lzyParser parser(&tokens);
    
    tree::ParseTree* tree = parser.expr();
    
    std::cout << "Parse tree: " << tree->toStringTree(&parser) << std::endl;
    return 0;
}
