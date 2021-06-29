// TODO: we need to transform the AST into a general version, due to the changes we perform
// 1) Parentheses get removed around some expressions - we should remove simple parentheses
// 2) We remove unnecessary semicolons
// 3) We change the formatting of number literals -> we should run str::parse(number).to_string() to normalise them
// 4) We change semicolons to commas in punctuated lists
// 5) We change string quotes
