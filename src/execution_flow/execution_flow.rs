use std::marker::PhantomData;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

pub type FunctionLabel = u64;
pub type LoopLabel = usize;

pub const MAIN_FUNCTION_LABEL: u64 = u64::MAX;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Scope<
    Instructions: Clone + Length,
    FunctionCall: GetFunctionLabel + Clone,
    FunctionReturn: GetFunctionLabel + Clone,
> {
    Instructions(Instructions),
    FunctionCall(FunctionCall),
    FunctionReturn(FunctionReturn),
    Loop {
        label: LoopLabel,
    },
    LoopContinue {
        label: LoopLabel,
    },
    Condition {
        instructions_if_true: Vec<Self>,
        instructions_if_false: Vec<Self>,
    },
    Panic,
    Empty,
}

impl<
        Instructions: Clone + Length,
        FunctionCall: GetFunctionLabel + Clone,
        FunctionReturn: GetFunctionLabel + Clone,
    > Scope<Instructions, FunctionCall, FunctionReturn>
{
    pub fn is_loop(&self) -> bool {
        if let Scope::Loop { label: _ } = self {
            return true;
        }
        return false;
    }

    pub fn is_loop_continue(&self) -> bool {
        if let Scope::LoopContinue { label: _ } = self {
            return true;
        }
        return false;
    }

    pub fn is_function_return(&self) -> bool {
        if let Scope::FunctionReturn(_) = self {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionFlow<
    Instructions: Clone + Length,
    FunctionCall: Clone + GetFunctionLabel,
    FunctionReturn: Clone + GetFunctionLabel,
    Function: Clone + GetFunctionLabel + AccessContent<Instructions, FunctionCall, FunctionReturn>,
> {
    pub functions: HashMap<FunctionLabel, Function>,
    _p: (
        PhantomData<Instructions>,
        PhantomData<FunctionCall>,
        PhantomData<FunctionReturn>,
    ),
}

pub trait GetFunctionLabel {
    fn get_label(&self) -> FunctionLabel;

    fn is_main(&self) -> bool {
        return self.get_label() == MAIN_FUNCTION_LABEL;
    }
}

pub trait Length {
    fn len(&self) -> usize;
}

pub trait AccessContent<
    Instructions: Clone + Length,
    FunctionCall: GetFunctionLabel + Clone,
    FunctionReturn: GetFunctionLabel + Clone,
>
{
    fn get_content(&self) -> &Vec<Scope<Instructions, FunctionCall, FunctionReturn>>;

    fn get_content_mut(&mut self) -> &mut Vec<Scope<Instructions, FunctionCall, FunctionReturn>>;
}

impl<
        Instructions: Clone + Length,
        FunctionCall: Clone + GetFunctionLabel,
        FunctionReturn: Debug + GetFunctionLabel + Clone,
        Function: Clone + GetFunctionLabel + AccessContent<Instructions, FunctionCall, FunctionReturn>,
    > ExecutionFlow<Instructions, FunctionCall, FunctionReturn, Function>
{
    pub fn new(functions: HashMap<FunctionLabel, Function>) -> Self {
        return Self {
            functions,
            _p: (PhantomData, PhantomData, PhantomData),
        };
    }
    pub fn get_secondary_functions(&self) -> impl Iterator<Item = &Function> {
        return self
            .functions
            .values()
            .filter(|function| function.get_label() != MAIN_FUNCTION_LABEL);
    }

    pub fn get_main_function(&self) -> &Function {
        return &self.functions[&MAIN_FUNCTION_LABEL];
    }
    pub fn count_function_uses(&self) -> HashMap<FunctionLabel, usize> {
        let mut function_uses: HashMap<FunctionLabel, usize> = HashMap::new();
        for label in self.functions.keys() {
            function_uses.insert(*label, 0);
        }
        for (_, function) in &self.functions {
            Self::count_function_uses_in_scopes(&function.get_content(), &mut function_uses);
        }
        return function_uses;
    }

    fn count_function_uses_in_scopes(
        scopes: &Vec<Scope<Instructions, FunctionCall, FunctionReturn>>,
        function_uses: &mut HashMap<FunctionLabel, usize>,
    ) {
        for scope in scopes {
            match scope {
                Scope::FunctionCall(function_call) => {
                    *function_uses.get_mut(&function_call.get_label()).unwrap() += 1;
                }
                Scope::Condition {
                    instructions_if_true,
                    instructions_if_false,
                } => {
                    Self::count_function_uses_in_scopes(instructions_if_true, function_uses);
                    Self::count_function_uses_in_scopes(instructions_if_false, function_uses);
                }
                _ => (),
            }
        }
    }

    pub fn compute_size_of_scopes(
        scopes: &Vec<Scope<Instructions, FunctionCall, FunctionReturn>>,
    ) -> usize {
        let mut size = 0;
        for scope in scopes {
            match scope {
                Scope::Instructions(instructions_with_opcodes) => {
                    size += instructions_with_opcodes.len()
                }
                Scope::FunctionCall(_) => size += 1,
                Scope::FunctionReturn(_) => size += 1,
                Scope::Loop { label: _ } => size += 1,
                Scope::LoopContinue { label: _ } => size += 1,
                Scope::Condition {
                    instructions_if_true,
                    instructions_if_false,
                } => {
                    size += Self::compute_size_of_scopes(instructions_if_true)
                        + Self::compute_size_of_scopes(instructions_if_false)
                }
                Scope::Panic => size += 1,
                Scope::Empty => (),
            }
        }
        return size;
    }

    pub fn apply_on_all_scopes(
        &self,
        key: &mut impl FnMut(&Scope<Instructions, FunctionCall, FunctionReturn>),
    ) {
        for (_, function) in &self.functions {
            Self::apply_on_scopes(function.get_content(), key);
        }
    }

    pub fn apply_on_scopes(
        scopes: &Vec<Scope<Instructions, FunctionCall, FunctionReturn>>,
        key: &mut impl FnMut(&Scope<Instructions, FunctionCall, FunctionReturn>),
    ) {
        for scope in scopes {
            key(scope);
            if let Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } = scope
            {
                Self::apply_on_scopes(instructions_if_true, key);
                Self::apply_on_scopes(instructions_if_false, key);
            }
        }
    }

    pub fn apply_on_all_scopes_mut(
        &mut self,
        key: &mut impl FnMut(&Scope<Instructions, FunctionCall, FunctionReturn>),
    ) {
        for (_, function) in &mut self.functions {
            Self::apply_on_scopes_mut(function.get_content_mut(), key);
        }
    }

    pub fn apply_on_scopes_mut(
        scopes: &mut Vec<Scope<Instructions, FunctionCall, FunctionReturn>>,
        key: &mut impl FnMut(&Scope<Instructions, FunctionCall, FunctionReturn>),
    ) {
        for scope in scopes {
            key(scope);
            if let Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } = scope
            {
                Self::apply_on_scopes_mut(instructions_if_true, key);
                Self::apply_on_scopes_mut(instructions_if_false, key);
            }
        }
    }

    pub fn any_scopes(
        // is there a scope for which 'key' is true ?
        scopes: &Vec<Scope<Instructions, FunctionCall, FunctionReturn>>,
        key: &mut impl Fn(&Scope<Instructions, FunctionCall, FunctionReturn>) -> bool,
    ) -> bool {
        let mut res: bool = false;
        Self::apply_on_scopes(scopes, &mut |scope| {
            res = res || key(scope);
        });
        return res;
    }
    pub fn remove_functions(
        &mut self,
        should_function_be_kept: impl Fn(
            &Function,
            usize, // n uses
        ) -> bool,
        replace_function_call_by_content_in_scopes: impl Fn(
            &mut Vec<Scope<Instructions, FunctionCall, FunctionReturn>>, // scopes
            FunctionLabel,                                               // label to replace
            &Function,                                                   //function content
        ),
    ) {
        let n_uses_per_function: HashMap<FunctionLabel, usize> = self.count_function_uses();
        let mut labels_to_replace: HashSet<FunctionLabel> = HashSet::new();
        for (label, n_uses) in n_uses_per_function {
            if !should_function_be_kept(&self.functions[&label], n_uses) {
                labels_to_replace.insert(label);
            }
        }

        for label_to_replace in &labels_to_replace {
            let function_content: Function = self.functions[label_to_replace].clone();
            //remove_function_returns_in_scopes(&mut replace_with, *label_to_replace);

            for (_, function) in &mut self.functions {
                replace_function_call_by_content_in_scopes(
                    function.get_content_mut(),
                    *label_to_replace,
                    &function_content,
                );
            }
        }
        for label in labels_to_replace {
            self.functions.remove(&label);
        }
    }

    pub fn remove_scopes_by_key(
        scopes: &mut Vec<Scope<Instructions, FunctionCall, FunctionReturn>>,
        to_remove: &impl Fn(&Scope<Instructions, FunctionCall, FunctionReturn>) -> bool,
    ) {
        scopes.retain(|scope| !to_remove(scope));
        for scope in scopes {
            if let Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } = scope
            {
                Self::remove_scopes_by_key(instructions_if_true, to_remove);
                Self::remove_scopes_by_key(instructions_if_false, to_remove);
            }
        }
    }
}
