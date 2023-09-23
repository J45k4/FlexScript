use std::collections::HashMap;
use crate::Obj;
use crate::ObjProp;
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

    pub fn lookup(&mut self, ptr: &Ptr) -> Option<&mut Value> {
        if ptr.id > UNAMED_VAR_ID {
            return match self.scopes[ptr.scope_id as usize].vars.get_mut(&ptr.id) {
                Some(val) => Some(val),
                None => None
            };
        }

        let mut scope_id = ptr.scope_id;

        loop {
            let s = match self.scopes.get(scope_id as usize) {
                Some(s) => s,
                None => return None,
            };

            match s.vars.get(&ptr.id) {
                Some(_) => break,
                None => {}
            }

            match self.get_parent_scope(scope_id) {
                Some(parent_id) => scope_id = parent_id,
                None => return None
            }
        }

        self.scopes[scope_id as usize].vars.get_mut(&ptr.id)
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

    pub fn copy_value(&mut self, val: Value) -> Value  {
        match val {
            Value::Ptr(ptr) => {
                let scope = self.scopes.get(ptr.scope_id as usize).unwrap();
                self.copy_value(scope.vars.get(&ptr.id).unwrap().clone())
            },
            Value::Obj(obj) => {
                Value::Obj(
                    Obj {
                        name: obj.name,
                        props: obj.props.iter().map(|prop| {
                            ObjProp {
                                name: prop.name.clone(),
                                value: self.copy_value(prop.value.clone())
                            }
                        }).collect()
                    }
                )
            },
            Value::List(list) => {
                Value::List(list.iter().map(|item| self.copy_value(item.clone())).collect())
            },
            _ => val
        }
    }

    pub fn move_value(&mut self, val: Value, to_scope_id: u32) -> Value  {
        match val {
            Value::Ptr(ptr) => {
                let new_id = self.move_to(&ptr, to_scope_id);
                Value::Ptr(new_id)
            },
            Value::Obj(obj) => {
                Value::Obj(
                    Obj {
                        name: obj.name,
                        props: obj.props.iter().map(|prop| {
                            ObjProp {
                                name: prop.name.clone(),
                                value: self.move_value(prop.value.clone(), to_scope_id)
                            }
                        }).collect()
                    }
                )
            },
            _ => val
        }
    }

    pub fn move_to(&mut self, ptr: &Ptr, to_scope_id: u32) -> Ptr {
        let v = {
            let mut scope = self.scopes.get_mut(ptr.scope_id as usize).unwrap();
            let v = scope.vars.get(&ptr.id).unwrap().clone();
            scope.remove_var(&ptr.id);
            v
        };

        let v = self.move_value(v, to_scope_id);

        let scope2  = self.scopes.get_mut(to_scope_id as usize).unwrap();
        let new_id = scope2.store_unamed(v.clone());

        Ptr {
            id: new_id,
            scope_id: to_scope_id
        }
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
        let ptr = scope_manager.store_unamed(scope_id, Value::Int(10));
        let val = scope_manager.lookup(&ptr).unwrap();
        assert_eq!(Value::Int(10), *val);
    }

    #[test]
    fn lookup_value_from_parent_scope() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let child_scope_id = scope_manager.create_child_scope(scope_id);
        scope_manager.store_named(scope_id, 1, Value::Int(10));
        let val = scope_manager.lookup(&Ptr { id: 1, scope_id: child_scope_id }).unwrap();
        assert_eq!(Value::Int(10), *val);
    }


    #[test]
    fn move_value_to_parent() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let child_scope_id = scope_manager.create_child_scope(scope_id);

        let ptr = scope_manager.store_unamed(child_scope_id, Value::Int(10));
        let new_ptr = scope_manager.move_to(&ptr, scope_id);

        let val = scope_manager.lookup(&new_ptr).unwrap();
        assert_eq!(Value::Int(10), *val);
        assert_eq!(new_ptr.scope_id, scope_id);
    }

    #[test]
    fn move_obj_to_parent() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let child_scope_id = scope_manager.create_child_scope(scope_id);

        let ptr = scope_manager.store_unamed(child_scope_id, Value::Obj(
            Obj {
                name: None,
                props: vec![
                    ObjProp {
                        name: "a".to_string(),
                        value: Value::Int(10)
                    }
                ]
            }
        ));
        let new_ptr = scope_manager.move_to(&ptr, scope_id);

        let val = scope_manager.lookup(&new_ptr).unwrap();
        assert_eq!(Value::Obj(
            Obj {
                name: None,
                props: vec![
                    ObjProp {
                        name: "a".to_string(),
                        value: Value::Int(10)
                    }
                ]
            }
        ), *val);
        assert_eq!(new_ptr.scope_id, scope_id);
    }

    #[test]
    fn store_named_variable() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        scope_manager.store_named(scope_id, 1, Value::Int(10));
        let val = scope_manager.lookup(&Ptr { id: 1, scope_id: scope_id }).unwrap();
        assert_eq!(Value::Int(10), *val);
    }

    #[test]
    fn store_unnamed_variable() {
        let mut scope_manager = ScopeManager::new();
        let scope_id = scope_manager.create_scope();
        let ptr = scope_manager.store_unamed(scope_id, Value::Int(10));
        let val = scope_manager.lookup(&ptr).unwrap();
        assert_eq!(Value::Int(10), *val);
    }
} 