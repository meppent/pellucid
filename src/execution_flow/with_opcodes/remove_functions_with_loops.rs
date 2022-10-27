use crate::execution_flow::execution_flow::{
    AccessContent, FunctionLabel, GetFunctionLabel, Scope,
};

use super::{
    flow_with_opcodes::ExecutionFlowWithOpcodes,
    scopes_with_opcodes::{FunctionWithOpcodes, OpcodeScope},
};

fn replace_function_call_by_content_in_scopes(
    scopes: &mut Vec<OpcodeScope>,
    label_to_replace: FunctionLabel,
    function_content: &FunctionWithOpcodes,
) {
    for scope_index in (0..scopes.len()).rev() {
        match &mut scopes[scope_index] {
            Scope::FunctionCall(function_call) => {
                if label_to_replace == function_call.get_label() {
                    let mut converted_content: Vec<OpcodeScope> =
                        function_content.get_content().clone();

                    ExecutionFlowWithOpcodes::remove_scopes_by_key(
                        &mut converted_content,
                        &|scope: &OpcodeScope| scope.is_function_return(),
                    );

                    // TODO try: scopes.splice(scope_index+1..scope_index, function_content.clone());
                    scopes.remove(scope_index);
                    scopes.splice(scope_index..scope_index, converted_content);
                }
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                replace_function_call_by_content_in_scopes(
                    instructions_if_true,
                    label_to_replace,
                    function_content,
                );
                replace_function_call_by_content_in_scopes(
                    instructions_if_false,
                    label_to_replace,
                    function_content,
                );
            }
            _ => (),
        }
    }
}

impl ExecutionFlowWithOpcodes {
    pub fn remove_secondary_functions_containing_loops(&mut self) {
        self.remove_functions(
            |function: &FunctionWithOpcodes, _: usize| -> bool {
                function.is_main()
                    || !ExecutionFlowWithOpcodes::any_scopes(
                        function.get_content(),
                        &mut |scope: &OpcodeScope| scope.is_loop() || scope.is_loop_continue(),
                    )
            },
            replace_function_call_by_content_in_scopes,
        );
    }
}
