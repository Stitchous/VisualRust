inner_attrs
: inner_attr               { $$ = mk_node("InnerAttrs", 1, $1); }
| inner_attrs inner_attr   { $$ = ext_node($1, 1, $2); }
;



outer_attrs
: outer_attr               { $$ = mk_node("OuterAttrs", 1, $1); }
| outer_attrs outer_attr   { $$ = ext_node($1, 1, $2); }
;



meta_seq
: %empty                   { $$ = mk_none(); }
| meta_item                { $$ = mk_node("MetaItems", 1, $1); }
| meta_seq ',' meta_item   { $$ = ext_node($1, 1, $3); }
;



mod_items
: mod_item                               { $$ = mk_node("Items", 1, $1); }
| mod_items mod_item                     { $$ = ext_node($1, 1, $2); }
;



struct_decl_fields
: struct_decl_field                           { $$ = mk_node("StructFields", 1, $1); }
| struct_decl_fields ',' struct_decl_field    { $$ = ext_node($1, 1, $3); }
| %empty                                      { $$ = mk_none(); }
;



struct_tuple_fields
: struct_tuple_field                          { $$ = mk_node("StructFields", 1, $1); }
| struct_tuple_fields ',' struct_tuple_field  { $$ = ext_node($1, 1, $3); }
;



enum_defs
: enum_def               { $$ = mk_node("EnumDefs", 1, $1); }
| enum_defs ',' enum_def { $$ = ext_node($1, 1, $3); }
| %empty                 { $$ = mk_none(); }
;



foreign_items
: foreign_item               { $$ = mk_node("ForeignItems", 1, $1); }
| foreign_items foreign_item { $$ = ext_node($1, 1, $2); }
;



idents_or_self
: ident_or_self                    { $$ = mk_node("IdentsOrSelf", 1, $1); }
| ident_or_self AS ident           { $$ = mk_node("IdentsOrSelf", 2, $1, $3); }
| idents_or_self ',' ident_or_self { $$ = ext_node($1, 1, $3); }
;



trait_items
: trait_item               { $$ = mk_node("TraitItems", 1, $1); }
| trait_items trait_item   { $$ = ext_node($1, 1, $2); }
;



params
: param                { $$ = mk_node("Args", 1, $1); }
| params ',' param     { $$ = ext_node($1, 1, $3); }
;



inferrable_params
: inferrable_param                       { $$ = mk_node("InferrableParams", 1, $1); }
| inferrable_params ',' inferrable_param { $$ = ext_node($1, 1, $3); }
;



anon_params
: anon_param                 { $$ = mk_node("Args", 1, $1); }
| anon_params ',' anon_param { $$ = ext_node($1, 1, $3); }
;

// anon means it's allowed to be anonymous (type-only), but it can
// still have a name


where_predicates
: where_predicate                      { $$ = mk_node("WherePredicates", 1, $1); }
| where_predicates ',' where_predicate { $$ = ext_node($1, 1, $3); }
;



ty_params
: ty_param               { $$ = mk_node("TyParams", 1, $1); }
| ty_params ',' ty_param { $$ = ext_node($1, 1, $3); }
;

// A path with no type parameters; e.g. `foo::bar::Baz`
//
// These show up in 'use' view-items, because these are processed
// without respect to types.


path_no_types_allowed
: ident                               { $$ = mk_node("ViewPath", 1, $1); }
| MOD_SEP ident                       { $$ = mk_node("ViewPath", 1, $2); }
| SELF                                { $$ = mk_node("ViewPath", 1, mk_atom("Self")); }
| MOD_SEP SELF                        { $$ = mk_node("ViewPath", 1, mk_atom("Self")); }
| path_no_types_allowed MOD_SEP ident { $$ = ext_node($1, 1, $3); }
;

// A path with a lifetime and type parameters, with no double colons
// before the type parameters; e.g. `foo::bar<'a>::Baz<T>`
//
// These show up in "trait references", the components of
// type-parameter bounds lists, as well as in the prefix of the
// path_generic_args_and_bounds rule, which is the full form of a
// named typed expression.
//
// They do not have (nor need) an extra '::' before '<' because
// unlike in expr context, there are no "less-than" type exprs to
// be ambiguous with.


path_generic_args_without_colons
: %prec IDENT
  ident                                                                       { $$ = mk_node("components", 1, $1); }
