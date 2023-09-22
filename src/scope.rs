use std::collections::HashMap;
use crate::Ptr;
use crate::Value;

const UNAMED_VAR_ID: u32 = 2147483648;

#[derive(Debug)]
struct Scope {
    parent_id: Option<u32>,
    vars: HashMap<u32, Value>,
    unamed_var_id: u32,
    unused_unamed_ids: Vec<u32>
}

impl Scope {
    fn new(parent_id: Option<u32>) -> Self {
        Self {
            parent_id,
            vars: HashMap::new(),
            unamed_var_id: UNAMED_VAR_ID,
            unused_unamed_ids: vec![]
        }
    }

    fn store_unamed(&mut self, val: Value) -> u32 {
        self.unamed_var_id += 1;
        self.vars.insert(self.unamed_var_id, val);
        self.unamed_var_id
    }

    fn remove_var(&mut self, var_id: &u32) {
        if self.vars.remove(var_id).is_some() {
            if var_id > &UNAMED_VAR_ID {
                self.unused_unamed_ids.push(*var_id);
            }
        }
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

    pub fn store_unamed(&mut self, scope_id: u32, val: Value) -> Ptr {
        let scope = self.scopes.get_mut(scope_id as usize).unwrap();
        let id = scope.store_unamed(val);
        Ptr {
            id,
            scope_id
        }
    }

    pub fn remove_var(&mut self, scope_id: u32, var_id: &u32) {
        let scope = self.scopes.get_mut(scope_id as usize).unwrap();
        scope.remove_var(var_id);
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

    pub fn delete_scope(&mut self, scope_id: u32) {
        self.scopes.remove(scope_id as usize);
    }

    pub fn create_child_scope(&mut self, parent_id: u32) -> u32 {
        self.scopes.push(Scope::new(Some(parent_id)));
        (self.scopes.len() - 1) as u32
    }

    pub fn get_parent_scope(&self, scope_id: u32) -> Option<u32> {
        let scope = self.scopes.get(scope_id as usize)?;
        scope.parent_id
    }

    pub fn has_scope(&self, scope_id: u32) -> bool {
        self.scopes.get(scope_id as usize).is_some()
    }

    pub fn move_to(&mut self, var_id: &u32, from_scope_id: u32, to_scope_id: u32) -> Ptr {
        let val = self.lookup(from_scope_id, var_id).unwrap().clone();
        let new_id = self.store_unamed(to_scope_id, val);
        self.remove_var(from_scope_id, var_id);
        new_id
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_scope_and_remove_scope() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        assert_eq!(0, scope_id);
        assert!(scope_manager.has_scope(scope_id));
        scope_manager.delete_scope(scope_id);
        assert!(!scope_manager.has_scope(scope_id));
    }

    #[test]
    fn lookup_value() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let var_id = scope_manager.store_unamed(scope_id, Value::Int(10));
        let val = scope_manager.lookup(scope_id, &var_id).unwrap();
        assert_eq!(Value::Int(10), *val);
    }

    #[test]
    fn lookup_value_from_parent_scope() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let child_scope_id = scope_manager.create_child_scope(scope_id);
        scope_manager.store_named(scope_id, 1, Value::Int(10));
        let val = scope_manager.lookup(child_scope_id, &1).unwrap();
        assert_eq!(Value::Int(10), *val);
    }


    // #[test]
    // fn move_value_to_parent() {
    //     let mut scope_manager = ScopeManager::new();
    //     let scope_id = scope_manager.create_scope();
    //     let child_scope_id = scope_manager.create_child_scope(scope_id);

    //     let var_id = scope_manager.store_unamed(child_scope_id, Value::Int(10));
    //     let new_id = scope_manager.move_to_parent(child_scope_id, &var_id);

    //     let val = scope_manager.lookup(scope_id, &new_id).unwrap();
    //     assert_eq!(Value::Int(10), *val);
    // }

    #[test]
    fn store_named_variable() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        scope_manager.store_named(scope_id, 1, Value::Int(10));
        let val = scope_manager.lookup(scope_id, &1).unwrap();
        assert_eq!(Value::Int(10), *val);
    }

    #[test]
    fn store_unnamed_variable() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let var_id = scope_manager.store_unamed(scope_id, Value::Int(10));
        let val = scope_manager.lookup(scope_id, &var_id).unwrap();
        assert_eq!(Value::Int(10), *val);
    }

    // #[test]
    // fn move_value_with_references() {
    //     let mut scope_manager = ScopeManager::new();
    //     let scope_id = scope_manager.create_scope();
    //     let child_scope_id = scope_manager.create_child_scope(scope_id);

    //     let item1_id = scope_manager.store_unamed(child_scope_id, Value::Int(10));
    //     let item2_id = scope_manager.store_unamed(child_scope_id, Value::Int(20));
    //     let item3_id = scope_manager.store_unamed(scope_id, Value::Int(30));

    //     let item1 = Value::Ptr(item1_id);
    //     let item2 = Value::Ptr(item2_id);
    //     let item3 = Value::Ptr(item3_id);

    //     let value = Value::List(
    //         vec![]
    //     )

    //     let var_id = scope_manager.store_unamed(child_scope_id, Value::Int(10));
    //     let new_id = scope_manager.move_to_parent(child_scope_id, &var_id);

    //     let val = scope_manager.lookup(scope_id, &new_id).unwrap();
    //     assert_eq!(Value::Int(10), *val);

    //     let val = scope_manager.lookup(child_scope_id, &var_id).unwrap();
    //     assert_eq!(Value::Null, *val);
    // }
} 