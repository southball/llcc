typedef enum {
  TK_RESERVED,
  TK_IDENT,
  TK_NUM,
  TK_EOF,
} TokenKind;

typedef struct Token Token;

struct Token {
  TokenKind kind;
  Token *next;
  int val;
  char *str;
  int len;
};

typedef enum {
  ND_ADD,
  ND_SUB,
  ND_MUL,
  ND_DIV,
  ND_LT,
  ND_LE,
  ND_GT,
  ND_GE,
  ND_EQ,
  ND_NE,
  ND_NUM,
  ND_ASSIGN,
  ND_LVAR,
} NodeKind;

typedef struct Node Node;

struct Node {
  NodeKind kind;
  Node *lhs;
  Node *rhs;
  int val;    // Only used when kind == ND_NUM
  int offset; // Only used when kind == ND_LVAR
};

// Current token and user input
extern Token *token;
extern char *user_input;
extern Node *code[100];

void error(char *fmt, ...);
void error_at(char *str, char *fmt, ...);

Token *tokenize(char *p);
Node *expr();
void gen(Node *node);
