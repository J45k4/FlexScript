use std::collections::HashMap;
use crate::Value;

#[derive(Debug)]
struct Scope {
    parent_id: Option<usize>,
    vars: HashMap<usize, Value>
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

    pub fn insert(&mut self, scope_id: usize, var_id: usize, val: Value) {
        let scope = self.scopes.get_mut(scope_id).unwrap();
        scope.vars.insert(var_id, val);
    }

    pub fn lookup(&self, scope_id: usize, var_id: &usize) -> Option<&Value> {
        let mut current_scope = scope_id;

        loop {
            let scope = self.scopes.get(current_scope)?;

            if let Some(val) = scope.vars.get(var_id) {
                return Some(val);
            }

            match scope.parent_id {
                Some(parent_id) => current_scope = parent_id,
                None => return None
            }
        }
    }

    pub fn create_scope(&mut self) -> usize {
        self.scopes.push(
            Scope { 
                parent_id: None, 
                vars: HashMap::new()
            }
        );

        self.scopes.len() - 1
    }

    pub fn create_child_scope(&mut self, parent_id: usize) -> usize {
        self.scopes.push(
            Scope { 
                parent_id: Some(parent_id), 
                vars: HashMap::new()
            }
        );

        self.scopes.len() - 1
    }

    pub fn get_parent_scope(&self, scope_id: usize) -> Option<usize> {
        let scope = self.scopes.get(scope_id)?;
        scope.parent_id
    }
}