| %prec IDENT
  ident generic_args                                                          { $$ = mk_node("components", 2, $1, $2); }
| %prec IDENT
  ident '(' maybe_ty_sums ')' ret_ty                                          { $$ = mk_node("components", 2, $1, $3); }
| %prec IDENT
  path_generic_args_without_colons MOD_SEP ident                              { $$ = ext_node($1, 1, $3); }
| %prec IDENT
  path_generic_args_without_colons MOD_SEP ident generic_args                 { $$ = ext_node($1, 2, $3, $4); }
| %prec IDENT
  path_generic_args_without_colons MOD_SEP ident '(' maybe_ty_sums ')' ret_ty { $$ = ext_node($1, 2, $3, $5); }
;



pats_or
: pat              { $$ = mk_node("Pats", 1, $1); }
| pats_or '|' pat  { $$ = ext_node($1, 1, $3); }
;



pat_fields
: pat_field                  { $$ = mk_node("PatFields", 1, $1); }
| pat_fields ',' pat_field   { $$ = ext_node($1, 1, $3); }
;



pat_tup
: pat               { $$ = mk_node("pat_tup", 1, $1); }
| pat_tup ',' pat   { $$ = ext_node($1, 1, $3); }
;



pat_vec_elts
: pat                    { $$ = mk_node("PatVecElts", 1, $1); }
| pat_vec_elts ',' pat   { $$ = ext_node($1, 1, $3); }
;

////////////////////////////////////////////////////////////////////////
// Part 3: Types
////////////////////////////////////////////////////////////////////////



ty_sums
: ty_sum             { $$ = mk_node("TySums", 1, $1); }
| ty_sums ',' ty_sum { $$ = ext_node($1, 1, $3); }
;



boundseq
: polybound
| boundseq '+' polybound { $$ = ext_node($1, 1, $3); }
;



bindings
: binding              { $$ = mk_node("Bindings", 1, $1); }
| bindings ',' binding { $$ = ext_node($1, 1, $3); }
;



bounds
: bound            { $$ = mk_node("bounds", 1, $1); }
| bounds '+' bound { $$ = ext_node($1, 1, $3); }
;



ltbounds
: lifetime              { $$ = mk_node("ltbounds", 1, $1); }
| ltbounds '+' lifetime { $$ = ext_node($1, 1, $3); }
;



lifetimes
: lifetime_and_bounds               { $$ = mk_node("Lifetimes", 1, $1); }
| lifetimes ',' lifetime_and_bounds { $$ = ext_node($1, 1, $3); }
;



stmts
: stmt           { $$ = mk_node("stmts", 1, $1); }
| stmts stmt     { $$ = ext_node($1, 1, $2); }
;



exprs
: expr                                                        { $$ = mk_node("exprs", 1, $1); }
| exprs ',' expr                                              { $$ = ext_node($1, 1, $3); }
;



path_generic_args_with_colons
: ident                                              { $$ = mk_node("components", 1, $1); }
| path_generic_args_with_colons MOD_SEP ident        { $$ = ext_node($1, 1, $3); }
| path_generic_args_with_colons MOD_SEP generic_args { $$ = ext_node($1, 1, $3); }
;

// the braces-delimited macro is a block_expr so it doesn't appear here


nonblock_expr
: lit                                                           { $$ = mk_node("ExprLit", 1, $1); }
| %prec IDENT
  path_expr                                                     { $$ = mk_node("ExprPath", 1, $1); }
