use std::collections::HashMap;
use crate::Value;

const UNAMED_VAR_ID: u32 = 2147483648;

#[derive(Debug)]
struct Scope {
    parent_id: Option<u32>,
    vars: HashMap<u32, Value>,
    unamed_var_id: u32
}

impl Scope {
    fn new(parent_id: Option<u32>) -> Self {
        Self {
            parent_id,
            vars: HashMap::new(),
            unamed_var_id: UNAMED_VAR_ID
        }
    }

    fn store_unamed(&mut self, val: Value) -> u32 {
        self.unamed_var_id += 1;
        self.vars.insert(self.unamed_var_id, val);
        self.unamed_var_id
    }
}

pub struct ScopeManager {
    scopes: Vec<Scope>   
}

impl ScopeManager {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
        }
    }

    pub fn store_named(&mut self, scope_id: u32, var_id: u32, val: Value) {
        let scope = self.scopes.get_mut(scope_id as usize).unwrap();
        scope.vars.insert(var_id, val);
    }

    pub fn store_unamed(&mut self, val: Value) -> u32 {
        let scope = self.scopes.last_mut().unwrap();
        scope.store_unamed(val)
    }

    pub fn lookup(&mut self, scope_id: u32, var_id: &u32) -> Option<&mut Value> {
        if var_id > &UNAMED_VAR_ID {
            return match self.scopes[scope_id as usize].vars.get_mut(var_id) {
                Some(val) => Some(val),
                None => None
            };
        }

        let mut current_scope = scope_id;

        loop {
            match self.scopes.get_mut(scope_id as usize) {
                Some(s) => {
                    match s.vars.get_mut(var_id) {
                        Some(val) => break Some(val),
                        None => break None
                    }
                },
                None => break None
            }
        }
    }

    pub fn create_scope(&mut self) -> u32 {
        self.scopes.push(Scope::new(None));
        (self.scopes.len() - 1) as u32
    }

    pub fn create_child_scope(&mut self, parent_id: u32) -> u32 {
        self.scopes.push(Scope::new(Some(parent_id)));
        (self.scopes.len() - 1) as u32
    }

    pub fn get_parent_scope(&self, scope_id: u32) -> Option<u32> {
        let scope = self.scopes.get(scope_id as usize)?;
        scope.parent_id
    }
}