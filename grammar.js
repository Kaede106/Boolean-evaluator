module.exports = grammar({
  name: "boolean_algebra",

  extras: ($) => [/\s/],

  rules: {
    source_file: ($) => $.expression,

    expression: ($) =>
      choice(
        $.variable,
        $.not_expression,
        $.binary_expression,
        $.parenthesized_expression,
      ),

    variable: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    not_expression: ($) =>
      prec(
        3,
        seq(
          field("operator", choice("!", "~", "\\lnot", "NOT")),
          field("argument", $.expression),
        ),
      ),

    binary_expression: ($) => {
      const table = [
        { op: choice("&", "\\land", "AND", "and"), prec: 2 },
        { op: choice("|", "\\lor", "OR", "or"), prec: 1 },
      ];

      return choice(
        ...table.map(({ op, prec: p }) =>
          prec.left(
            p,
            seq(
              field("left", $.expression),
              field("operator", op),
              field("right", $.expression),
            ),
          ),
        ),
      );
    },

    parenthesized_expression: ($) =>
      seq("(", field("expression", $.expression), ")"),
  },
});