| SELF                                                          { $$ = mk_node("ExprPath", 1, mk_node("ident", 1, mk_atom("self"))); }
| macro_expr                                                    { $$ = mk_node("ExprMac", 1, $1); }
| path_expr '{' struct_expr_fields '}'                          { $$ = mk_node("ExprStruct", 2, $1, $3); }
| nonblock_expr '.' path_generic_args_with_colons               { $$ = mk_node("ExprField", 2, $1, $3); }
| nonblock_expr '.' LIT_INTEGER                                 { $$ = mk_node("ExprTupleIndex", 1, $1); }
| nonblock_expr '[' maybe_expr ']'                              { $$ = mk_node("ExprIndex", 2, $1, $3); }
| nonblock_expr '(' maybe_exprs ')'                             { $$ = mk_node("ExprCall", 2, $1, $3); }
| '[' vec_expr ']'                                              { $$ = mk_node("ExprVec", 1, $2); }
| '(' maybe_exprs ')'                                           { $$ = mk_node("ExprParen", 1, $2); }
| CONTINUE                                                      { $$ = mk_node("ExprAgain", 0); }
| CONTINUE lifetime                                             { $$ = mk_node("ExprAgain", 1, $2); }
| RETURN                                                        { $$ = mk_node("ExprRet", 0); }
| RETURN expr                                                   { $$ = mk_node("ExprRet", 1, $2); }
| BREAK                                                         { $$ = mk_node("ExprBreak", 0); }
| BREAK lifetime                                                { $$ = mk_node("ExprBreak", 1, $2); }
| nonblock_expr LARROW expr                                     { $$ = mk_node("ExprInPlace", 2, $1, $3); }
| nonblock_expr '=' expr                                        { $$ = mk_node("ExprAssign", 2, $1, $3); }
| nonblock_expr SHLEQ expr                                      { $$ = mk_node("ExprAssignShl", 2, $1, $3); }
| nonblock_expr SHREQ expr                                      { $$ = mk_node("ExprAssignShr", 2, $1, $3); }
| nonblock_expr MINUSEQ expr                                    { $$ = mk_node("ExprAssignSub", 2, $1, $3); }
| nonblock_expr ANDEQ expr                                      { $$ = mk_node("ExprAssignBitAnd", 2, $1, $3); }
| nonblock_expr OREQ expr                                       { $$ = mk_node("ExprAssignBitOr", 2, $1, $3); }
| nonblock_expr PLUSEQ expr                                     { $$ = mk_node("ExprAssignAdd", 2, $1, $3); }
| nonblock_expr STAREQ expr                                     { $$ = mk_node("ExprAssignMul", 2, $1, $3); }
| nonblock_expr SLASHEQ expr                                    { $$ = mk_node("ExprAssignDiv", 2, $1, $3); }
| nonblock_expr CARETEQ expr                                    { $$ = mk_node("ExprAssignBitXor", 2, $1, $3); }
| nonblock_expr PERCENTEQ expr                                  { $$ = mk_node("ExprAssignRem", 2, $1, $3); }
| nonblock_expr OROR expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiOr"), $1, $3); }
| nonblock_expr ANDAND expr                                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiAnd"), $1, $3); }
| nonblock_expr EQEQ expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiEq"), $1, $3); }
| nonblock_expr NE expr                                         { $$ = mk_node("ExprBinary", 3, mk_atom("BiNe"), $1, $3); }
| nonblock_expr '<' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiLt"), $1, $3); }
| nonblock_expr '>' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiGt"), $1, $3); }
| nonblock_expr LE expr                                         { $$ = mk_node("ExprBinary", 3, mk_atom("BiLe"), $1, $3); }
| nonblock_expr GE expr                                         { $$ = mk_node("ExprBinary", 3, mk_atom("BiGe"), $1, $3); }
| nonblock_expr '|' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitOr"), $1, $3); }
| nonblock_expr '^' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitXor"), $1, $3); }
| nonblock_expr '&' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitAnd"), $1, $3); }
| nonblock_expr SHL expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiShl"), $1, $3); }
| nonblock_expr SHR expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiShr"), $1, $3); }
| nonblock_expr '+' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiAdd"), $1, $3); }
| nonblock_expr '-' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiSub"), $1, $3); }
| nonblock_expr '*' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiMul"), $1, $3); }
| nonblock_expr '/' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiDiv"), $1, $3); }
| nonblock_expr '%' expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiRem"), $1, $3); }
| nonblock_expr DOTDOT                                          { $$ = mk_node("ExprRange", 2, $1, mk_none()); }
| nonblock_expr DOTDOT expr                                     { $$ = mk_node("ExprRange", 2, $1, $3); }
|               DOTDOT expr                                     { $$ = mk_node("ExprRange", 2, mk_none(), $2); }
|               DOTDOT                                          { $$ = mk_node("ExprRange", 2, mk_none(), mk_none()); }
| nonblock_expr AS ty                                           { $$ = mk_node("ExprCast", 2, $1, $3); }
| BOX nonparen_expr                                             { $$ = mk_node("ExprBox", 1, $2); }
| %prec BOXPLACE BOX '(' maybe_expr ')' nonblock_expr           { $$ = mk_node("ExprBox", 2, $3, $5); }
| expr_qualified_path
| nonblock_prefix_expr
;



