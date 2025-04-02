#include <iostream>
#include <fstream>
#include <antlr4-runtime.h>
#include "lzyLexer.h"
#include "lzyParser.h"
#include "huttel.h"

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
    
    try {
        antlr4::ANTLRInputStream input(stream);
        lzyLexer lexer(&input);
        antlr4::CommonTokenStream tokens(&lexer);
        lzyParser parser(&tokens);
        
        tree::ParseTree* tree = parser.program();
        
        if (parser.getNumberOfSyntaxErrors() > 0) {
            std::cerr << HUTTEL 
                      << "Parsing failed with " 
                      << parser.getNumberOfSyntaxErrors() 
                      << " syntax errors" << std::endl;
            return 1;
        }
        
        std::cout << "Parse tree: " << tree->toStringTree(&parser) << std::endl;
    }
    catch (const std::exception& e) {
        std::cerr << "Error during parsing: " << e.what() << std::endl;
        return 1;
    }
    
    return 0;
}
