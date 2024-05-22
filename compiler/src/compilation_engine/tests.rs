use pretty_assertions::assert_eq;

use super::*;

#[test]
fn array_test() {
    let input =
        include_str!("/home/alex/Projects/Assembly/nand2tetris/projects/10/ArrayTest/Main.jack");

    let class =
        include_str!("/home/alex/Projects/Assembly/nand2tetris/projects/10/ArrayTest/Main.xml");

    let out = compile(input, "compile");

    assert_eq!(out, class)
}

#[test]
fn expression_less_square() {
    let input = include_str!("../../../projects/10/ExpressionLessSquare/Main.jack");
    let out = include_str!("../../../projects/10/ExpressionLessSquare/Main.xml");

    assert_eq!(compile(input, "compile"), out);

    let input = include_str!("../../../projects/10/ExpressionLessSquare/Square.jack");
    let out = include_str!("../../../projects/10/ExpressionLessSquare/Square.xml");

    assert_eq!(compile(input, "compile"), out);

    let input = include_str!("../../../projects/10/ExpressionLessSquare/SquareGame.jack");
    let out = include_str!("../../../projects/10/ExpressionLessSquare/SquareGame.xml");

    assert_eq!(compile(input, "compile"), out);
}

#[test]
fn square() {
    let input = include_str!("../../../projects/10/Square/Main.jack");
    let out = include_str!("../../../projects/10/Square/Main.xml");

    assert_eq!(compile(input, "compile"), out);

    let input = include_str!("../../../projects/10/Square/Square.jack");
    let out = include_str!("../../../projects/10/Square/Square.xml");

    assert_eq!(compile(input, "compile"), out);

    let input = include_str!("../../../projects/10/Square/SquareGame.jack");
    let out = include_str!("../../../projects/10/Square/SquareGame.xml");

    assert_eq!(compile(input, "compile"), out);
}

#[test]
fn test_term_1() {
    let out = compile(r#"Keyboard.readInt("ENTER THE NEXT NUMBER: ")"#, "term");

    assert_eq!(
        out,
        "\
<term>
  <identifier> Keyboard </identifier>
  <symbol> . </symbol>
  <identifier> readInt </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <stringConstant> ENTER THE NEXT NUMBER:  </stringConstant>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
</term>
"
    );
}

#[test]
fn test_term_2() {
    let out = compile("a[i]", "term");

    assert_eq!(
        out,
        "\
<term>
  <identifier> a </identifier>
  <symbol> [ </symbol>
  <expression>
    <term>
      <identifier> i </identifier>
    </term>
  </expression>
  <symbol> ] </symbol>
</term>
"
    )
}

#[test]
fn test_return_1() {
    let out = compile("return;", "return");

    assert_eq!(
        out,
        "\
<returnStatement>
  <keyword> return </keyword>
  <symbol> ; </symbol>
</returnStatement>
"
    )
}

#[test]
fn test_return_2() {
    let out = compile("return this;", "return");

    assert_eq!(
        out,
        "\
<returnStatement>
  <keyword> return </keyword>
  <expression>
    <term>
      <keyword> this </keyword>
    </term>
  </expression>
  <symbol> ; </symbol>
</returnStatement>
"
    )
}

#[test]
fn expression_1() {
    let out = compile("x - 2", "expression");
    assert_eq!(
        out,
        "\
<expression>
  <term>
    <identifier> x </identifier>
  </term>
  <symbol> - </symbol>
  <term>
    <integerConstant> 2 </integerConstant>
  </term>
</expression>
"
    )
}

#[test]
fn expression_2() {
    let out = compile("(y + size) < 254", "expression");

    assert_eq!(
        out,
        "\
<expression>
  <term>
    <symbol> ( </symbol>
    <expression>
      <term>
        <identifier> y </identifier>
      </term>
      <symbol> + </symbol>
      <term>
        <identifier> size </identifier>
      </term>
    </expression>
    <symbol> ) </symbol>
  </term>
  <symbol> &lt; </symbol>
  <term>
    <integerConstant> 254 </integerConstant>
  </term>
</expression>
"
    )
}

#[test]
fn compile_var_1() {
    let out = compile("var Array a;", "var_dec");

    assert_eq!(
        out,
        "\
<varDec>
  <keyword> var </keyword>
  <identifier> Array </identifier>
  <identifier> a </identifier>
  <symbol> ; </symbol>
</varDec>
"
    );
}

#[test]
fn compile_var_2() {
    let out = compile("var int i, sum;", "var_dec");

    assert_eq!(
        out,
        "\
<varDec>
  <keyword> var </keyword>
  <keyword> int </keyword>
  <identifier> i </identifier>
  <symbol> , </symbol>
  <identifier> sum </identifier>
  <symbol> ; </symbol>
</varDec>
"
    );
}

#[test]
fn parameter_list_1() {
    let out = compile("int Ax, int Ay, int Asize", "parameter_list");

    assert_eq!(
        out,
        "\
<parameterList>
  <keyword> int </keyword>
  <identifier> Ax </identifier>
  <symbol> , </symbol>
  <keyword> int </keyword>
  <identifier> Ay </identifier>
  <symbol> , </symbol>
  <keyword> int </keyword>
  <identifier> Asize </identifier>
</parameterList>
"
    );
}

fn compile(input: &str, routine: &str) -> String {
    let mut buf = vec![];

    let mut compiler = CompilationEngine::new(input, &mut buf);

    match routine {
        "compile" => compiler.compile(),
        "term" => compiler.compile_term(),
        "return" => compiler.compile_return(),
        "var_dec" => compiler.compile_var_dec(),
        "expression" => compiler.compile_expression(),
        "parameter_list" => compiler.compile_parameter_list(),
        r => panic!("unknown routine: {r}"),
    }
    .unwrap();

    String::from_utf8(buf).unwrap()
}
