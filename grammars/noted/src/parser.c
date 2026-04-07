#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 79
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 56
#define ALIAS_COUNT 0
#define TOKEN_COUNT 34
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 9
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 9

enum ts_symbol_identifiers {
  aux_sym_heading_token1 = 1,
  aux_sym_heading_token2 = 2,
  anon_sym_GT_LBRACK_BANG = 3,
  anon_sym_RBRACK = 4,
  anon_sym_SPACE = 5,
  sym_callout_type_name = 6,
  sym_blockquote_continuation = 7,
  aux_sym_fenced_code_block_token1 = 8,
  aux_sym_fenced_code_block_token2 = 9,
  anon_sym_BQUOTE_BQUOTE_BQUOTE = 10,
  aux_sym_fenced_code_block_token3 = 11,
  aux_sym_fenced_code_block_token4 = 12,
  aux_sym_list_item_token1 = 13,
  sym_checkbox = 14,
  sym_blank_line = 15,
  sym_thematic_break = 16,
  anon_sym_LBRACK_LBRACK = 17,
  anon_sym_PIPE = 18,
  anon_sym_RBRACK_RBRACK = 19,
  sym_wikilink_target = 20,
  aux_sym_wikilink_alias_token1 = 21,
  anon_sym_BANG_LBRACK_LBRACK = 22,
  sym_tag = 23,
  anon_sym_STAR_STAR = 24,
  aux_sym_bold_text_token1 = 25,
  anon_sym_STAR = 26,
  anon_sym_BQUOTE = 27,
  aux_sym_inline_code_token1 = 28,
  anon_sym_LBRACK = 29,
  anon_sym_RBRACK_LPAREN = 30,
  aux_sym_link_token1 = 31,
  anon_sym_RPAREN = 32,
  sym_text = 33,
  sym_document = 34,
  sym_heading = 35,
  sym_callout_block = 36,
  sym_fenced_code_block = 37,
  sym_list_item = 38,
  sym_paragraph_line = 39,
  sym__inline = 40,
  sym_wikilink = 41,
  sym_wikilink_alias = 42,
  sym_embed = 43,
  sym_embed_target = 44,
  sym_bold = 45,
  sym_bold_text = 46,
  sym_italic = 47,
  sym_italic_text = 48,
  sym_inline_code = 49,
  sym_link = 50,
  aux_sym_document_repeat1 = 51,
  aux_sym_heading_repeat1 = 52,
  aux_sym_callout_block_repeat1 = 53,
  aux_sym_bold_repeat1 = 54,
  aux_sym_italic_repeat1 = 55,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_heading_token1] = "heading_token1",
  [aux_sym_heading_token2] = "heading_token2",
  [anon_sym_GT_LBRACK_BANG] = "> [!",
  [anon_sym_RBRACK] = "]",
  [anon_sym_SPACE] = " ",
  [sym_callout_type_name] = "callout_type_name",
  [sym_blockquote_continuation] = "blockquote_continuation",
  [aux_sym_fenced_code_block_token1] = "fenced_code_block_token1",
  [aux_sym_fenced_code_block_token2] = "fenced_code_block_token2",
  [anon_sym_BQUOTE_BQUOTE_BQUOTE] = "```",
  [aux_sym_fenced_code_block_token3] = "fenced_code_block_token3",
  [aux_sym_fenced_code_block_token4] = "fenced_code_block_token4",
  [aux_sym_list_item_token1] = "list_item_token1",
  [sym_checkbox] = "checkbox",
  [sym_blank_line] = "blank_line",
  [sym_thematic_break] = "thematic_break",
  [anon_sym_LBRACK_LBRACK] = "[[",
  [anon_sym_PIPE] = "|",
  [anon_sym_RBRACK_RBRACK] = "]]",
  [sym_wikilink_target] = "wikilink_target",
  [aux_sym_wikilink_alias_token1] = "wikilink_alias_token1",
  [anon_sym_BANG_LBRACK_LBRACK] = "![[",
  [sym_tag] = "tag",
  [anon_sym_STAR_STAR] = "**",
  [aux_sym_bold_text_token1] = "bold_text_token1",
  [anon_sym_STAR] = "*",
  [anon_sym_BQUOTE] = "`",
  [aux_sym_inline_code_token1] = "inline_code_token1",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK_LPAREN] = "](",
  [aux_sym_link_token1] = "link_token1",
  [anon_sym_RPAREN] = ")",
  [sym_text] = "text",
  [sym_document] = "document",
  [sym_heading] = "heading",
  [sym_callout_block] = "callout_block",
  [sym_fenced_code_block] = "fenced_code_block",
  [sym_list_item] = "list_item",
  [sym_paragraph_line] = "paragraph_line",
  [sym__inline] = "_inline",
  [sym_wikilink] = "wikilink",
  [sym_wikilink_alias] = "wikilink_alias",
  [sym_embed] = "embed",
  [sym_embed_target] = "embed_target",
  [sym_bold] = "bold",
  [sym_bold_text] = "bold_text",
  [sym_italic] = "italic",
  [sym_italic_text] = "italic_text",
  [sym_inline_code] = "inline_code",
  [sym_link] = "link",
  [aux_sym_document_repeat1] = "document_repeat1",
  [aux_sym_heading_repeat1] = "heading_repeat1",
  [aux_sym_callout_block_repeat1] = "callout_block_repeat1",
  [aux_sym_bold_repeat1] = "bold_repeat1",
  [aux_sym_italic_repeat1] = "italic_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_heading_token1] = aux_sym_heading_token1,
  [aux_sym_heading_token2] = aux_sym_heading_token2,
  [anon_sym_GT_LBRACK_BANG] = anon_sym_GT_LBRACK_BANG,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_SPACE] = anon_sym_SPACE,
  [sym_callout_type_name] = sym_callout_type_name,
  [sym_blockquote_continuation] = sym_blockquote_continuation,
  [aux_sym_fenced_code_block_token1] = aux_sym_fenced_code_block_token1,
  [aux_sym_fenced_code_block_token2] = aux_sym_fenced_code_block_token2,
  [anon_sym_BQUOTE_BQUOTE_BQUOTE] = anon_sym_BQUOTE_BQUOTE_BQUOTE,
  [aux_sym_fenced_code_block_token3] = aux_sym_fenced_code_block_token3,
  [aux_sym_fenced_code_block_token4] = aux_sym_fenced_code_block_token4,
  [aux_sym_list_item_token1] = aux_sym_list_item_token1,
  [sym_checkbox] = sym_checkbox,
  [sym_blank_line] = sym_blank_line,
  [sym_thematic_break] = sym_thematic_break,
  [anon_sym_LBRACK_LBRACK] = anon_sym_LBRACK_LBRACK,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_RBRACK_RBRACK] = anon_sym_RBRACK_RBRACK,
  [sym_wikilink_target] = sym_wikilink_target,
  [aux_sym_wikilink_alias_token1] = aux_sym_wikilink_alias_token1,
  [anon_sym_BANG_LBRACK_LBRACK] = anon_sym_BANG_LBRACK_LBRACK,
  [sym_tag] = sym_tag,
  [anon_sym_STAR_STAR] = anon_sym_STAR_STAR,
  [aux_sym_bold_text_token1] = aux_sym_bold_text_token1,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_BQUOTE] = anon_sym_BQUOTE,
  [aux_sym_inline_code_token1] = aux_sym_inline_code_token1,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK_LPAREN] = anon_sym_RBRACK_LPAREN,
  [aux_sym_link_token1] = aux_sym_link_token1,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_text] = sym_text,
  [sym_document] = sym_document,
  [sym_heading] = sym_heading,
  [sym_callout_block] = sym_callout_block,
  [sym_fenced_code_block] = sym_fenced_code_block,
  [sym_list_item] = sym_list_item,
  [sym_paragraph_line] = sym_paragraph_line,
  [sym__inline] = sym__inline,
  [sym_wikilink] = sym_wikilink,
  [sym_wikilink_alias] = sym_wikilink_alias,
  [sym_embed] = sym_embed,
  [sym_embed_target] = sym_embed_target,
  [sym_bold] = sym_bold,
  [sym_bold_text] = sym_bold_text,
  [sym_italic] = sym_italic,
  [sym_italic_text] = sym_italic_text,
  [sym_inline_code] = sym_inline_code,
  [sym_link] = sym_link,
  [aux_sym_document_repeat1] = aux_sym_document_repeat1,
  [aux_sym_heading_repeat1] = aux_sym_heading_repeat1,
  [aux_sym_callout_block_repeat1] = aux_sym_callout_block_repeat1,
  [aux_sym_bold_repeat1] = aux_sym_bold_repeat1,
  [aux_sym_italic_repeat1] = aux_sym_italic_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_heading_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_heading_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_GT_LBRACK_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SPACE] = {
    .visible = true,
    .named = false,
  },
  [sym_callout_type_name] = {
    .visible = true,
    .named = true,
  },
  [sym_blockquote_continuation] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_fenced_code_block_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_fenced_code_block_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_BQUOTE_BQUOTE_BQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_fenced_code_block_token3] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_fenced_code_block_token4] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_list_item_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_checkbox] = {
    .visible = true,
    .named = true,
  },
  [sym_blank_line] = {
    .visible = true,
    .named = true,
  },
  [sym_thematic_break] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_wikilink_target] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_wikilink_alias_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_BANG_LBRACK_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_tag] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_STAR_STAR] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_bold_text_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_inline_code_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_link_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [sym_document] = {
    .visible = true,
    .named = true,
  },
  [sym_heading] = {
    .visible = true,
    .named = true,
  },
  [sym_callout_block] = {
    .visible = true,
    .named = true,
  },
  [sym_fenced_code_block] = {
    .visible = true,
    .named = true,
  },
  [sym_list_item] = {
    .visible = true,
    .named = true,
  },
  [sym_paragraph_line] = {
    .visible = true,
    .named = true,
  },
  [sym__inline] = {
    .visible = false,
    .named = true,
  },
  [sym_wikilink] = {
    .visible = true,
    .named = true,
  },
  [sym_wikilink_alias] = {
    .visible = true,
    .named = true,
  },
  [sym_embed] = {
    .visible = true,
    .named = true,
  },
  [sym_embed_target] = {
    .visible = true,
    .named = true,
  },
  [sym_bold] = {
    .visible = true,
    .named = true,
  },
  [sym_bold_text] = {
    .visible = true,
    .named = true,
  },
  [sym_italic] = {
    .visible = true,
    .named = true,
  },
  [sym_italic_text] = {
    .visible = true,
    .named = true,
  },
  [sym_inline_code] = {
    .visible = true,
    .named = true,
  },
  [sym_link] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_document_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_heading_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_callout_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_bold_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_italic_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_alias = 1,
  field_callout_type = 2,
  field_checkbox = 3,
  field_code = 4,
  field_fence_start = 5,
  field_marker = 6,
  field_target = 7,
  field_text = 8,
  field_url = 9,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_alias] = "alias",
  [field_callout_type] = "callout_type",
  [field_checkbox] = "checkbox",
  [field_code] = "code",
  [field_fence_start] = "fence_start",
  [field_marker] = "marker",
  [field_target] = "target",
  [field_text] = "text",
  [field_url] = "url",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
  [5] = {.index = 4, .length = 1},
  [6] = {.index = 5, .length = 2},
  [7] = {.index = 7, .length = 2},
  [8] = {.index = 9, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_marker, 0},
  [1] =
    {field_checkbox, 1},
  [2] =
    {field_target, 1},
  [3] =
    {field_callout_type, 1},
  [4] =
    {field_fence_start, 0},
  [5] =
    {field_alias, 3},
    {field_target, 1},
  [7] =
    {field_text, 1},
    {field_url, 3},
  [9] =
    {field_code, 2},
    {field_fence_start, 0},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 41,
  [43] = 43,
  [44] = 43,
  [45] = 45,
  [46] = 32,
  [47] = 34,
  [48] = 48,
  [49] = 34,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 68,
  [75] = 68,
  [76] = 76,
  [77] = 72,
  [78] = 72,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token4);
      if (eof) ADVANCE(32);
      ADVANCE_MAP(
        '\t', 82,
        '\n', 60,
        ' ', 42,
        '!', 87,
        '#', 72,
        ')', 120,
        '*', 107,
        '+', 80,
        '-', 79,
        '=', 85,
        '>', 81,
        '[', 114,
        ']', 40,
        '_', 90,
        '`', 111,
        '|', 65,
      );
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      if (lookahead != 0) ADVANCE(93);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == '!') ADVANCE(38);
      if (lookahead != 0) ADVANCE(2);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead != 0) ADVANCE(2);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(35);
      if (lookahead == ' ') ADVANCE(41);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == ']') ADVANCE(18);
      if (lookahead == '|') ADVANCE(64);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(35);
      if (lookahead == '!') ADVANCE(133);
      if (lookahead == '#') ADVANCE(138);
      if (lookahead == '*') ADVANCE(109);
      if (lookahead == '[') ADVANCE(116);
      if (lookahead == '`') ADVANCE(110);
      if (lookahead != 0 &&
          lookahead != ']') ADVANCE(139);
      END_STATE();
    case 5:
      if (lookahead == '\n') ADVANCE(35);
      if (lookahead == '!') ADVANCE(133);
      if (lookahead == '#') ADVANCE(138);
      if (lookahead == '*') ADVANCE(109);
      if (lookahead == '[') ADVANCE(115);
      if (lookahead == ']') ADVANCE(39);
      if (lookahead == '`') ADVANCE(110);
      if (lookahead != 0) ADVANCE(139);
      END_STATE();
    case 6:
      if (lookahead == ' ') ADVANCE(59);
      END_STATE();
    case 7:
      if (lookahead == ' ') ADVANCE(33);
      END_STATE();
    case 8:
      if (lookahead == ' ') ADVANCE(33);
      if (lookahead == '#') ADVANCE(9);
      END_STATE();
    case 9:
      if (lookahead == ' ') ADVANCE(33);
      if (lookahead == '#') ADVANCE(7);
      END_STATE();
    case 10:
      if (lookahead == ' ') ADVANCE(33);
      if (lookahead == '#') ADVANCE(8);
      END_STATE();
    case 11:
      if (lookahead == ' ') ADVANCE(33);
      if (lookahead == '#') ADVANCE(10);
      END_STATE();
    case 12:
      if (lookahead == ' ') ADVANCE(55);
      END_STATE();
    case 13:
      if (lookahead == '!') ADVANCE(36);
      END_STATE();
    case 14:
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ']') ADVANCE(66);
      END_STATE();
    case 15:
      if (lookahead == '*') ADVANCE(109);
      if (lookahead == '`') ADVANCE(110);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(105);
      END_STATE();
    case 16:
      if (lookahead == '*') ADVANCE(106);
      if (lookahead == '`') ADVANCE(110);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(105);
      END_STATE();
    case 17:
      if (lookahead == '[') ADVANCE(101);
      END_STATE();
    case 18:
      if (lookahead == ']') ADVANCE(66);
      END_STATE();
    case 19:
      if (lookahead == ']') ADVANCE(6);
      END_STATE();
    case 20:
      if (lookahead == '`') ADVANCE(48);
      END_STATE();
    case 21:
      if (lookahead == '`') ADVANCE(50);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 22:
      if (lookahead == '`') ADVANCE(29);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 23:
      if (lookahead == '`') ADVANCE(21);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 24:
      if (lookahead == '`') ADVANCE(23);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 25:
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']' &&
          lookahead != '|') ADVANCE(96);
      END_STATE();
    case 26:
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ')') ADVANCE(118);
      END_STATE();
    case 27:
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(100);
      END_STATE();
    case 28:
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '`') ADVANCE(113);
      END_STATE();
    case 29:
      if (lookahead != 0 &&
          lookahead != '`') ADVANCE(49);
      END_STATE();
    case 30:
      if (eof) ADVANCE(32);
      ADVANCE_MAP(
        '\n', 60,
        '!', 133,
        '#', 124,
        '*', 108,
        '+', 126,
        '-', 125,
        '=', 132,
        '>', 127,
        '[', 116,
        ']', 14,
        '_', 136,
        '`', 112,
        '\t', 129,
        ' ', 129,
      );
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(137);
      if (lookahead != 0) ADVANCE(139);
      END_STATE();
    case 31:
      if (eof) ADVANCE(32);
      ADVANCE_MAP(
        '\n', 60,
        '!', 133,
        '#', 124,
        '*', 108,
        '+', 126,
        '-', 125,
        '=', 132,
        '>', 128,
        '[', 116,
        '_', 136,
        '`', 112,
        '\t', 129,
        ' ', 129,
      );
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(137);
      if (lookahead != 0 &&
          lookahead != ']') ADVANCE(139);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym_heading_token1);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(aux_sym_heading_token1);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(aux_sym_heading_token2);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_GT_LBRACK_BANG);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_GT_LBRACK_BANG);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == ']') ADVANCE(2);
      if (lookahead == '|') ADVANCE(98);
      if (lookahead != 0) ADVANCE(70);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_GT_LBRACK_BANG);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead != 0) ADVANCE(2);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ']') ADVANCE(66);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_SPACE);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_SPACE);
      ADVANCE_MAP(
        '*', 78,
        '|', 99,
        '\t', 82,
        ' ', 82,
        '+', 80,
        '-', 80,
        '!', 95,
        '#', 95,
        '[', 95,
        '`', 95,
      );
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_callout_type_name);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_callout_type_name);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_blockquote_continuation);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token1);
      if (lookahead == ']') ADVANCE(48);
      if (lookahead == '|') ADVANCE(47);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(46);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token1);
      if (lookahead == ']') ADVANCE(48);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(48);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token2);
      if (lookahead == '`') ADVANCE(22);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_BQUOTE_BQUOTE_BQUOTE);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_BQUOTE_BQUOTE_BQUOTE);
      if (lookahead == ']') ADVANCE(48);
      if (lookahead == '|') ADVANCE(47);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(46);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token3);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(52);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token4);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(aux_sym_fenced_code_block_token4);
      if (lookahead == '\n') ADVANCE(53);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(aux_sym_list_item_token1);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(aux_sym_list_item_token1);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(aux_sym_list_item_token1);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(aux_sym_list_item_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_checkbox);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_blank_line);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_thematic_break);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(anon_sym_LBRACK_LBRACK);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_LBRACK_LBRACK);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(99);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_RBRACK_RBRACK);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == '!') ADVANCE(37);
      if (lookahead == ']') ADVANCE(2);
      if (lookahead == '|') ADVANCE(98);
      if (lookahead != 0) ADVANCE(70);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_wikilink_target);
      ADVANCE_MAP(
        '\n', 45,
        '[', 67,
        ']', 2,
        '|', 97,
        '!', 70,
        '#', 70,
        '*', 70,
        '`', 70,
      );
      if (lookahead != 0) ADVANCE(69);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_wikilink_target);
      ADVANCE_MAP(
        '\n', 45,
        ']', 2,
        '|', 97,
        '!', 70,
        '#', 70,
        '*', 70,
        '[', 70,
        '`', 70,
      );
      if (lookahead != 0) ADVANCE(69);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == ']') ADVANCE(2);
      if (lookahead == '|') ADVANCE(98);
      if (lookahead != 0) ADVANCE(70);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_wikilink_target);
      ADVANCE_MAP(
        '\n', 61,
        ' ', 71,
        '|', 99,
        '!', 95,
        '#', 95,
        '*', 95,
        '[', 95,
        '`', 95,
      );
      if (lookahead != 0 &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '#') ADVANCE(76);
      if (lookahead == '|') ADVANCE(100);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(94);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '#') ADVANCE(77);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '#') ADVANCE(73);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '#') ADVANCE(74);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '#') ADVANCE(75);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(34);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(57);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_wikilink_target);
      ADVANCE_MAP(
        ' ', 56,
        '-', 83,
        '|', 99,
        '!', 95,
        '#', 95,
        '*', 95,
        '[', 95,
        '`', 95,
      );
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(56);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ' ') ADVANCE(68);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(sym_wikilink_target);
      ADVANCE_MAP(
        '*', 78,
        '|', 99,
        '\t', 82,
        ' ', 82,
        '+', 80,
        '-', 80,
        '!', 95,
        '#', 95,
        '[', 95,
        '`', 95,
      );
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '-') ADVANCE(71);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '=') ADVANCE(71);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '=') ADVANCE(84);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '[') ADVANCE(102);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '[') ADVANCE(86);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == ']') ADVANCE(6);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(95);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '_') ADVANCE(71);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '_') ADVANCE(89);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '`') ADVANCE(51);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_wikilink_target);
      ADVANCE_MAP(
        '|', 99,
        ')', 80,
        '.', 80,
        '!', 95,
        '#', 95,
        '*', 95,
        '[', 95,
        '`', 95,
      );
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(92);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead == '-' ||
          ('/' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(94);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(sym_wikilink_target);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']' &&
          lookahead != '|') ADVANCE(96);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(aux_sym_wikilink_alias_token1);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == ']') ADVANCE(2);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(98);
      if (lookahead != 0) ADVANCE(97);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(aux_sym_wikilink_alias_token1);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == ']') ADVANCE(2);
      if (lookahead != 0) ADVANCE(98);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(aux_sym_wikilink_alias_token1);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(99);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(aux_sym_wikilink_alias_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(100);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_BANG_LBRACK_LBRACK);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_BANG_LBRACK_LBRACK);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(sym_tag);
      if (lookahead == '-' ||
          ('/' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(103);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_STAR_STAR);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(aux_sym_bold_text_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '*' &&
          lookahead != '`') ADVANCE(105);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == ' ') ADVANCE(57);
      if (lookahead == '*') ADVANCE(104);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == ' ') ADVANCE(55);
      if (lookahead == '*') ADVANCE(104);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == '*') ADVANCE(104);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_BQUOTE);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_BQUOTE);
      if (lookahead == '`') ADVANCE(91);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_BQUOTE);
      if (lookahead == '`') ADVANCE(20);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(aux_sym_inline_code_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '`') ADVANCE(113);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == ' ') ADVANCE(88);
      if (lookahead == 'X') ADVANCE(88);
      if (lookahead == '[') ADVANCE(63);
      if (lookahead == 'x') ADVANCE(88);
      if (lookahead == '|') ADVANCE(100);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(95);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == ' ') ADVANCE(19);
      if (lookahead == 'X') ADVANCE(19);
      if (lookahead == '[') ADVANCE(62);
      if (lookahead == 'x') ADVANCE(19);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == '[') ADVANCE(62);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_RBRACK_LPAREN);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(aux_sym_link_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ')') ADVANCE(118);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      if (lookahead == '|') ADVANCE(99);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == '`') ADVANCE(95);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ']') ADVANCE(93);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == '[') ADVANCE(1);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == ']' ||
          lookahead == '`') ADVANCE(2);
      if (lookahead != 0) ADVANCE(122);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == '!' ||
          lookahead == '#' ||
          lookahead == '*' ||
          lookahead == '[' ||
          lookahead == ']' ||
          lookahead == '`') ADVANCE(2);
      if (lookahead != 0) ADVANCE(122);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '\n') ADVANCE(61);
      if (lookahead == ' ') ADVANCE(123);
      if (lookahead != 0 &&
          lookahead != ' ' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == ' ') ADVANCE(33);
      if (lookahead == '#') ADVANCE(11);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(103);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == ' ') ADVANCE(58);
      if (lookahead == '-') ADVANCE(130);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ' ' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == ' ') ADVANCE(58);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ' ' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == ' ') ADVANCE(134);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ' ' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == ' ') ADVANCE(121);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != ' ' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(129);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(126);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(137);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != ' ' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '-') ADVANCE(123);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '=') ADVANCE(123);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '=') ADVANCE(131);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '[') ADVANCE(17);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '[') ADVANCE(13);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '_') ADVANCE(123);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '_' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '_') ADVANCE(135);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '_' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == ')' ||
          lookahead == '.') ADVANCE(126);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(137);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != ')' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(sym_text);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(103);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(sym_text);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '!' &&
          lookahead != '#' &&
          lookahead != '*' &&
          lookahead != '[' &&
          lookahead != ']' &&
          lookahead != '`') ADVANCE(139);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 30},
  [2] = {.lex_state = 30},
  [3] = {.lex_state = 30},
  [4] = {.lex_state = 5},
  [5] = {.lex_state = 4},
  [6] = {.lex_state = 4},
  [7] = {.lex_state = 4},
  [8] = {.lex_state = 4},
  [9] = {.lex_state = 4},
  [10] = {.lex_state = 4},
  [11] = {.lex_state = 4},
  [12] = {.lex_state = 31},
  [13] = {.lex_state = 31},
  [14] = {.lex_state = 4},
  [15] = {.lex_state = 31},
  [16] = {.lex_state = 31},
  [17] = {.lex_state = 31},
  [18] = {.lex_state = 4},
  [19] = {.lex_state = 31},
  [20] = {.lex_state = 31},
  [21] = {.lex_state = 30},
  [22] = {.lex_state = 30},
  [23] = {.lex_state = 30},
  [24] = {.lex_state = 30},
  [25] = {.lex_state = 30},
  [26] = {.lex_state = 30},
  [27] = {.lex_state = 30},
  [28] = {.lex_state = 30},
  [29] = {.lex_state = 30},
  [30] = {.lex_state = 4},
  [31] = {.lex_state = 4},
  [32] = {.lex_state = 4},
  [33] = {.lex_state = 4},
  [34] = {.lex_state = 4},
  [35] = {.lex_state = 4},
  [36] = {.lex_state = 4},
  [37] = {.lex_state = 15},
  [38] = {.lex_state = 15},
  [39] = {.lex_state = 16},
  [40] = {.lex_state = 16},
  [41] = {.lex_state = 16},
  [42] = {.lex_state = 16},
  [43] = {.lex_state = 15},
  [44] = {.lex_state = 15},
  [45] = {.lex_state = 15},
  [46] = {.lex_state = 15},
  [47] = {.lex_state = 15},
  [48] = {.lex_state = 16},
  [49] = {.lex_state = 16},
  [50] = {.lex_state = 27},
  [51] = {.lex_state = 24},
  [52] = {.lex_state = 3},
  [53] = {.lex_state = 27},
  [54] = {.lex_state = 3},
  [55] = {.lex_state = 30},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 3},
  [58] = {.lex_state = 3},
  [59] = {.lex_state = 30},
  [60] = {.lex_state = 30},
  [61] = {.lex_state = 54},
  [62] = {.lex_state = 5},
  [63] = {.lex_state = 5},
  [64] = {.lex_state = 24},
  [65] = {.lex_state = 26},
  [66] = {.lex_state = 52},
  [67] = {.lex_state = 25},
  [68] = {.lex_state = 5},
  [69] = {.lex_state = 30},
  [70] = {.lex_state = 52},
  [71] = {.lex_state = 54},
  [72] = {.lex_state = 28},
  [73] = {.lex_state = 27},
  [74] = {.lex_state = 5},
  [75] = {.lex_state = 5},
  [76] = {.lex_state = 30},
  [77] = {.lex_state = 28},
  [78] = {.lex_state = 28},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_heading_token1] = ACTIONS(1),
    [aux_sym_heading_token2] = ACTIONS(1),
    [anon_sym_GT_LBRACK_BANG] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_SPACE] = ACTIONS(1),
    [sym_callout_type_name] = ACTIONS(1),
    [sym_blockquote_continuation] = ACTIONS(1),
    [aux_sym_fenced_code_block_token1] = ACTIONS(1),
    [anon_sym_BQUOTE_BQUOTE_BQUOTE] = ACTIONS(1),
    [aux_sym_fenced_code_block_token4] = ACTIONS(1),
    [aux_sym_list_item_token1] = ACTIONS(1),
    [sym_checkbox] = ACTIONS(1),
    [sym_blank_line] = ACTIONS(1),
    [sym_thematic_break] = ACTIONS(1),
    [anon_sym_LBRACK_LBRACK] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_RBRACK_RBRACK] = ACTIONS(1),
    [sym_wikilink_target] = ACTIONS(1),
    [aux_sym_wikilink_alias_token1] = ACTIONS(1),
    [anon_sym_BANG_LBRACK_LBRACK] = ACTIONS(1),
    [sym_tag] = ACTIONS(1),
    [anon_sym_STAR_STAR] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_BQUOTE] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_text] = ACTIONS(1),
  },
  [1] = {
    [sym_document] = STATE(56),
    [sym_heading] = STATE(3),
    [sym_callout_block] = STATE(3),
    [sym_fenced_code_block] = STATE(3),
    [sym_list_item] = STATE(3),
    [sym_paragraph_line] = STATE(3),
    [sym__inline] = STATE(5),
    [sym_wikilink] = STATE(5),
    [sym_embed] = STATE(5),
    [sym_bold] = STATE(5),
    [sym_italic] = STATE(5),
    [sym_inline_code] = STATE(5),
    [sym_link] = STATE(5),
    [aux_sym_document_repeat1] = STATE(3),
    [aux_sym_heading_repeat1] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_heading_token1] = ACTIONS(5),
    [anon_sym_GT_LBRACK_BANG] = ACTIONS(7),
    [aux_sym_fenced_code_block_token1] = ACTIONS(9),
    [aux_sym_list_item_token1] = ACTIONS(11),
    [sym_blank_line] = ACTIONS(13),
    [sym_thematic_break] = ACTIONS(13),
    [anon_sym_LBRACK_LBRACK] = ACTIONS(15),
    [anon_sym_BANG_LBRACK_LBRACK] = ACTIONS(17),
    [sym_tag] = ACTIONS(19),
    [anon_sym_STAR_STAR] = ACTIONS(21),
    [anon_sym_STAR] = ACTIONS(23),
    [anon_sym_BQUOTE] = ACTIONS(25),
    [anon_sym_LBRACK] = ACTIONS(27),
    [sym_text] = ACTIONS(29),
  },
  [2] = {
    [sym_heading] = STATE(2),
    [sym_callout_block] = STATE(2),
    [sym_fenced_code_block] = STATE(2),
    [sym_list_item] = STATE(2),
    [sym_paragraph_line] = STATE(2),
    [sym__inline] = STATE(5),
    [sym_wikilink] = STATE(5),
    [sym_embed] = STATE(5),
    [sym_bold] = STATE(5),
    [sym_italic] = STATE(5),
    [sym_inline_code] = STATE(5),
    [sym_link] = STATE(5),
    [aux_sym_document_repeat1] = STATE(2),
    [aux_sym_heading_repeat1] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(31),
    [aux_sym_heading_token1] = ACTIONS(33),
    [anon_sym_GT_LBRACK_BANG] = ACTIONS(36),
    [aux_sym_fenced_code_block_token1] = ACTIONS(39),
    [aux_sym_list_item_token1] = ACTIONS(42),
    [sym_blank_line] = ACTIONS(45),
    [sym_thematic_break] = ACTIONS(45),
    [anon_sym_LBRACK_LBRACK] = ACTIONS(48),
    [anon_sym_BANG_LBRACK_LBRACK] = ACTIONS(51),
    [sym_tag] = ACTIONS(54),
    [anon_sym_STAR_STAR] = ACTIONS(57),
    [anon_sym_STAR] = ACTIONS(60),
    [anon_sym_BQUOTE] = ACTIONS(63),
    [anon_sym_LBRACK] = ACTIONS(66),
    [sym_text] = ACTIONS(69),
  },
  [3] = {
    [sym_heading] = STATE(2),
    [sym_callout_block] = STATE(2),
    [sym_fenced_code_block] = STATE(2),
    [sym_list_item] = STATE(2),
    [sym_paragraph_line] = STATE(2),
    [sym__inline] = STATE(5),
    [sym_wikilink] = STATE(5),
    [sym_embed] = STATE(5),
    [sym_bold] = STATE(5),
    [sym_italic] = STATE(5),
    [sym_inline_code] = STATE(5),
    [sym_link] = STATE(5),
    [aux_sym_document_repeat1] = STATE(2),
    [aux_sym_heading_repeat1] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(72),
    [aux_sym_heading_token1] = ACTIONS(5),
    [anon_sym_GT_LBRACK_BANG] = ACTIONS(7),
    [aux_sym_fenced_code_block_token1] = ACTIONS(9),
    [aux_sym_list_item_token1] = ACTIONS(11),
    [sym_blank_line] = ACTIONS(74),
    [sym_thematic_break] = ACTIONS(74),
    [anon_sym_LBRACK_LBRACK] = ACTIONS(15),
    [anon_sym_BANG_LBRACK_LBRACK] = ACTIONS(17),
    [sym_tag] = ACTIONS(19),
    [anon_sym_STAR_STAR] = ACTIONS(21),
    [anon_sym_STAR] = ACTIONS(23),
    [anon_sym_BQUOTE] = ACTIONS(25),
    [anon_sym_LBRACK] = ACTIONS(27),
    [sym_text] = ACTIONS(29),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 11,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(76), 1,
      aux_sym_heading_token2,
    ACTIONS(78), 1,
      sym_checkbox,
    ACTIONS(80), 1,
      sym_tag,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(84), 1,
      sym_text,
    STATE(8), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [41] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(86), 1,
      aux_sym_heading_token2,
    ACTIONS(88), 1,
      sym_tag,
    ACTIONS(90), 1,
      sym_text,
    STATE(10), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [79] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(92), 1,
      aux_sym_heading_token2,
    ACTIONS(94), 1,
      sym_tag,
    ACTIONS(96), 1,
      sym_text,
    STATE(7), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [117] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(88), 1,
      sym_tag,
    ACTIONS(90), 1,
      sym_text,
    ACTIONS(98), 1,
      aux_sym_heading_token2,
    STATE(10), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [155] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(88), 1,
      sym_tag,
    ACTIONS(90), 1,
      sym_text,
    ACTIONS(100), 1,
      aux_sym_heading_token2,
    STATE(10), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [193] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(102), 1,
      aux_sym_heading_token2,
    ACTIONS(104), 1,
      sym_tag,
    ACTIONS(106), 1,
      sym_text,
    STATE(11), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [231] = 10,
    ACTIONS(108), 1,
      aux_sym_heading_token2,
    ACTIONS(110), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(113), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(116), 1,
      sym_tag,
    ACTIONS(119), 1,
      anon_sym_STAR_STAR,
    ACTIONS(122), 1,
      anon_sym_STAR,
    ACTIONS(125), 1,
      anon_sym_BQUOTE,
    ACTIONS(128), 1,
      anon_sym_LBRACK,
    ACTIONS(131), 1,
      sym_text,
    STATE(10), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [269] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(88), 1,
      sym_tag,
    ACTIONS(90), 1,
      sym_text,
    ACTIONS(134), 1,
      aux_sym_heading_token2,
    STATE(10), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [307] = 4,
    ACTIONS(140), 1,
      sym_blockquote_continuation,
    STATE(12), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(138), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(136), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [333] = 4,
    ACTIONS(147), 1,
      sym_blockquote_continuation,
    STATE(16), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(145), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(143), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [359] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(149), 1,
      aux_sym_heading_token2,
    ACTIONS(151), 1,
      sym_tag,
    ACTIONS(153), 1,
      sym_text,
    STATE(18), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [397] = 4,
    ACTIONS(159), 1,
      sym_blockquote_continuation,
    STATE(12), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(157), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(155), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [423] = 4,
    ACTIONS(159), 1,
      sym_blockquote_continuation,
    STATE(12), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(163), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(161), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [449] = 4,
    ACTIONS(165), 1,
      sym_blockquote_continuation,
    STATE(15), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(163), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(161), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [475] = 10,
    ACTIONS(15), 1,
      anon_sym_LBRACK_LBRACK,
    ACTIONS(17), 1,
      anon_sym_BANG_LBRACK_LBRACK,
    ACTIONS(21), 1,
      anon_sym_STAR_STAR,
    ACTIONS(23), 1,
      anon_sym_STAR,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(82), 1,
      anon_sym_BQUOTE,
    ACTIONS(88), 1,
      sym_tag,
    ACTIONS(90), 1,
      sym_text,
    ACTIONS(167), 1,
      aux_sym_heading_token2,
    STATE(10), 8,
      sym__inline,
      sym_wikilink,
      sym_embed,
      sym_bold,
      sym_italic,
      sym_inline_code,
      sym_link,
      aux_sym_heading_repeat1,
  [513] = 4,
    ACTIONS(169), 1,
      sym_blockquote_continuation,
    STATE(20), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(157), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(155), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [539] = 4,
    ACTIONS(159), 1,
      sym_blockquote_continuation,
    STATE(12), 1,
      aux_sym_callout_block_repeat1,
    ACTIONS(173), 6,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(171), 9,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [565] = 2,
    ACTIONS(177), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(175), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [585] = 2,
    ACTIONS(181), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(179), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [605] = 2,
    ACTIONS(185), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(183), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [625] = 2,
    ACTIONS(189), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(187), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [645] = 2,
    ACTIONS(193), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(191), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [665] = 2,
    ACTIONS(197), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(195), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [685] = 2,
    ACTIONS(201), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(199), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [705] = 2,
    ACTIONS(205), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(203), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [725] = 2,
    ACTIONS(209), 5,
      aux_sym_list_item_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(207), 10,
      ts_builtin_sym_end,
      aux_sym_heading_token1,
      anon_sym_GT_LBRACK_BANG,
      aux_sym_fenced_code_block_token1,
      sym_blank_line,
      sym_thematic_break,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
  [745] = 2,
    ACTIONS(213), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(211), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [759] = 2,
    ACTIONS(217), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(215), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [773] = 2,
    ACTIONS(221), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(219), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [787] = 2,
    ACTIONS(225), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(223), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [801] = 2,
    ACTIONS(229), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(227), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [815] = 2,
    ACTIONS(233), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(231), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [829] = 2,
    ACTIONS(237), 3,
      anon_sym_STAR,
      anon_sym_LBRACK,
      sym_text,
    ACTIONS(235), 6,
      aux_sym_heading_token2,
      anon_sym_LBRACK_LBRACK,
      anon_sym_BANG_LBRACK_LBRACK,
      sym_tag,
      anon_sym_STAR_STAR,
      anon_sym_BQUOTE,
  [843] = 5,
    ACTIONS(239), 1,
      anon_sym_STAR_STAR,
    ACTIONS(241), 1,
      aux_sym_bold_text_token1,
    ACTIONS(244), 1,
      anon_sym_STAR,
    ACTIONS(247), 1,
      anon_sym_BQUOTE,
    STATE(37), 4,
      sym_bold_text,
      sym_italic,
      sym_inline_code,
      aux_sym_bold_repeat1,
  [862] = 5,
    ACTIONS(250), 1,
      anon_sym_STAR_STAR,
    ACTIONS(252), 1,
      aux_sym_bold_text_token1,
    ACTIONS(254), 1,
      anon_sym_STAR,
    ACTIONS(256), 1,
      anon_sym_BQUOTE,
    STATE(37), 4,
      sym_bold_text,
      sym_italic,
      sym_inline_code,
      aux_sym_bold_repeat1,
  [881] = 4,
    ACTIONS(252), 1,
      aux_sym_bold_text_token1,
    ACTIONS(256), 1,
      anon_sym_BQUOTE,
    ACTIONS(258), 1,
      anon_sym_STAR,
    STATE(38), 4,
      sym_bold_text,
      sym_italic,
      sym_inline_code,
      aux_sym_bold_repeat1,
  [897] = 4,
    ACTIONS(260), 1,
      aux_sym_bold_text_token1,
    ACTIONS(263), 1,
      anon_sym_STAR,
    ACTIONS(265), 1,
      anon_sym_BQUOTE,
    STATE(40), 3,
      sym_italic_text,
      sym_inline_code,
      aux_sym_italic_repeat1,
  [912] = 4,
    ACTIONS(268), 1,
      aux_sym_bold_text_token1,
    ACTIONS(270), 1,
      anon_sym_STAR,
    ACTIONS(272), 1,
      anon_sym_BQUOTE,
    STATE(40), 3,
      sym_italic_text,
      sym_inline_code,
      aux_sym_italic_repeat1,
  [927] = 4,
    ACTIONS(268), 1,
      aux_sym_bold_text_token1,
    ACTIONS(272), 1,
      anon_sym_BQUOTE,
    ACTIONS(274), 1,
      anon_sym_STAR,
    STATE(40), 3,
      sym_italic_text,
      sym_inline_code,
      aux_sym_italic_repeat1,
  [942] = 3,
    ACTIONS(268), 1,
      aux_sym_bold_text_token1,
    ACTIONS(272), 1,
      anon_sym_BQUOTE,
    STATE(42), 3,
      sym_italic_text,
      sym_inline_code,
      aux_sym_italic_repeat1,
  [954] = 3,
    ACTIONS(268), 1,
      aux_sym_bold_text_token1,
    ACTIONS(272), 1,
      anon_sym_BQUOTE,
    STATE(41), 3,
      sym_italic_text,
      sym_inline_code,
      aux_sym_italic_repeat1,
  [966] = 2,
    ACTIONS(278), 1,
      anon_sym_STAR,
    ACTIONS(276), 3,
      anon_sym_STAR_STAR,
      aux_sym_bold_text_token1,
      anon_sym_BQUOTE,
  [975] = 2,
    ACTIONS(221), 1,
      anon_sym_STAR,
    ACTIONS(219), 3,
      anon_sym_STAR_STAR,
      aux_sym_bold_text_token1,
      anon_sym_BQUOTE,
  [984] = 2,
    ACTIONS(229), 1,
      anon_sym_STAR,
    ACTIONS(227), 3,
      anon_sym_STAR_STAR,
      aux_sym_bold_text_token1,
      anon_sym_BQUOTE,
  [993] = 1,
    ACTIONS(280), 3,
      aux_sym_bold_text_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
  [999] = 1,
    ACTIONS(227), 3,
      aux_sym_bold_text_token1,
      anon_sym_STAR,
      anon_sym_BQUOTE,
  [1005] = 2,
    ACTIONS(282), 1,
      aux_sym_wikilink_alias_token1,
    STATE(60), 1,
      sym_embed_target,
  [1012] = 2,
    ACTIONS(284), 1,
      aux_sym_fenced_code_block_token2,
    ACTIONS(286), 1,
      anon_sym_BQUOTE_BQUOTE_BQUOTE,
  [1019] = 2,
    ACTIONS(288), 1,
      aux_sym_heading_token2,
    ACTIONS(290), 1,
      anon_sym_SPACE,
  [1026] = 2,
    ACTIONS(292), 1,
      aux_sym_wikilink_alias_token1,
    STATE(76), 1,
      sym_wikilink_alias,
  [1033] = 2,
    ACTIONS(294), 1,
      anon_sym_PIPE,
    ACTIONS(296), 1,
      anon_sym_RBRACK_RBRACK,
  [1040] = 1,
    ACTIONS(298), 1,
      anon_sym_RBRACK_RBRACK,
  [1044] = 1,
    ACTIONS(300), 1,
      ts_builtin_sym_end,
  [1048] = 1,
    ACTIONS(302), 1,
      anon_sym_RPAREN,
  [1052] = 1,
    ACTIONS(304), 1,
      sym_callout_type_name,
  [1056] = 1,
    ACTIONS(306), 1,
      anon_sym_RBRACK_RBRACK,
  [1060] = 1,
    ACTIONS(308), 1,
      anon_sym_RBRACK_RBRACK,
  [1064] = 1,
    ACTIONS(310), 1,
      aux_sym_fenced_code_block_token4,
  [1068] = 1,
    ACTIONS(312), 1,
      anon_sym_RBRACK,
  [1072] = 1,
    ACTIONS(314), 1,
      aux_sym_heading_token2,
  [1076] = 1,
    ACTIONS(316), 1,
      anon_sym_BQUOTE_BQUOTE_BQUOTE,
  [1080] = 1,
    ACTIONS(318), 1,
      aux_sym_link_token1,
  [1084] = 1,
    ACTIONS(320), 1,
      aux_sym_fenced_code_block_token3,
  [1088] = 1,
    ACTIONS(322), 1,
      sym_wikilink_target,
  [1092] = 1,
    ACTIONS(324), 1,
      anon_sym_BQUOTE,
  [1096] = 1,
    ACTIONS(326), 1,
      anon_sym_RBRACK_LPAREN,
  [1100] = 1,
    ACTIONS(328), 1,
      aux_sym_fenced_code_block_token3,
  [1104] = 1,
    ACTIONS(330), 1,
      aux_sym_fenced_code_block_token4,
  [1108] = 1,
    ACTIONS(332), 1,
      aux_sym_inline_code_token1,
  [1112] = 1,
    ACTIONS(334), 1,
      aux_sym_wikilink_alias_token1,
  [1116] = 1,
    ACTIONS(336), 1,
      anon_sym_BQUOTE,
  [1120] = 1,
    ACTIONS(338), 1,
      anon_sym_BQUOTE,
  [1124] = 1,
    ACTIONS(340), 1,
      anon_sym_RBRACK_RBRACK,
  [1128] = 1,
    ACTIONS(342), 1,
      aux_sym_inline_code_token1,
  [1132] = 1,
    ACTIONS(344), 1,
      aux_sym_inline_code_token1,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 41,
  [SMALL_STATE(6)] = 79,
  [SMALL_STATE(7)] = 117,
  [SMALL_STATE(8)] = 155,
  [SMALL_STATE(9)] = 193,
  [SMALL_STATE(10)] = 231,
  [SMALL_STATE(11)] = 269,
  [SMALL_STATE(12)] = 307,
  [SMALL_STATE(13)] = 333,
  [SMALL_STATE(14)] = 359,
  [SMALL_STATE(15)] = 397,
  [SMALL_STATE(16)] = 423,
  [SMALL_STATE(17)] = 449,
  [SMALL_STATE(18)] = 475,
  [SMALL_STATE(19)] = 513,
  [SMALL_STATE(20)] = 539,
  [SMALL_STATE(21)] = 565,
  [SMALL_STATE(22)] = 585,
  [SMALL_STATE(23)] = 605,
  [SMALL_STATE(24)] = 625,
  [SMALL_STATE(25)] = 645,
  [SMALL_STATE(26)] = 665,
  [SMALL_STATE(27)] = 685,
  [SMALL_STATE(28)] = 705,
  [SMALL_STATE(29)] = 725,
  [SMALL_STATE(30)] = 745,
  [SMALL_STATE(31)] = 759,
  [SMALL_STATE(32)] = 773,
  [SMALL_STATE(33)] = 787,
  [SMALL_STATE(34)] = 801,
  [SMALL_STATE(35)] = 815,
  [SMALL_STATE(36)] = 829,
  [SMALL_STATE(37)] = 843,
  [SMALL_STATE(38)] = 862,
  [SMALL_STATE(39)] = 881,
  [SMALL_STATE(40)] = 897,
  [SMALL_STATE(41)] = 912,
  [SMALL_STATE(42)] = 927,
  [SMALL_STATE(43)] = 942,
  [SMALL_STATE(44)] = 954,
  [SMALL_STATE(45)] = 966,
  [SMALL_STATE(46)] = 975,
  [SMALL_STATE(47)] = 984,
  [SMALL_STATE(48)] = 993,
  [SMALL_STATE(49)] = 999,
  [SMALL_STATE(50)] = 1005,
  [SMALL_STATE(51)] = 1012,
  [SMALL_STATE(52)] = 1019,
  [SMALL_STATE(53)] = 1026,
  [SMALL_STATE(54)] = 1033,
  [SMALL_STATE(55)] = 1040,
  [SMALL_STATE(56)] = 1044,
  [SMALL_STATE(57)] = 1048,
  [SMALL_STATE(58)] = 1052,
  [SMALL_STATE(59)] = 1056,
  [SMALL_STATE(60)] = 1060,
  [SMALL_STATE(61)] = 1064,
  [SMALL_STATE(62)] = 1068,
  [SMALL_STATE(63)] = 1072,
  [SMALL_STATE(64)] = 1076,
  [SMALL_STATE(65)] = 1080,
  [SMALL_STATE(66)] = 1084,
  [SMALL_STATE(67)] = 1088,
  [SMALL_STATE(68)] = 1092,
  [SMALL_STATE(69)] = 1096,
  [SMALL_STATE(70)] = 1100,
  [SMALL_STATE(71)] = 1104,
  [SMALL_STATE(72)] = 1108,
  [SMALL_STATE(73)] = 1112,
  [SMALL_STATE(74)] = 1116,
  [SMALL_STATE(75)] = 1120,
  [SMALL_STATE(76)] = 1124,
  [SMALL_STATE(77)] = 1128,
  [SMALL_STATE(78)] = 1132,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(44),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(72),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(6),
  [36] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(58),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(63),
  [42] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [45] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(2),
  [48] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(67),
  [51] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(50),
  [54] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [57] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(39),
  [60] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(44),
  [63] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [66] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(73),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1, 0, 0),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [84] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [88] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [90] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [96] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [106] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0),
  [110] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(67),
  [113] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(50),
  [116] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(10),
  [119] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(39),
  [122] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(44),
  [125] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(72),
  [128] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(73),
  [131] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_heading_repeat1, 2, 0, 0), SHIFT_REPEAT(10),
  [134] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_callout_block_repeat1, 2, 0, 0),
  [138] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_callout_block_repeat1, 2, 0, 0),
  [140] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_callout_block_repeat1, 2, 0, 0), SHIFT_REPEAT(12),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_callout_block, 4, 0, 4),
  [145] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_callout_block, 4, 0, 4),
  [147] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [151] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [153] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_callout_block, 6, 0, 4),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_callout_block, 6, 0, 4),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_callout_block, 5, 0, 4),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_callout_block, 5, 0, 4),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_callout_block, 7, 0, 4),
  [173] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_callout_block, 7, 0, 4),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_heading, 2, 0, 1),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_heading, 2, 0, 1),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_item, 2, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_item, 2, 0, 0),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paragraph_line, 2, 0, 0),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_paragraph_line, 2, 0, 0),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_heading, 3, 0, 1),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_heading, 3, 0, 1),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_item, 3, 0, 2),
  [193] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_item, 3, 0, 2),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fenced_code_block, 5, 0, 5),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fenced_code_block, 5, 0, 5),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_item, 3, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_item, 3, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_item, 4, 0, 2),
  [205] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_item, 4, 0, 2),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fenced_code_block, 6, 0, 8),
  [209] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_fenced_code_block, 6, 0, 8),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_embed, 3, 0, 3),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_embed, 3, 0, 3),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_wikilink, 3, 0, 3),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_wikilink, 3, 0, 3),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_italic, 3, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_italic, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bold, 3, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bold, 3, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_inline_code, 3, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_inline_code, 3, 0, 0),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_wikilink, 5, 0, 6),
  [233] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_wikilink, 5, 0, 6),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_link, 5, 0, 7),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_link, 5, 0, 7),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_bold_repeat1, 2, 0, 0),
  [241] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_bold_repeat1, 2, 0, 0), SHIFT_REPEAT(45),
  [244] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_bold_repeat1, 2, 0, 0), SHIFT_REPEAT(43),
  [247] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_bold_repeat1, 2, 0, 0), SHIFT_REPEAT(77),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [254] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [260] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_italic_repeat1, 2, 0, 0), SHIFT_REPEAT(48),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_italic_repeat1, 2, 0, 0),
  [265] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_italic_repeat1, 2, 0, 0), SHIFT_REPEAT(78),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bold_text, 1, 0, 0),
  [278] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bold_text, 1, 0, 0),
  [280] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_italic_text, 1, 0, 0),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_wikilink_alias, 1, 0, 0),
  [300] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [306] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_embed_target, 1, 0, 0),
  [308] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [310] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [312] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [314] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [320] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [326] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [334] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [342] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_noted(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
