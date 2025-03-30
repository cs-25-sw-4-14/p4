cat src/lzy.g4 | grep -Ev "options|import" > src/combined.g4
tail -n +4 src/parserStatements.g4 >> src/combined.g4
tail -n +4 src/parserExpressions.g4 >> src/combined.g4
tail -n +2 src/lzyLexer.g4 >> src/combined.g4