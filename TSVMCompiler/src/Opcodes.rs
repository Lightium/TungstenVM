#[derive(Clone, RustcEncodable)]
pub enum Opcodes {
    None, // none
    Var, // var Name Type = Value
    Set, // set Name Type = NewValue
    Print, // print Variable
    Equals, // eq Variable Variable
    LessThan, // lss Variable Variable
    GreaterThan, // gtr Variable Variable
    Or, // or Variable Variable
    And, // and Variable Variable
    Res, // res Variable
    Jump, // jmp PosName
    JumpIfResTrue, // jt PosName
    Call, // call Function
    CallExt, // callext VariableOutput DLLName Function
    Pos // pos PosName
}