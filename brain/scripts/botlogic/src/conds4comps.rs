//! This set of functions helps us get the result
//!  from a list of conditionals based on comparisons
//!  that is received as a String
use rustlogic;
use std::collections::HashMap;


/// This struct stores the String with the conditionals
///  and the dictionary storing the values for the variables in the conditionals.
#[derive(Clone, Debug)]
struct LogicDict {
    formula: String,
    varset: HashMap<String, bool>,
}

/// This is the main Function called from the outside.  
/// It receives a String of conditions and a Hashmap of the variables and their values,  
///  e.g.: "(temperature>20)&speed=7" ["temperature": 14, "speed": 7]   
/// Returns the boolean result we'd get 
///  if the conditions came as code instead of a String.  
///    
pub fn get_result(v: &str, vars_raw: &HashMap<&str, &str>) -> bool {
    let mut vars = HashMap::<String, String>::new();
    for (key, value) in &*vars_raw {
        vars.insert(key.to_string(), value.to_string());
        /*
        // TODO: modify this to not depend on it being a number ...move parse to another function?
        match value.parse::<f32>() {
            Ok(vi) => { 
                vars.insert(key.to_string(), vi as i32);
            }
            Err(_) => { 
                match value.parse::<i32>() {
                    Ok(vi) => { 
                        vars.insert(key.to_string(), vi);
                    }
                    Err(_) => { }
                };

            }
        };
        */
    }
    let comps = get_comps_done(v, vars);
    let logicvars = parse_to_rustlogic(&comps); 


    // TODO: prepare for when there is not formula
    let formula =match rustlogic::parse(&logicvars.formula) {
            Ok(p) => { p }
            Err(e) => { panic!("ERROR PARSING! {:?}", e) }
        };
    
    let mut variable_map = HashMap::new();

    let mut aux: &str;
    for k in logicvars.varset.keys() {
        aux = &k;
        variable_map.insert(aux, logicvars.varset[k]);
    }

    formula.get_value_from_variables(&variable_map).unwrap()
}

/// Gets a String of conditions and a Dict of Variables and their values  
///  e.g.: "(temperature>20)&speed=7" ["temperature": 14, "speed": 7]   
/// Returns another String,
///   where the comparisons inside the list of conditions   
///   have been translated to their boolean results  
///  e.g.: "(false)&true"
fn get_comps_done(s: &str, vars: HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut r = s.clone().to_string();
    // need this to trigger one last run of do_comps
    // TODO: find a better way
    r.push('#');
    let mut expr: String = String::new();
    let mut prev_char: String = String::new();
    let cleaned_r = r
        .replace("&&", "&")
        .replace("||", "|");
    for c in cleaned_r.chars() {
        match c {
            '(' | ')' | ' ' | '&' | '|' |  '#' => {
                if prev_char == "var2" {
                    //for ch in do_comps(&expr, vars.clone()).to_string().chars() {
                    for ch in do_compare(&expr, vars.clone()).to_string().chars() {
                        result.push(ch);
                    }
                    expr.clear();
                }
                if c != '#' { result.push(c); }
                prev_char = "limit".to_string();
            }
            '!' |'=' | '>' | '<' => {
                expr.push(c);
                prev_char = "comp".to_string();
            }
            _ => {
                expr.push(c);
                match prev_char.as_str() {
                    "comp" | "var2" => { prev_char = "var2".to_string(); }
                    _ => { prev_char = "var".to_string(); }
                }
            }
        }
    }
    result
}