expr
: lit                                                 { $$ = mk_node("ExprLit", 1, $1); }
| %prec IDENT
  path_expr                                           { $$ = mk_node("ExprPath", 1, $1); }
| SELF                                                { $$ = mk_node("ExprPath", 1, mk_node("ident", 1, mk_atom("self"))); }
| macro_expr                                          { $$ = mk_node("ExprMac", 1, $1); }
| path_expr '{' struct_expr_fields '}'                { $$ = mk_node("ExprStruct", 2, $1, $3); }
| expr '.' path_generic_args_with_colons              { $$ = mk_node("ExprField", 2, $1, $3); }
| expr '.' LIT_INTEGER                                { $$ = mk_node("ExprTupleIndex", 1, $1); }
| expr '[' maybe_expr ']'                             { $$ = mk_node("ExprIndex", 2, $1, $3); }
| expr '(' maybe_exprs ')'                            { $$ = mk_node("ExprCall", 2, $1, $3); }
| '(' maybe_exprs ')'                                 { $$ = mk_node("ExprParen", 1, $2); }
| '[' vec_expr ']'                                    { $$ = mk_node("ExprVec", 1, $2); }
| CONTINUE                                            { $$ = mk_node("ExprAgain", 0); }
| CONTINUE ident                                      { $$ = mk_node("ExprAgain", 1, $2); }
| RETURN                                              { $$ = mk_node("ExprRet", 0); }
| RETURN expr                                         { $$ = mk_node("ExprRet", 1, $2); }
| BREAK                                               { $$ = mk_node("ExprBreak", 0); }
| BREAK ident                                         { $$ = mk_node("ExprBreak", 1, $2); }
| expr LARROW expr                                    { $$ = mk_node("ExprInPlace", 2, $1, $3); }
| expr '=' expr                                       { $$ = mk_node("ExprAssign", 2, $1, $3); }
| expr SHLEQ expr                                     { $$ = mk_node("ExprAssignShl", 2, $1, $3); }
| expr SHREQ expr                                     { $$ = mk_node("ExprAssignShr", 2, $1, $3); }
| expr MINUSEQ expr                                   { $$ = mk_node("ExprAssignSub", 2, $1, $3); }
| expr ANDEQ expr                                     { $$ = mk_node("ExprAssignBitAnd", 2, $1, $3); }
| expr OREQ expr                                      { $$ = mk_node("ExprAssignBitOr", 2, $1, $3); }
| expr PLUSEQ expr                                    { $$ = mk_node("ExprAssignAdd", 2, $1, $3); }
| expr STAREQ expr                                    { $$ = mk_node("ExprAssignMul", 2, $1, $3); }
| expr SLASHEQ expr                                   { $$ = mk_node("ExprAssignDiv", 2, $1, $3); }
| expr CARETEQ expr                                   { $$ = mk_node("ExprAssignBitXor", 2, $1, $3); }
| expr PERCENTEQ expr                                 { $$ = mk_node("ExprAssignRem", 2, $1, $3); }
| expr OROR expr                                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiOr"), $1, $3); }
| expr ANDAND expr                                    { $$ = mk_node("ExprBinary", 3, mk_atom("BiAnd"), $1, $3); }
| expr EQEQ expr                                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiEq"), $1, $3); }
| expr NE expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiNe"), $1, $3); }
| expr '<' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiLt"), $1, $3); }
| expr '>' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiGt"), $1, $3); }
| expr LE expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiLe"), $1, $3); }
| expr GE expr                                        { $$ = mk_node("ExprBinary", 3, mk_atom("BiGe"), $1, $3); }
| expr '|' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitOr"), $1, $3); }
| expr '^' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitXor"), $1, $3); }
| expr '&' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitAnd"), $1, $3); }
| expr SHL expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiShl"), $1, $3); }
| expr SHR expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiShr"), $1, $3); }
| expr '+' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiAdd"), $1, $3); }
| expr '-' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiSub"), $1, $3); }
| expr '*' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiMul"), $1, $3); }
| expr '/' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiDiv"), $1, $3); }
| expr '%' expr                                       { $$ = mk_node("ExprBinary", 3, mk_atom("BiRem"), $1, $3); }
| expr DOTDOT                                         { $$ = mk_node("ExprRange", 2, $1, mk_none()); }
| expr DOTDOT expr                                    { $$ = mk_node("ExprRange", 2, $1, $3); }
|      DOTDOT expr                                    { $$ = mk_node("ExprRange", 2, mk_none(), $2); }
|      DOTDOT                                         { $$ = mk_node("ExprRange", 2, mk_none(), mk_none()); }
| expr AS ty                                          { $$ = mk_node("ExprCast", 2, $1, $3); }
| BOX nonparen_expr                                   { $$ = mk_node("ExprBox", 1, $2); }
| %prec BOXPLACE BOX '(' maybe_expr ')' expr          { $$ = mk_node("ExprBox", 2, $3, $5); }
| expr_qualified_path
| block_expr
| block
| nonblock_prefix_expr
;



