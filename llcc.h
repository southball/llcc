typedef enum {
  TK_RESERVED,
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
} NodeKind;

typedef struct Node Node;

struct Node {
  NodeKind kind;
  Node *lhs;
  Node *rhs;
  int val; // Only used when kind == ND_NUM
};

// Current token and user input
extern Token *token;
extern char *user_input;

Token *tokenize(char *p);
Node *expr();
void gen(Node *node);