/// Receives a &str containing -JUST- a comparison, 
///  as well as a Hashmap with the value for those variables involved.
///  e.g.: "temperature>20" ["temperature": 14]   
///
/// Then returns the boolean result of that comparison (e.g.: false)  
fn do_compare(s: &str, vars: HashMap<String, String>) -> bool {
    let mut prev_char: String = String::new();
    let mut var1: String = String::new();
    let mut oper: String = String::new();
    let mut var2: String = String::new();
    for c in s.chars() {
        match c {
            // 
            '=' | '>' | '<' => {
                if prev_char != "var1" && prev_char != "oper" { panic!("Wrong syntax at {}", s) }
                oper.push(c);
                prev_char = "oper".to_string()
            }
            // anything else should be part of a variable name
            _ => {
                match prev_char.as_str() {
                    "var2" => { var2.push(c); }
                    _ => { 
                        if prev_char == "oper" {
                            prev_char = "var2".to_string();
                            var2.push(c); 
                        } else {
                            prev_char = "var1".to_string();
                            var1.push(c); 
                        }
                    }
                }
            }
        }
    }
    // TODO: check contents of vars and act accordingly
    //   look for values in vars -> if not found, use varname as value
    //   both usize/int/float? -> turn both to float and use oper as usual
    //   one/both are String? -> treat them as strings
    //      - only == and != will be tested
    //      - the rest will be taken as FALSE with a message that comparison is wrong

    // Check if we are talking about a known variable
    // If the virst variable is not among ours, we throw a message and return false
    let val1_st = match vars.get(&var1) {
        Some(v) => { v.to_string() }
        None => { 
            // Control logging maybe and add this as a WARN
            println!("WARNING! {} is not defined (yet?). Returning TRUE...", var1);
            return true
        }
    };
    let val2_st = match vars.get(&var2) {
        Some(v) => { v.to_string() }
        None => { var2 }
    };
    // Check if those variables are numbers
    let val1_fl = match val1_st.parse::<f32>() {
        Ok(vi) => { Some(vi) }
        Err(_) => { None }
    };
    let val2_fl = match val2_st.parse::<f32>() {
        Ok(vi) => { Some(vi) }
        Err(_) => { None }
    };
    //   both usize/int/float? -> use oper as usual with their floats
    if ! val1_fl.is_none() && ! val2_fl.is_none() {
        match oper.as_str() {
            "=" | "==" => { return val1_fl == val2_fl; }
            ">" => { return val1_fl > val2_fl; }
            "<" => { return val1_fl < val2_fl; }
            ">=" => { return val1_fl >= val2_fl; }
            "<=" => { return val1_fl <= val2_fl; }
            "!=" => { return val1_fl != val2_fl; }
            _  => { return false; }
        }
    //   one/both are String? -> treat them as strings
    //      - only == and != will be tested
    //      - the rest will be taken as FALSE with a message that comparison is wrong
    } else {
        match oper.as_str() {
            "=" | "==" => { return val1_st.eq(&val2_st); }
            "!=" => { return val1_st.ne(&val2_st); }
            _  => { 
                // Control logging maybe and add this as a WARN
                println!("WARNING! wrong comparison on {} {} {}, Returning FALSE...", val1_st, oper, val2_st);
                return false; }
        }
    }
}

/// Creates a [LogicDict] from something that looks like:  
/// "(true&true)|false&true"  
/// to something like  
/// "(var1&var2)|var3&var4" ["var1": true, "var2": true ...]  
/// , which is what rustlogic will use
fn parse_to_rustlogic(s: &str) -> LogicDict {
    let mut vars: HashMap<String, bool> = HashMap::new();
    let mut tbd = true;
    let mut var_ix = 0;
    //let mut formula: String = String::new();
    let mut formula = s.to_string();
    while tbd == true {
        let form_aux: String;
        if formula.contains("false") {
            var_ix +=1;
            form_aux = formula.clone().replacen("false", &format!("[var{}]", var_ix), 1);
            vars.insert(format!("var{}", var_ix), false);
            formula = form_aux;
        } else if formula.contains("true") {
            var_ix +=1;
            form_aux = formula.clone().replacen("true", &format!("[var{}]", var_ix), 1);
            vars.insert(format!("var{}", var_ix), true);
            formula = form_aux;
        } else {
            tbd = false;
        }
    }
    let result = LogicDict {
        formula: formula.to_string(),
        varset: vars,
    };
    result.clone()
}