nonparen_expr
: lit                                                 { $$ = mk_node("ExprLit", 1, $1); }
| %prec IDENT
  path_expr                                           { $$ = mk_node("ExprPath", 1, $1); }
| SELF                                                { $$ = mk_node("ExprPath", 1, mk_node("ident", 1, mk_atom("self"))); }
| macro_expr                                          { $$ = mk_node("ExprMac", 1, $1); }
| path_expr '{' struct_expr_fields '}'                { $$ = mk_node("ExprStruct", 2, $1, $3); }
| nonparen_expr '.' path_generic_args_with_colons     { $$ = mk_node("ExprField", 2, $1, $3); }
| nonparen_expr '.' LIT_INTEGER                       { $$ = mk_node("ExprTupleIndex", 1, $1); }
| nonparen_expr '[' maybe_expr ']'                    { $$ = mk_node("ExprIndex", 2, $1, $3); }
| nonparen_expr '(' maybe_exprs ')'                   { $$ = mk_node("ExprCall", 2, $1, $3); }
| '[' vec_expr ']'                                    { $$ = mk_node("ExprVec", 1, $2); }
| CONTINUE                                            { $$ = mk_node("ExprAgain", 0); }
| CONTINUE ident                                      { $$ = mk_node("ExprAgain", 1, $2); }
| RETURN                                              { $$ = mk_node("ExprRet", 0); }
| RETURN expr                                         { $$ = mk_node("ExprRet", 1, $2); }
| BREAK                                               { $$ = mk_node("ExprBreak", 0); }
| BREAK ident                                         { $$ = mk_node("ExprBreak", 1, $2); }
| nonparen_expr LARROW nonparen_expr                  { $$ = mk_node("ExprInPlace", 2, $1, $3); }
| nonparen_expr '=' nonparen_expr                     { $$ = mk_node("ExprAssign", 2, $1, $3); }
| nonparen_expr SHLEQ nonparen_expr                   { $$ = mk_node("ExprAssignShl", 2, $1, $3); }
| nonparen_expr SHREQ nonparen_expr                   { $$ = mk_node("ExprAssignShr", 2, $1, $3); }
| nonparen_expr MINUSEQ nonparen_expr                 { $$ = mk_node("ExprAssignSub", 2, $1, $3); }
| nonparen_expr ANDEQ nonparen_expr                   { $$ = mk_node("ExprAssignBitAnd", 2, $1, $3); }
| nonparen_expr OREQ nonparen_expr                    { $$ = mk_node("ExprAssignBitOr", 2, $1, $3); }
| nonparen_expr PLUSEQ nonparen_expr                  { $$ = mk_node("ExprAssignAdd", 2, $1, $3); }
| nonparen_expr STAREQ nonparen_expr                  { $$ = mk_node("ExprAssignMul", 2, $1, $3); }
| nonparen_expr SLASHEQ nonparen_expr                 { $$ = mk_node("ExprAssignDiv", 2, $1, $3); }
| nonparen_expr CARETEQ nonparen_expr                 { $$ = mk_node("ExprAssignBitXor", 2, $1, $3); }
| nonparen_expr PERCENTEQ nonparen_expr               { $$ = mk_node("ExprAssignRem", 2, $1, $3); }
| nonparen_expr OROR nonparen_expr                    { $$ = mk_node("ExprBinary", 3, mk_atom("BiOr"), $1, $3); }
| nonparen_expr ANDAND nonparen_expr                  { $$ = mk_node("ExprBinary", 3, mk_atom("BiAnd"), $1, $3); }
| nonparen_expr EQEQ nonparen_expr                    { $$ = mk_node("ExprBinary", 3, mk_atom("BiEq"), $1, $3); }
| nonparen_expr NE nonparen_expr                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiNe"), $1, $3); }
| nonparen_expr '<' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiLt"), $1, $3); }
| nonparen_expr '>' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiGt"), $1, $3); }
| nonparen_expr LE nonparen_expr                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiLe"), $1, $3); }
| nonparen_expr GE nonparen_expr                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiGe"), $1, $3); }
| nonparen_expr '|' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitOr"), $1, $3); }
| nonparen_expr '^' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitXor"), $1, $3); }
| nonparen_expr '&' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitAnd"), $1, $3); }
| nonparen_expr SHL nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiShl"), $1, $3); }
| nonparen_expr SHR nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiShr"), $1, $3); }
| nonparen_expr '+' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiAdd"), $1, $3); }
| nonparen_expr '-' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiSub"), $1, $3); }
| nonparen_expr '*' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiMul"), $1, $3); }
| nonparen_expr '/' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiDiv"), $1, $3); }
| nonparen_expr '%' nonparen_expr                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiRem"), $1, $3); }
| nonparen_expr DOTDOT                                { $$ = mk_node("ExprRange", 2, $1, mk_none()); }
| nonparen_expr DOTDOT nonparen_expr                  { $$ = mk_node("ExprRange", 2, $1, $3); }
|               DOTDOT nonparen_expr                  { $$ = mk_node("ExprRange", 2, mk_none(), $2); }
|               DOTDOT                                { $$ = mk_node("ExprRange", 2, mk_none(), mk_none()); }
| nonparen_expr AS ty                                 { $$ = mk_node("ExprCast", 2, $1, $3); }
| BOX nonparen_expr                                   { $$ = mk_node("ExprBox", 1, $2); }
| %prec BOXPLACE BOX '(' maybe_expr ')' expr          { $$ = mk_node("ExprBox", 1, $3, $5); }
| expr_qualified_path
| block_expr
| block
| nonblock_prefix_expr
;



expr_nostruct
: lit                                                 { $$ = mk_node("ExprLit", 1, $1); }
| %prec IDENT
  path_expr                                           { $$ = mk_node("ExprPath", 1, $1); }
| SELF                                                { $$ = mk_node("ExprPath", 1, mk_node("ident", 1, mk_atom("self"))); }
| macro_expr                                          { $$ = mk_node("ExprMac", 1, $1); }
| expr_nostruct '.' path_generic_args_with_colons     { $$ = mk_node("ExprField", 2, $1, $3); }
| expr_nostruct '.' LIT_INTEGER                       { $$ = mk_node("ExprTupleIndex", 1, $1); }
| expr_nostruct '[' maybe_expr ']'                    { $$ = mk_node("ExprIndex", 2, $1, $3); }
| expr_nostruct '(' maybe_exprs ')'                   { $$ = mk_node("ExprCall", 2, $1, $3); }
| '[' vec_expr ']'                                    { $$ = mk_node("ExprVec", 1, $2); }
| '(' maybe_exprs ')'                                 { $$ = mk_node("ExprParen", 1, $2); }
| CONTINUE                                            { $$ = mk_node("ExprAgain", 0); }
| CONTINUE ident                                      { $$ = mk_node("ExprAgain", 1, $2); }
| RETURN                                              { $$ = mk_node("ExprRet", 0); }
| RETURN expr                                         { $$ = mk_node("ExprRet", 1, $2); }
| BREAK                                               { $$ = mk_node("ExprBreak", 0); }
| BREAK ident                                         { $$ = mk_node("ExprBreak", 1, $2); }
| expr_nostruct LARROW expr_nostruct                  { $$ = mk_node("ExprInPlace", 2, $1, $3); }
| expr_nostruct '=' expr_nostruct                     { $$ = mk_node("ExprAssign", 2, $1, $3); }
| expr_nostruct SHLEQ expr_nostruct                   { $$ = mk_node("ExprAssignShl", 2, $1, $3); }
| expr_nostruct SHREQ expr_nostruct                   { $$ = mk_node("ExprAssignShr", 2, $1, $3); }
| expr_nostruct MINUSEQ expr_nostruct                 { $$ = mk_node("ExprAssignSub", 2, $1, $3); }
| expr_nostruct ANDEQ expr_nostruct                   { $$ = mk_node("ExprAssignBitAnd", 2, $1, $3); }
| expr_nostruct OREQ expr_nostruct                    { $$ = mk_node("ExprAssignBitOr", 2, $1, $3); }
| expr_nostruct PLUSEQ expr_nostruct                  { $$ = mk_node("ExprAssignAdd", 2, $1, $3); }
| expr_nostruct STAREQ expr_nostruct                  { $$ = mk_node("ExprAssignMul", 2, $1, $3); }
| expr_nostruct SLASHEQ expr_nostruct                 { $$ = mk_node("ExprAssignDiv", 2, $1, $3); }
| expr_nostruct CARETEQ expr_nostruct                 { $$ = mk_node("ExprAssignBitXor", 2, $1, $3); }
| expr_nostruct PERCENTEQ expr_nostruct               { $$ = mk_node("ExprAssignRem", 2, $1, $3); }
| expr_nostruct OROR expr_nostruct                    { $$ = mk_node("ExprBinary", 3, mk_atom("BiOr"), $1, $3); }
| expr_nostruct ANDAND expr_nostruct                  { $$ = mk_node("ExprBinary", 3, mk_atom("BiAnd"), $1, $3); }
| expr_nostruct EQEQ expr_nostruct                    { $$ = mk_node("ExprBinary", 3, mk_atom("BiEq"), $1, $3); }
| expr_nostruct NE expr_nostruct                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiNe"), $1, $3); }
| expr_nostruct '<' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiLt"), $1, $3); }
| expr_nostruct '>' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiGt"), $1, $3); }
| expr_nostruct LE expr_nostruct                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiLe"), $1, $3); }
| expr_nostruct GE expr_nostruct                      { $$ = mk_node("ExprBinary", 3, mk_atom("BiGe"), $1, $3); }
| expr_nostruct '|' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitOr"), $1, $3); }
| expr_nostruct '^' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitXor"), $1, $3); }
| expr_nostruct '&' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiBitAnd"), $1, $3); }
| expr_nostruct SHL expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiShl"), $1, $3); }
| expr_nostruct SHR expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiShr"), $1, $3); }
| expr_nostruct '+' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiAdd"), $1, $3); }
| expr_nostruct '-' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiSub"), $1, $3); }
| expr_nostruct '*' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiMul"), $1, $3); }
| expr_nostruct '/' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiDiv"), $1, $3); }
| expr_nostruct '%' expr_nostruct                     { $$ = mk_node("ExprBinary", 3, mk_atom("BiRem"), $1, $3); }
| expr_nostruct DOTDOT               %prec RANGE      { $$ = mk_node("ExprRange", 2, $1, mk_none()); }
| expr_nostruct DOTDOT expr_nostruct                  { $$ = mk_node("ExprRange", 2, $1, $3); }
|               DOTDOT expr_nostruct                  { $$ = mk_node("ExprRange", 2, mk_none(), $2); }
|               DOTDOT                                { $$ = mk_node("ExprRange", 2, mk_none(), mk_none()); }
| expr_nostruct AS ty                                 { $$ = mk_node("ExprCast", 2, $1, $3); }
| BOX nonparen_expr                                   { $$ = mk_node("ExprBox", 1, $2); }
| %prec BOXPLACE BOX '(' maybe_expr ')' expr_nostruct { $$ = mk_node("ExprBox", 1, $3, $5); }
| expr_qualified_path
| block_expr
| block
| nonblock_prefix_expr_nostruct
;



field_inits
: field_init                 { $$ = mk_node("FieldInits", 1, $1); }
| field_inits ',' field_init { $$ = ext_node($1, 1, $3); }
;



full_block_expr
: block_expr
| full_block_expr '.' path_generic_args_with_colons %prec IDENT         { $$ = mk_node("ExprField", 2, $1, $3); }
| full_block_expr '.' path_generic_args_with_colons '[' maybe_expr ']'  { $$ = mk_node("ExprIndex", 3, $1, $3, $5); }
| full_block_expr '.' path_generic_args_with_colons '(' maybe_exprs ')' { $$ = mk_node("ExprCall", 3, $1, $3, $5); }
| full_block_expr '.' LIT_INTEGER                                       { $$ = mk_node("ExprTupleIndex", 1, $1); }
;



match_clauses
: match_clause               { $$ = mk_node("Arms", 1, $1); }
| match_clauses match_clause { $$ = ext_node($1, 1, $2); }
;



token_trees
: %empty                     { $$ = mk_node("TokenTrees", 0); }
| token_trees token_tree     { $$ = ext_node($1, 1, $2); }
;
